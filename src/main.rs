use std::{path::Path};
use std::sync::Arc;
use std::time::Duration;

use clap::{App, Arg};
// use log::{error, info, LevelFilter};
use log::{LevelFilter};

use near_crypto::*;
use neard::get_default_home;
use testlib::user::{rpc_user::RpcUser, User};

const DEFAULT_WAIT_PERIOD_MILLISEC: &str = "10000";
// const DEFAULT_RPC_URL: &str = "http://localhost:3030";
// const DEFAULT_RPC_URL: &str = "https://rpc.mainnet.near.org";
const DEFAULT_RPC_URL: &str = "rpc.testnet.near.org";
const DEFAULT_AGENT_KEYFILE: &str = "agent_key.json";
const DEFAULT_AGENT_ACCOUNT_ID: &str = "agent_crond";

/// Generate a validator key and save it to the file path.
fn generate_validator_key(account_id: &str, path: &Path) {
    // let signer = InMemoryValidatorSigner::from_random(account_id.to_string(), KeyType::ED25519);
    let signer = InMemorySigner::from_seed(account_id, KeyType::ED25519, account_id);
    // signer.public_key
    println!("CROND: Using key {} for {}.", signer.public_key, account_id);
    signer.write_to_file(path);
}

fn main() {
    env_logger::Builder::new().filter(None, LevelFilter::Info).init();
    let default_home = get_default_home();
    let matches = App::new("Key-pairs generator")
        .about(
            "Continuously checking on chain cron tasks, executing transactions to earn rewards",
        )
        .arg(
            Arg::with_name("home")
                .long("home")
                .default_value(&default_home)
                .help("Directory for config and data (default \"~/.near\")")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("agent-key")
                .long("agent-key")
                .default_value(&DEFAULT_AGENT_KEYFILE)
                .help("File storing key pair of the agent, for signing transactions (default agent_key.json")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("wait-period")
                .long("wait-period")
                .default_value(DEFAULT_WAIT_PERIOD_MILLISEC)
                .help("Waiting period between checking if there are more txns to be made (in milliseconds)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rpc-url")
                .long("rpc-url")
                .default_value(DEFAULT_RPC_URL)
                .help("Url of RPC for the node to transact")
                .takes_value(true),
        )
        .get_matches();

    let rpc_url = matches.value_of("rpc-url").unwrap();
    let home_dir = matches.value_of("home").map(|dir| Path::new(dir)).unwrap();
    let agent_key_file = matches.value_of("agent-key").map(|dir| Path::new(dir)).unwrap();
    let wait_period = matches
        .value_of("wait-period")
        .map(|s| s.parse().expect("Wait period must be a number"))
        .unwrap();

    let bp: String = format!("{}-credentials", home_dir.to_owned().to_str().unwrap());
    let base_path = Path::new(&bp).join(&agent_key_file);
    // generate_validator_key(DEFAULT_AGENT_ACCOUNT_ID, &base_path);
    let signer = InMemorySigner::from_file(&base_path);
    let account_id = signer.account_id.clone();

    // TODO: Abort if no balance, suggest user fill account with balance or request some.
    let user = RpcUser::new(rpc_url, account_id.clone(), Arc::new(signer));
    let status = user.get_status();
    println!("STATUS: {:?}", status);
    println!("Checking account {} balance... {}", &account_id, rpc_url);
    let bal = user.view_balance(&account_id);
    println!("User bal {:?}", bal);
    let state = user.view_state(&account_id, b"");
    println!("User state {:?}", state);
    let acct = user.view_account(&account_id);
    println!("User acct {:?}", acct);

    loop {
        // TODO:
        // - Check for any tasks
        let tasks: Vec<u8> = Vec::new();
        println!("Has tasks: {}", tasks.len());
        std::thread::sleep(Duration::from_millis(wait_period));
    }
}
