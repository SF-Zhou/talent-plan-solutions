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

fn main() {
    match Command::from_args() {
        Command::Get { .. } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Set { .. } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Command::Rm { .. } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
