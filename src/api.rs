use std::{collections::HashMap, env};

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::domain::{
    AcceptContractResponse, Agent, MyContractsResponse, PurchaseShipResponse, RegisterResponse,
    Waypoint, WaypointTraitSymbol,
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

    pub async fn purchase_ship(
        self: Self,
        token: String,
        ship_type: String,
        waypoint_symbol: String,
    ) -> Result<PurchaseShipResponse, Error> {
        let url = self.api_base_url + "/my/ships";
        let mut body = HashMap::new();
        body.insert("shipType", ship_type);
        body.insert("waypointSymbol", waypoint_symbol);
        let response = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(token)
            .send()
            .await?
            .json::<ApiResponse<PurchaseShipResponse>>()
            .await?;
        Ok(response.data)
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

    pub async fn fetch_contracts(self: Self, token: String) -> Result<MyContractsResponse, Error> {
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
