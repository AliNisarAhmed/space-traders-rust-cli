use app_dirs2::{app_dir, AppDataType, AppInfo};
use clap::{Parser, Subcommand};
use domain::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use crate::api::{accept_contract, fetch_agent_info, fetch_contracts, register_player};

const APP_INFO: AppInfo = AppInfo {
    name: "space-traders-cli-rust",
    author: "Ali Ahmed",
};

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
    MyContracts,
    AcceptContract {
        #[arg(short, long)]
        contract_id: String,
    },
    #[command(alias = "whoami")]
    WhoAmI,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserInfo {
    token: String,
    agent: Agent,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let current_user_dir = app_dir(AppDataType::UserData, &APP_INFO, "current-user").unwrap();
    let client = reqwest::Client::new();

    if let Some(user_info) = check_user_token(&current_user_dir) {
        match args.command {
            Some(Command::Status) => println!("You are logged in as {:?}", user_info.agent.symbol),
            Some(Command::Register { username, faction }) => {
                let res = register_player(username, faction).await.unwrap();
                save_user_info(res, current_user_dir);
            }
            Some(Command::WhoAmI) => {
                println!("fetching Agent info...");
                let res = fetch_agent_info(client, user_info.token).await.unwrap();
                println!("{:#?}", res)
            }
            Some(Command::MyContracts) => {
                let res = fetch_contracts(client, user_info.token).await.unwrap();
                println!("{:#?}", res)
            }
            Some(Command::AcceptContract { contract_id }) => {
                let res = accept_contract(client, user_info.token, contract_id)
                    .await
                    .unwrap();
                println!("{:#?}", res)
            }
            None => println!("invalid command"),
        }
    } else {
        println!("please log in first by typing st-app login <username>")
    }
}

fn save_user_info(register_resp: RegisterResponse, user_dir: PathBuf) {
    let user_info: UserInfo = UserInfo {
        agent: register_resp.agent,
        token: register_resp.token,
    };
    let file_path = user_dir.join("current_user.json");
    let token_file = File::open(file_path).unwrap();
    let mut writer = BufWriter::new(token_file);
    serde_json::to_writer(&mut writer, &user_info).unwrap();
}

fn check_user_token(current_user_dir: &PathBuf) -> Option<UserInfo> {
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

// ---- API ----

mod api {
    use std::collections::HashMap;

    use reqwest::{Client, Error};
    use serde::{Deserialize, Serialize};

    use crate::domain::{
        AcceptContractResponse, MyAgentResponse, MyContractsResponse, RegisterResponse,
    };

    const API_BASE_URL: &str = "https://api.spacetraders.io/v2";

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ApiResponse<T> {
        data: T,
    }

    pub async fn accept_contract(
        client: Client,
        token: String,
        contract_id: String,
    ) -> Result<AcceptContractResponse, Error> {
        let url = API_BASE_URL.to_owned() + "/my/contracts/${contract_id}/accept";
        let response = client
            .post(url)
            .bearer_auth(token)
            .send()
            .await?
            .json::<ApiResponse<AcceptContractResponse>>()
            .await?;
        Ok(response.data)
    }
    pub async fn fetch_contracts(
        client: Client,
        token: String,
    ) -> Result<MyContractsResponse, Error> {
        let url = API_BASE_URL.to_owned() + "/my/contracts";
        let response = client
            .get(url)
            .bearer_auth(token)
            .send()
            .await?
            .json::<ApiResponse<MyContractsResponse>>()
            .await?;
        Ok(response.data)
    }

    pub async fn fetch_agent_info(client: Client, token: String) -> Result<MyAgentResponse, Error> {
        let url = API_BASE_URL.to_owned() + "/my/agent";
        let response = client
            .get(url)
            .bearer_auth(token)
            .send()
            .await?
            .json::<ApiResponse<MyAgentResponse>>()
            .await?;
        Ok(response.data)
    }

    pub async fn register_player(
        username: String,
        faction: String,
    ) -> Result<RegisterResponse, Error> {
        println!("registering...");
        let client = reqwest::Client::new();
        let mut body = HashMap::new();
        body.insert("symbol", username);
        body.insert("faction", faction);
        let response = client
            .post(API_BASE_URL.to_owned() + "/register")
            .json(&body)
            .send()
            .await?
            .json::<ApiResponse<RegisterResponse>>()
            .await?;
        Ok(response.data)
    }
}

// ---- Domain ----

mod domain {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Agent {
        pub symbol: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct RegisterResponse {
        pub token: String,
        pub agent: Agent,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MyAgentResponse {
        pub account_id: String,
        pub symbol: String,
        pub headquarters: String,
        pub credits: i32,
        pub starting_faction: String,
        pub ship_count: Option<i32>,
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
}
