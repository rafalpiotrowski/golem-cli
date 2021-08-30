use std::sync::Arc;
use std::time::Duration;
use std::env;
use std::str::FromStr;
use dotenv::dotenv;
use clap::{App, Arg};

pub mod golem;

#[tokio::main]
async fn main() -> web3::Result<()> {
    println!("Hello Golem!");

    dotenv().ok();

    let path = std::env::current_dir().unwrap();
    println!("working dir: '{}'", path.display());

    let mut network_url: String = "127.0.0.1:8545".to_string();
    let mut balance_refresh_period = Duration::from_secs(5);
    
    let matches = App::new("golem-cli")
        .version("0.1")
        .author("Rafal Piotrowski")
        .about("interaction with Golem contract/token GOL")
        .arg(
            Arg::with_name("network")
                .short("n")
                .long("network")
                .value_name("rinkeby or local or ganache")
                .help("Sets a name of network to use")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("balance")
                .short("b")
                .long("balance")
                .value_name("1")
                .help("if present program will monitor balance on accounts every x seconds")
                .takes_value(true),
        ).get_matches();

    match matches.value_of("network").unwrap() {
        "local" => network_url = "127.0.0.1:8545".to_string(),
        "ganache" => network_url = "127.0.0.1:8545".to_string(),
        "rinkeby" => {
            network_url = {
                let project_id = env::var("INFURA_PROJECT_ID").expect("INFURA_PROJECT_ID is not set");
                format!("https://rinkeby.infura.io/v3/{}", project_id)
            };
        },
        _ => unreachable!("see possible parameters with golem-cli --help"),
    };

    println!("using URL: {}", network_url);

    match matches.value_of("balance").unwrap() {
        x => balance_refresh_period = Duration::from_secs(u64::from_str(x).unwrap())
    };

    let mut golem = golem::GolemToken::new(&network_url);
    golem.initialize_accounts().await?;

    let golem_for_tasks = Arc::new(golem).clone();

    //display accounts balances
    let h = tokio::spawn(async move {
        golem_for_tasks.print_balances().await.unwrap();
    });

    let _t1 = match h.await {
        Ok(_) => println!("balance completed"),
        Err(e) => println!("balance failed: {:?}", e)
    };

    Ok(())
}
