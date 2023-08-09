use std::{collections::HashMap, env};

use reqwest::{Client, Error, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    domain::{
        AcceptContractResponse, Agent, ExtractResourceResponse, Market, MyContractsResponse,
        PurchaseShipResponse, RegisterResponse, SellCargoResponse, Ship, ShipCargo,
        ShipDockResponse, ShipNav, ShipNavigateResponse, ShipOrbitResponse, ShipRefuelResponse,
        ShipType, Survey, TradeSymbol, Waypoint, WaypointTraitSymbol,
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

    pub async fn get_market(
        self: Self,
        system_symbol: String,
        waypoint_symbol: String,
    ) -> ApiResult<Market> {
        let url = format!(
            "{}/systems/{system_symbol}/waypoints/{waypoint_symbol}/market",
            self.api_base_url
        );
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }

    pub async fn sell_ship_cargo(
        self: Self,
        ship_symbol: String,
        good_type: TradeSymbol,
        units: u32,
    ) -> ApiResult<SellCargoResponse> {
        let url = format!("{}/my/ships/{ship_symbol}/sell", self.api_base_url);
        let mut body = HashMap::new();
        body.insert("symbol", good_type.to_string());
        body.insert("units", units.to_string());
        let response = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        dbg!(&response);
        handle_api_response(response).await
    }

    pub async fn get_ship_cargo(self: Self, ship_symbol: String) -> ApiResult<ShipCargo> {
        let url = format!("{}/my/ships/{ship_symbol}/cargo", self.api_base_url);
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }
    pub async fn extract_resource(
        self: Self,
        ship_symbol: String,
        resource_survey: Option<Survey>,
    ) -> ApiResult<ExtractResourceResponse> {
        let url = format!("{}/my/ships/{ship_symbol}/extract", self.api_base_url);
        let mut body = HashMap::new();
        if let Some(survey) = resource_survey {
            body.insert("survey", serde_json::to_string(&survey).unwrap());
        }
        let response = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }

    pub async fn refuel_ship(
        self: Self,
        ship_symbol: String,
        maybe_units: Option<i32>,
    ) -> ApiResult<ShipRefuelResponse> {
        let url = format!("{}/my/ships/{ship_symbol}/refuel", self.api_base_url);
        let mut body = HashMap::new();
        if let Some(units_to_refuel) = maybe_units {
            body.insert("units", units_to_refuel.to_string());
        }
        let response = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }

    pub async fn get_ship_status(self: Self, ship_symbol: String) -> ApiResult<Ship> {
        let url = format!("{}/my/ships/{ship_symbol}", self.api_base_url);
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }

    pub async fn dock_ship(self: Self, ship_symbol: String) -> ApiResult<ShipDockResponse> {
        let url = format!("{}/my/ships/{ship_symbol}/dock", self.api_base_url);
        let response = self
            .client
            .post(url)
            .header("Content-Length", 0)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }

    pub async fn get_ship_nav_status(self: Self, ship_symbol: String) -> ApiResult<ShipNav> {
        let url = format!("{}/my/ships/{ship_symbol}/nav", self.api_base_url);
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }

    pub async fn navigate_ship(
        self: Self,
        ship_symbol: String,
        waypoint_symbol: String,
    ) -> ApiResult<ShipNavigateResponse> {
        let url = format!("{}/my/ships/{ship_symbol}/navigate", self.api_base_url);
        let mut body = HashMap::new();
        body.insert("waypointSymbol", waypoint_symbol);
        let response = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }
    //
    pub async fn orbit_ship(self: Self, ship_symbol: String) -> ApiResult<ShipOrbitResponse> {
        let url = format!("{}/my/ships/{ship_symbol}/orbit", self.api_base_url);
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.user_info.token)
            .header("Content-Length", 0)
            .send()
            .await;
        handle_api_response(response).await
    }
    //
    pub async fn list_ships(self: Self) -> ApiResult<Vec<Ship>> {
        let url = self.api_base_url + "/my/ships";
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }
    //
    pub async fn purchase_ship(
        self: Self,
        ship_type: ShipType,
        waypoint_symbol: String,
    ) -> ApiResult<PurchaseShipResponse> {
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
            .await;
        handle_api_response(response).await
    }

    pub async fn fetch_agent_info(self: Self) -> ApiResult<Agent> {
        let url = self.api_base_url + "/my/agent";
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }

    pub async fn list_waypoints(
        self: Self,
        system_symbol: String,
        waypoint_trait: Option<WaypointTraitSymbol>,
    ) -> ApiResult<Vec<Waypoint>> {
        let url = format!("{API_BASE_URL}/systems/{system_symbol}/waypoints");
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        let api_response = handle_api_response::<Vec<Waypoint>>(response)
            .await
            .unwrap();
        if let Some(filter) = waypoint_trait {
            let new_data = api_response
                .data
                .iter()
                .cloned()
                .filter(|wp| wp.traits.iter().any(|tr| tr.symbol == filter))
                .collect();
            Ok(ApiSuccessResponse {
                data: new_data,
                ..api_response
            })
        } else {
            Ok(api_response)
        }
    }

    pub async fn accept_contract(
        self: Self,
        contract_id: String,
    ) -> ApiResult<AcceptContractResponse> {
        let url = format!("{}/my/contracts/{}/accept", API_BASE_URL, contract_id);
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.user_info.token)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Content-Length", 0)
            .send()
            .await;
        handle_api_response(response).await
    }

    pub async fn fetch_contracts(self: Self) -> ApiResult<MyContractsResponse> {
        let url = API_BASE_URL.to_owned() + "/my/contracts";
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.user_info.token)
            .send()
            .await;
        handle_api_response(response).await
    }

    pub async fn register_player(
        self: Self,
        username: String,
        faction: String,
    ) -> ApiResult<RegisterResponse> {
        println!("registering...");
        let mut body = HashMap::new();
        body.insert("symbol", username);
        body.insert("faction", faction);
        let response = self
            .client
            .post(API_BASE_URL.to_owned() + "/register")
            .json(&body)
            .send()
            .await;
        handle_api_response(response).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiSuccessResponse<T> {
    pub data: T,
    pub meta: Option<Meta>,
}

type ApiResult<T> = anyhow::Result<ApiSuccessResponse<T>, ApiError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorResponse {
    error: ApiErrorObj,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorObj {
    message: String,
    code: i32,
    data: Option<HashMap<String, String>>,
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("HTTP API error")]
    ServiceError {
        status: u16,
        message: String,
        code: i32,
        data: Option<HashMap<String, String>>,
    },
    #[error("Error parsing JSON response from API")]
    ParseError { message: String },

    #[error("Unknown error")]
    UnknownError { message: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    total: i32,
    page: i32,
    limit: i32,
}

pub async fn handle_api_response<T: DeserializeOwned>(
    response: Result<Response, Error>,
) -> ApiResult<T> {
    match response {
        Err(e) => Err(ApiError::UnknownError {
            message: e.to_string(),
        }),
        Ok(api_response) => {
            let status_code = api_response.status();

            if status_code.is_success() {
                match api_response.json::<ApiSuccessResponse<T>>().await {
                    Ok(parsed) => Ok(parsed),
                    Err(error) => Err(ApiError::ParseError {
                        message: error.to_string(),
                    }),
                }
            } else {
                let service_response = api_response.json::<ApiErrorResponse>().await.unwrap();
                Err(ApiError::ServiceError {
                    message: service_response.error.message,
                    code: service_response.error.code,
                    status: status_code.as_u16(),
                    data: service_response.error.data,
                })
            }
        }
    }
}
