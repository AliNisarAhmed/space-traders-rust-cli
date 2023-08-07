use std::{collections::HashMap, env};

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        AcceptContractResponse, Agent, MyContractsResponse, PurchaseShipResponse, RegisterResponse,
        Ship, ShipNav, ShipNavigateResponse, ShipOrbitResponse, ShipType, Waypoint,
        WaypointTraitSymbol,
    },
    UserInfo,
};

const API_BASE_URL: &str = "https://api.spacetraders.io/v2";

pub struct Api<'a> {
    client: Client,
    api_base_url: String,
    user_info: &'a UserInfo,
}

impl<'a> Api<'a> {
    pub fn new(user_info: &'a UserInfo) -> Self {
        let url = env::var("TEST_API_BASE_URL").unwrap_or(API_BASE_URL.to_owned());
        Api {
            client: Client::new(),
            api_base_url: url,
            user_info,
        }
    }

    pub async fn get_ship_nav_status(self: Self, ship_symbol: String) -> Result<ShipNav, Error> {
        let url = format!("{}/my/ships/{ship_symbol}/nav", self.api_base_url);
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await?
            .json::<ApiResponse<ShipNav>>()
            .await?;

        Ok(response.data)
    }

    pub async fn navigate_ship(
        self: Self,
        ship_symbol: String,
        waypoint_symbol: String,
    ) -> Result<ShipNavigateResponse, Error> {
        let url = format!("{}/my/ships/{ship_symbol}/navigate", self.api_base_url);
        let mut body = HashMap::new();
        body.insert("waypointSymbol", waypoint_symbol);
        let response = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(&self.user_info.token)
            .send()
            .await?
            .json::<ApiResponse<ShipNavigateResponse>>()
            .await?;
        Ok(response.data)
    }

    pub async fn orbit_ship(self: Self, ship_symbol: String) -> Result<ShipOrbitResponse, Error> {
        let url = format!("{}/my/ships/{ship_symbol}/orbit", self.api_base_url);
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.user_info.token)
            .header("Content-Length", 0)
            .send()
            .await?
            .json::<ApiResponse<ShipOrbitResponse>>()
            .await?;
        Ok(response.data)
    }

    pub async fn list_ships(self: Self) -> Result<Vec<Ship>, Error> {
        let url = self.api_base_url + "/my/ships";
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await?
            .json::<ApiResponse<Vec<Ship>>>()
            .await?;

        Ok(response.data)
    }

    pub async fn purchase_ship(
        self: Self,
        ship_type: ShipType,
        waypoint_symbol: String,
    ) -> Result<PurchaseShipResponse, Error> {
        let url = self.api_base_url + "/my/ships";
        let mut body = HashMap::new();
        body.insert("shipType", ship_type.to_string());
        body.insert("waypointSymbol", waypoint_symbol);
        let response = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(&self.user_info.token)
            .send()
            .await?
            .json::<ApiResponse<PurchaseShipResponse>>()
            .await?;
        Ok(response.data)
    }

    pub async fn fetch_agent_info(self: Self) -> Result<Agent, Error> {
        let url = self.api_base_url + "/my/agent";
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await?;
        let response = response.json::<ApiResponse<Agent>>().await?;
        Ok(response.data)
    }

    pub async fn list_waypoints(
        self: Self,
        system_symbol: String,
        waypoint_trait: Option<WaypointTraitSymbol>,
    ) -> Result<Vec<Waypoint>, Error> {
        let url = format!("{API_BASE_URL}/systems/{system_symbol}/waypoints");
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
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
        contract_id: String,
    ) -> Result<AcceptContractResponse, Error> {
        let url = format!("{}/my/contracts/{}/accept", API_BASE_URL, contract_id);
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.user_info.token)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Content-Length", 0)
            .send()
            .await?
            .json::<ApiResponse<AcceptContractResponse>>()
            .await?;
        Ok(response.data)
    }

    pub async fn fetch_contracts(self: Self) -> Result<MyContractsResponse, Error> {
        let url = API_BASE_URL.to_owned() + "/my/contracts";
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
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
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    total: i32,
    page: i32,
    limit: i32,
}
