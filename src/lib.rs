#![allow(non_camel_case_types)]

use std::{error::Error, path::PathBuf};

pub mod api;
pub mod domain;

use api::Api;
use clap::{Args, Parser, Subcommand};
use domain::*;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct AppArgs {
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
    MyContracts,
    AcceptContract {
        #[arg(short, long)]
        contract_id: String,
    },
    #[command(alias = "whoami")]
    WhoAmI,
    Waypoints(WaypointSubCommand),
    Ship(ShipSubCommand),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct WaypointSubCommand {
    #[command(subcommand)]
    command: WaypointSubCommandArgs,
}

#[derive(Debug, Subcommand)]
enum WaypointSubCommandArgs {
    List {
        #[arg(short = 't')]
        filter: Option<WaypointTraitSymbol>,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct ShipSubCommand {
    #[command(subcommand)]
    command: ShipSubCommandArgs,
}

#[derive(Debug, Subcommand)]
enum ShipSubCommandArgs {
    List,
    Purchase {
        #[arg(short, long)]
        ship_type: ShipType,
        #[arg(short, long)]
        waypoint_symbol: String,
    },
    Orbit {
        #[arg(short, long)]
        ship_symbol: String,
    },
    Navigate {
        #[arg(short, long)]
        ship_symbol: String,
        #[arg(short, long)]
        waypoint_symbol: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub token: String,
    pub agent: Agent,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    pub current_user_dir: Box<PathBuf>,
}

pub fn get_args() -> MyResult<AppArgs> {
    Ok(AppArgs::parse())
}

pub async fn run<'a>(args: AppArgs, config: Config) -> MyResult<()> {
    let api = Api::new();

    if let Some(user_info) = auth::check_user_token(&config.current_user_dir) {
        match args.command {
            Some(Command::Status) => println!("You are logged in as {:?}", user_info.agent.symbol),
            Some(Command::Register { username, faction }) => {
                let res = api.register_player(username, faction).await.unwrap();
                auth::save_user_info(&res, &config.current_user_dir);
            }
            Some(Command::WhoAmI) => {
                println!("fetching Agent info...");
                let res = api.fetch_agent_info(user_info.token).await.unwrap();
                println!("{:#?}", res)
            }
            Some(Command::MyContracts) => {
                let res = api.fetch_contracts(user_info.token).await.unwrap();
                println!("{:#?}", res)
            }
            Some(Command::AcceptContract { contract_id }) => {
                let res = api
                    .accept_contract(user_info.token, contract_id)
                    .await
                    .unwrap();
                println!("{:#?}", res)
            }
            Some(Command::Waypoints(WaypointSubCommand { command })) => match command {
                WaypointSubCommandArgs::List { filter } => {
                    let res = api
                        .list_waypoints(user_info.token, user_info.agent.get_system(), filter)
                        .await
                        .unwrap();
                    println!("{:#?}", res)
                }
            },
            Some(Command::Ship(ShipSubCommand { command })) => match command {
                ShipSubCommandArgs::Purchase {
                    ship_type,
                    waypoint_symbol,
                } => {
                    let res = api
                        .purchase_ship(user_info.token, ship_type, waypoint_symbol)
                        .await
                        .unwrap();
                    println!("{:#?}", res)
                }
                ShipSubCommandArgs::List => {
                    let res = api.list_ships(user_info.token).await.unwrap();
                    println!("{:#?}", res)
                }
                ShipSubCommandArgs::Orbit { ship_symbol } => {
                    let res = api.orbit_ship(user_info.token, ship_symbol).await.unwrap();
                    println!("{:#?}", res)
                }
                ShipSubCommandArgs::Navigate {
                    ship_symbol,
                    waypoint_symbol,
                } => {
                    let res = api
                        .navigate_ship(user_info.token, ship_symbol, waypoint_symbol)
                        .await
                        .unwrap();
                    println!("{:#?}", res)
                }
            },
            None => println!("invalid command"),
        }
    } else {
        println!("please log in first by typing st-app login <username>")
    }
    Ok(())
}

// ---- AUTH ----

pub mod auth {
    use std::{
        fs::File,
        io::{BufReader, BufWriter, Write},
        path::Path,
    };

    use crate::{domain::RegisterResponse, UserInfo};

    pub fn save_user_info(register_resp: &RegisterResponse, user_dir: &Path) {
        let user_info: UserInfo = UserInfo {
            agent: register_resp.agent.to_owned(),
            token: register_resp.token.to_owned(),
        };
        let file_path = user_dir.join("current_user.json");
        let token_file = File::create(file_path).unwrap();
        let mut writer = BufWriter::new(token_file);
        serde_json::to_writer(&mut writer, &user_info).unwrap();
        writer.flush().unwrap();
    }

    pub fn check_user_token(current_user_dir: &Path) -> Option<UserInfo> {
        let token_file = current_user_dir.join("current_user.json");
        if token_file.exists() {
            let token_file = File::open(token_file).unwrap();
            let reader = BufReader::new(token_file);
            let user_info: UserInfo = serde_json::from_reader(reader).unwrap();
            Some(user_info)
        } else {
            None
        }
    }
}
