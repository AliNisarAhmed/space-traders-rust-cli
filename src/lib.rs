#![allow(non_camel_case_types)]

use std::{error::Error, path::PathBuf};

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

// ---- API ----

pub mod api {
    use std::{collections::HashMap, env};

    use reqwest::{Client, Error};
    use serde::{Deserialize, Serialize};

    use crate::domain::{
        AcceptContractResponse, Agent, MyContractsResponse, RegisterResponse, Waypoint,
        WaypointTraitSymbol,
    };

    const API_BASE_URL: &str = "https://api.spacetraders.io/v2";

    pub struct Api {
        client: Client,
        api_base_url: String,
    }

    impl Api {
        pub fn new() -> Self {
            let url = env::var("TEST_API_BASE_URL").unwrap_or(API_BASE_URL.to_owned());
            Api {
                client: Client::new(),
                api_base_url: url,
            }
        }

        pub async fn fetch_agent_info(self: Self, token: String) -> Result<Agent, Error> {
            let url = self.api_base_url + "/my/agent";
            let response = self.client.get(url).bearer_auth(token).send().await?;
            let response = response.json::<ApiResponse<Agent>>().await?;
            Ok(response.data)
        }

        pub async fn list_waypoints(
            self: Self,
            token: String,
            system_symbol: String,
            waypoint_trait: Option<WaypointTraitSymbol>,
        ) -> Result<Vec<Waypoint>, Error> {
            let url = format!("{API_BASE_URL}/systems/{system_symbol}/waypoints");
            let response = self
                .client
                .get(url)
                .bearer_auth(token)
                .send()
                .await?
                .json::<ApiResponse<Vec<Waypoint>>>()
                .await?;
            if let Some(filter) = waypoint_trait {
                Ok(response
                    .data
                    .iter()
                    .cloned()
                    .filter(|wp| wp.traits.iter().any(|tr| tr.symbol == filter))
                    .collect())
            } else {
                Ok(response.data)
            }
        }

        pub async fn accept_contract(
            self: Self,
            token: String,
            contract_id: String,
        ) -> Result<AcceptContractResponse, Error> {
            let url = format!("{}/my/contracts/{}/accept", API_BASE_URL, contract_id);
            let response = self
                .client
                .post(url)
                .bearer_auth(token)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("Content-Length", 0)
                .send()
                .await?
                .json::<ApiResponse<AcceptContractResponse>>()
                .await?;
            Ok(response.data)
        }

        pub async fn fetch_contracts(
            self: Self,
            token: String,
        ) -> Result<MyContractsResponse, Error> {
            let url = API_BASE_URL.to_owned() + "/my/contracts";
            let response = self
                .client
                .get(url)
                .bearer_auth(token)
                .send()
                .await?
                .json::<ApiResponse<MyContractsResponse>>()
                .await?;
            Ok(response.data)
        }

        pub async fn register_player(
            self: Self,
            username: String,
            faction: String,
        ) -> Result<RegisterResponse, Error> {
            println!("registering...");
            let mut body = HashMap::new();
            body.insert("symbol", username);
            body.insert("faction", faction);
            let response = self
                .client
                .post(API_BASE_URL.to_owned() + "/register")
                .json(&body)
                .send()
                .await?
                .json::<ApiResponse<RegisterResponse>>()
                .await?;
            Ok(response.data)
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ApiResponse<T> {
        pub data: T,
    }
}

// ---- Domain ----

pub mod domain {
    use clap::ValueEnum;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Agent {
        pub symbol: String,
        pub headquarters: String,
        pub credits: i32,
        pub starting_faction: String,
        pub ship_count: Option<i32>,
    }

    impl Agent {
        pub fn get_sector(&self) -> String {
            let vec: Vec<&str> = self.headquarters.split("-").collect();
            String::from(vec[0])
        }

        pub fn get_system(&self) -> String {
            let vec: Vec<&str> = self.headquarters.split("-").collect();
            // format!("{vec[0]}-{vec[1]}")
            String::from(vec[0]) + "-" + vec[1]
        }

        pub fn get_location(&self) -> String {
            let vec: Vec<&str> = self.headquarters.split("-").collect();
            String::from(vec[2])
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct RegisterResponse {
        pub token: String,
        pub agent: Agent,
    }

    pub type MyContractsResponse = Vec<Contract>;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Contract {
        pub id: String,
        pub faction_symbol: String,
        #[serde(rename = "type")]
        pub contract_type: String,
        pub terms: ContractTerms,
        pub accepted: bool,
        pub fulfilled: bool,
        pub expiration: String,
        pub deadline_to_accept: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContractTerms {
        pub deadline: String,
        pub payment: PaymentTerms,
        pub deliver: Vec<DeliverTerms>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PaymentTerms {
        pub on_accepted: i32,
        pub on_fulfilled: i32,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DeliverTerms {
        pub trade_symbol: String,
        pub destination_symbol: String,
        pub units_required: i32,
        pub units_fulfilled: i32,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AcceptContractResponse {
        pub agent: Agent,
        pub contract: Contract,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Waypoint {
        pub symbol: String,
        #[serde(rename = "type")]
        pub waypoint_type: WaypointType,
        pub system_symbol: String,
        pub x: i32,
        pub y: i32,
        pub orbitals: Vec<Orbital>,
        pub faction: Faction,
        pub traits: Vec<WaypointTrait>,
        pub chart: Chart,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum WaypointType {
        PLANET,
        GAS_GIANT,
        MOON,
        ORBITAL_STATION,
        JUMP_GATE,
        ASTEROID_FIELD,
        NEBULA,
        DEBRIS_FIELD,
        GRAVITY_WELL,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Orbital {
        pub symbol: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Faction {
        // TODO: convert this to an enum
        pub symbol: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Chart {
        pub waypoint_symbol: Option<String>,
        pub submitted_by: String,
        pub submitted_on: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct WaypointTrait {
        pub symbol: WaypointTraitSymbol,
        pub name: String,
        pub description: String,
    }

    #[derive(
        Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Deserialize, Serialize,
    )]
    pub enum WaypointTraitSymbol {
        UNCHARTED,
        MARKETPLACE,
        SHIPYARD,
        OUTPOST,
        OVERCROWDED,
        CORRUPT,
        BUREAUCRATIC,
        INDUSTRIAL,
        JOVIAN,
        ROCKY,
        VOLCANIC,
        FROZEN,
        SWAMP,
        BARREN,
        TEMPERATE,
        JUNGLE,
        OCEAN,
        STRIPPED,
        TOXIC_ATMOSPHERE,
        SCATTERED_SETTLEMENTS,
        SPRAWLING_CITIES,
        MEGA_STRUCTURES,
        HIGH_TECH,
        TRADING_HUB,
        BLACK_MARKET,
        RESEARCH_FACILITY,
        MILITARY_BASE,
        SURVEILLANCE_OUTPOST,
        EXPLORATION_OUTPOST,
        MINERAL_DEPOSITS,
        COMMON_METAL_DEPOSITS,
        PRECIOUS_METAL_DEPOSITS,
        RARE_METAL_DEPOSITS,
        METHANE_POOLS,
        ICE_CRYSTALS,
        EXPLOSIVE_GASES,
        STRONG_MAGNETOSPHERE,
        VIBRANT_AURORAS,
        SALT_FLATS,
        CANYONS,
        PERPETUAL_DAYLIGHT,
        PERPETUAL_OVERCAST,
        DRY_SEABEDS,
        MAGMA_SEAS,
        SUPERVOLCANOES,
        ASH_CLOUDS,
        VAST_RUINS,
        MUTATED_FLORA,
        TERRAFORMED,
        EXTREME_TEMPERATURES,
        EXTREME_PRESSURE,
        DIVERSE_LIFE,
        SCARCE_LIFE,
        FOSSILS,
        WEAK_GRAVITY,
        STRONG_GRAVITY,
        CRUSHING_GRAVITY,
        CORROSIVE_ATMOSPHERE,
        BREATHABLE_ATMOSPHERE,
    }
}
