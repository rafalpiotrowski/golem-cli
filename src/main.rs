use std::env;
use dotenv::dotenv;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> web3::Result<()> {
    println!("Hello Golem!");

    let path = std::env::current_dir().unwrap();
    println!("working dir: '{}'", path.display());

    let mut _network_url: String = "127.0.0.1:8545".to_string();
    
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
            Arg::with_name("buyers")
                .short("b")
                .long("buyers")
                .value_name("1")
                .help("Sets a number of concurrent buyers asking for GOL")
                .takes_value(true),
        ).get_matches();

        match matches.value_of("network").unwrap() {
            "local" => _network_url = "127.0.0.1:8545".to_string(),
            "ganache" => _network_url = "127.0.0.1:8545".to_string(),
            "rinkeby" => {
                dotenv().ok();
                _network_url = {
                    let project_id = env::var("INFURA_PROJECT_ID").expect("INFURA_PROJECT_ID is not set");
                    format!("https://rinkeby.infura.io/v3/{}", project_id)
                };
            },
            //_ => unreachable!("see possible parameters with golem-cli --help"),
            _ => (),
        };

    println!("using URL: {}", _network_url);

    

    Ok(())
}
