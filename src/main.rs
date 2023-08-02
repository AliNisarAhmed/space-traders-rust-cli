use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Status,
    Register {
        #[arg(short, long)]
        username: String,

        #[arg(short, long, default_value_t=String::from("COSMIC"))]
        faction: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::Status) => println!("Not logged in"),
        Some(Command::Register { username, faction }) => register_player(username, faction),
        None => println!("invalid command"),
    }
}

fn register_player(username: String, faction: String) {
    println!("registering...")
}
