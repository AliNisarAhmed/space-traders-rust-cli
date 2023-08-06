#![allow(non_camel_case_types)]

use std::{error::Error, path::PathBuf};

pub mod api;

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
    pub struct AcceptContractResponse {
        pub agent: Agent,
        pub contract: Contract,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PurchaseShipResponse {
        pub agent: Agent,
        pub ship: Ship,
        pub transaction: Transaction,
    }

    // ---------------------------------------------

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

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ShipType {
        SHIP_PROBE,
        SHIP_MINING_DRONE,
        SHIP_INTERCEPTOR,
        SHIP_LIGHT_HAULER,
        SHIP_COMMAND_FRIGATE,
        SHIP_EXPLORER,
        SHIP_HEAVY_FREIGHTER,
        SHIP_LIGHT_SHUTTLE,
        SHIP_ORE_HOUND,
        SHIP_REFINING_FREIGHTER,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Ship {
        pub symbol: String,
        pub registration: ShipRegistration,
        pub nav: ShipNav,
        pub crew: ShipCrew,
        pub frame: ShipFrame,
        pub reactor: ShipReactor,
        pub engine: ShipEngine,
        pub modules: Vec<ShipModule>,
        pub mounts: Vec<ShipMount>,
        pub cargo: ShipCargo,
        pub fuel: ShipFuel,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipRegistration {
        pub name: String,
        pub faction_symbol: String,
        pub role: ShipRole,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ShipRole {
        FABRICATOR,
        HARVESTOR,
        HAULER,
        INTERCEPTOR,
        EXCAVATOR,
        TRANSPORT,
        REPAIR,
        SURVEYOR,
        COMMAND,
        CARRIER,
        PATROL,
        SATELLITE,
        EXPLORER,
        REFINERY,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipNav {
        pub system_symbol: String,
        pub waypoint_symbol: String,
        pub route: Route,
        pub status: String,
        pub flight_mode: FlightMode,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Route {
        pub destination: Location,
        pub departure: Location,
        pub departure_time: String,
        pub arrival: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum FlightMode {
        DRIFT,
        STEALTH,
        CRUISE,
        BURN,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Location {
        pub symbol: String,
        pub waypoint_type: WaypointType,
        pub system_symbol: String,
        pub x: i32,
        pub y: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipCrew {
        pub current: i32,
        pub required: i32,
        pub capacity: i32,
        pub rotation: CrewRotation,
        pub morale: i32,
        pub wages: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum CrewRotation {
        STRICT,
        RELAXED,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipFrame {
        pub symbol: ShipFrameSymbol,
        pub name: String,
        pub description: String,
        pub condition: i32,
        pub module_slots: i32,
        pub mounting_points: i32,
        pub fuel_capacity: i32,
        pub requirements: Requirement,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ShipFrameSymbol {
        FRAME_PROBE,
        FRAME_DRONE,
        FRAME_INTERCEPTOR,
        FRAME_RACER,
        FRAME_FIGHTER,
        FRAME_FRIGATE,
        FRAME_SHUTTLE,
        FRAME_EXPLORER,
        FRAME_MINER,
        FRAME_LIGHT_FREIGHTER,
        FRAME_HEAVY_FREIGHTER,
        FRAME_TRANSPORT,
        FRAME_DESTROYER,
        FRAME_CRUISER,
        FRAME_CARRIER,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Requirement {
        pub power: i32,
        pub crew: i32,
        pub slots: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipReactor {
        pub symbol: ShipReactorSymbol,
        pub name: String,
        pub description: String,
        pub condition: i32,
        pub power_output: i32,
        pub requirements: Requirement,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ShipReactorSymbol {
        REACTOR_SOLAR_I,
        REACTOR_FUSION_I,
        REACTOR_FISSION_I,
        REACTOR_CHEMICAL_I,
        REACTOR_ANTIMATTER_I,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipEngine {
        pub symbol: ShipEngineSymbol,
        pub name: String,
        pub description: String,
        pub condition: i32,
        pub speed: i32,
        pub requirements: Requirement,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ShipEngineSymbol {
        ENGINE_IMPULSE_DRIVE_I,
        ENGINE_ION_DRIVE_I,
        ENGINE_ION_DRIVE_II,
        ENGINE_HYPER_DRIVE_I,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipModule {
        pub symbol: ShipModuleSymbol,
        pub capacity: i32,
        pub range: i32,
        pub name: String,
        pub description: String,
        pub requirements: Requirement,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ShipModuleSymbol {
        MODULE_MINERAL_PROCESSOR_I,
        MODULE_CARGO_HOLD_I,
        MODULE_CREW_QUARTERS_I,
        MODULE_ENVOY_QUARTERS_I,
        MODULE_PASSENGER_CABIN_I,
        MODULE_MICRO_REFINERY_I,
        MODULE_ORE_REFINERY_I,
        MODULE_FUEL_REFINERY_I,
        MODULE_SCIENCE_LAB_I,
        MODULE_JUMP_DRIVE_I,
        MODULE_JUMP_DRIVE_II,
        MODULE_JUMP_DRIVE_III,
        MODULE_WARP_DRIVE_I,
        MODULE_WARP_DRIVE_II,
        MODULE_WARP_DRIVE_III,
        MODULE_SHIELD_GENERATOR_I,
        MODULE_SHIELD_GENERATOR_II,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipMount {
        pub symbol: ShipMountSymbol,
        pub name: String,
        pub description: String,
        pub strength: i32,
        pub deposits: Vec<Deposit>,
        pub requirements: Requirement,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum ShipMountSymbol {
        MOUNT_GAS_SIPHON_I,
        MOUNT_GAS_SIPHON_II,
        MOUNT_GAS_SIPHON_III,
        MOUNT_SURVEYOR_I,
        MOUNT_SURVEYOR_II,
        MOUNT_SURVEYOR_III,
        MOUNT_SENSOR_ARRAY_I,
        MOUNT_SENSOR_ARRAY_II,
        MOUNT_SENSOR_ARRAY_III,
        MOUNT_MINING_LASER_I,
        MOUNT_MINING_LASER_II,
        MOUNT_MINING_LASER_III,
        MOUNT_LASER_CANNON_I,
        MOUNT_MISSILE_LAUNCHER_I,
        MOUNT_TURRET_I,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Deposit {
        QUARTZ_SAND,
        SILICON_CRYSTALS,
        PRECIOUS_STONES,
        ICE_WATER,
        AMMONIA_ICE,
        IRON_ORE,
        COPPER_ORE,
        SILVER_ORE,
        ALUMINUM_ORE,
        GOLD_ORE,
        PLATINUM_ORE,
        DIAMONDS,
        URANITE_ORE,
        MERITIUM_ORE,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipCargo {
        pub capacity: i32,
        pub units: i32,
        pub inventory: Vec<Inventory>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Inventory {
        pub symbol: String,
        pub name: String,
        pub description: String,
        pub units: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShipFuel {
        pub current: i32,
        pub capacity: i32,
        pub consumed: Option<Consumed>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Consumed {
        pub amount: i32,
        pub timestamp: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Transaction {
        pub waypoint_symbol: String,
        pub ship_symbol: String,
        pub price: i32,
        pub agent_symbol: String,
        pub timestamp: String,
    }
}
