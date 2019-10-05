use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
enum Command {
    #[structopt(about = "Get the string value of a given string key")]
    Get { key: String },
    #[structopt(about = "Set the value of a string key to a string")]
    Set { key: String, value: String },
    #[structopt(about = "Remove a given key")]
    Rm { key: String },
}

fn run() -> kvs::Result<()> {
    let mut store = kvs::KvStore::new()?;
    match Command::from_args() {
        Command::Get { key } => {
            if let Some(value) = store.get(key).unwrap() {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        Command::Set { key, value } => store.set(key, value),
        Command::Rm { key } => store.remove(key),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        exit(1);
    }
}
