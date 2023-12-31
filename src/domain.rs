use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipOrbitResponse {
    pub nav: ShipNav,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavigateResponse {
    pub fuel: ShipFuel,
    pub nav: ShipNav,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipDockResponse {
    pub nav: ShipNav,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipRefuelResponse {
    agent: Agent,
    fuel: ShipFuel,
    transaction: MarketTransaction,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractResourceResponse {
    pub cooldown: Cooldown,
    pub extraction: Extraction,
    pub cargo: ShipCargo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SellCargoResponse {
    pub agent: Agent,
    pub cargo: ShipCargo,
    pub transaction: MarketTransaction,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliverCargoResponse {
    pub cargo: ShipCargo,
    pub contract: Contract,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FulfillContractResponse {
    pub agent: Agent,
    pub contract: Contract,
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

impl Waypoint {
    pub fn get_system_id(waypoint_symbol: &str) -> String {
        let vec: Vec<&str> = waypoint_symbol.split("-").collect();
        String::from(vec[0]) + "-" + vec[1]
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, ValueEnum)]
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Deserialize, Serialize)]
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

#[derive(Display, Debug, Serialize, Deserialize, ValueEnum, Clone)]
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
    pub route: ShipNavRoute,
    pub status: ShipNavStatus,
    pub flight_mode: ShipNavFlightMode,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRoute {
    pub destination: ShipNavRouteWaypoint,
    pub departure: ShipNavRouteWaypoint,
    pub departure_time: String,
    pub arrival: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShipNavStatus {
    IN_TRANSIT,
    IN_ORBIT,
    DOCKED,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShipNavFlightMode {
    DRIFT,
    STEALTH,
    CRUISE,
    BURN,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRouteWaypoint {
    pub symbol: String,
    #[serde(rename = "type")]
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
    pub condition: Option<i32>,
    pub module_slots: i32,
    pub mounting_points: i32,
    pub fuel_capacity: i32,
    pub requirements: ShipRequirements,
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
pub struct ShipRequirements {
    pub power: Option<i32>,
    pub crew: Option<i32>,
    pub slots: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipReactor {
    pub symbol: ShipReactorSymbol,
    pub name: String,
    pub description: String,
    pub condition: Option<i32>,
    pub power_output: i32,
    pub requirements: ShipRequirements,
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
    pub condition: Option<i32>,
    pub speed: i32,
    pub requirements: ShipRequirements,
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
    pub capacity: Option<i32>,
    pub range: Option<i32>,
    pub name: String,
    pub description: String,
    pub requirements: ShipRequirements,
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
    pub description: Option<String>,
    pub strength: Option<i32>,
    pub deposits: Option<Vec<Deposit>>,
    pub requirements: ShipRequirements,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketTransaction {
    pub waypoint_symbol: String,
    pub ship_symbol: String,
    pub trade_symbol: String,
    #[serde(rename = "type")]
    pub transaction_type: MarketTransactionType,
    pub units: i32,
    pub price_per_unit: i32,
    pub total_price: i32,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MarketTransactionType {
    PURCHASE,
    SELL,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub ship_symbol: String,
    pub total_seconds: i32,
    pub remaining_seconds: i32,
    pub expiration: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extraction {
    pub ship_symbol: String,
    #[serde(rename = "yield")]
    pub extraction_yield: ExtractionYield,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractionYield {
    pub symbol: TradeSymbol,
    pub units: i32,
}

#[derive(Display, Debug, Serialize, Deserialize, ValueEnum, Clone)]
pub enum TradeSymbol {
    PRECIOUS_STONES,
    QUARTZ_SAND,
    SILICON_CRYSTALS,
    AMMONIA_ICE,
    LIQUID_HYDROGEN,
    LIQUID_NITROGEN,
    ICE_WATER,
    EXOTIC_MATTER,
    ADVANCED_CIRCUITRY,
    GRAVITON_EMITTERS,
    IRON,
    IRON_ORE,
    COPPER,
    COPPER_ORE,
    ALUMINUM,
    ALUMINUM_ORE,
    SILVER,
    SILVER_ORE,
    GOLD,
    GOLD_ORE,
    PLATINUM,
    PLATINUM_ORE,
    DIAMONDS,
    URANITE,
    URANITE_ORE,
    MERITIUM,
    MERITIUM_ORE,
    HYDROCARBON,
    ANTIMATTER,
    FERTILIZERS,
    FABRICS,
    FOOD,
    JEWELRY,
    MACHINERY,
    FIREARMS,
    ASSAULT_RIFLES,
    MILITARY_EQUIPMENT,
    EXPLOSIVES,
    LAB_INSTRUMENTS,
    AMMUNITION,
    ELECTRONICS,
    SHIP_PLATING,
    EQUIPMENT,
    FUEL,
    MEDICINE,
    DRUGS,
    CLOTHING,
    MICROPROCESSORS,
    PLASTICS,
    POLYNUCLEOTIDES,
    BIOCOMPOSITES,
    NANOBOTS,
    AI_MAINFRAMES,
    QUANTUM_DRIVES,
    ROBOTIC_DRONES,
    CYBER_IMPLANTS,
    GENE_THERAPEUTICS,
    NEURAL_CHIPS,
    MOOD_REGULATORS,
    VIRAL_AGENTS,
    MICRO_FUSION_GENERATORS,
    SUPERGRAINS,
    LASER_RIFLES,
    HOLOGRAPHICS,
    SHIP_SALVAGE,
    RELIC_TECH,
    NOVEL_LIFEFORMS,
    BOTANICAL_SPECIMENS,
    CULTURAL_ARTIFACTS,
    REACTOR_SOLAR_I,
    REACTOR_FUSION_I,
    REACTOR_FISSION_I,
    REACTOR_CHEMICAL_I,
    REACTOR_ANTIMATTER_I,
    ENGINE_IMPULSE_DRIVE_I,
    ENGINE_ION_DRIVE_I,
    ENGINE_ION_DRIVE_II,
    ENGINE_HYPER_DRIVE_I,
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
#[serde(rename_all = "camelCase")]
pub struct Survey {
    pub signature: String,
    pub symbol: String,
    pub deposits: Vec<SurveyDeposit>,
    pub expiration: String,
    pub size: DepositSize,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DepositSize {
    SMALL,
    MODERATE,
    LARGE,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurveyDeposit {
    symbol: Deposit,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub symbol: String,
    pub exports: Vec<TradeGood>,
    pub imports: Vec<TradeGood>,
    pub exchange: Vec<TradeGood>,
    pub transactions: Vec<MarketTransaction>,
    pub trade_goods: Vec<MarketTradeGood>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeGood {
    pub symbol: TradeSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketTradeGood {
    pub symbol: String,
    pub trade_volume: u32,
    pub supply: Supply,
    pub purchase_price: u32,
    pub sell_price: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Supply {
    SCARCE,
    LIMITED,
    MODERATE,
    ABUNDANT,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shipyard {
    pub symbol: String,
    pub ship_types: Vec<ShipyardShipTypes>,
    pub transactions: Vec<ShipyardTransaction>,
    pub ships: Vec<ShipyardShip>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardTransaction {
    pub waypoint_symbol: String,
    pub ship_symbol: String,
    pub price: u32,
    pub agent_symbol: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardShip {
    #[serde(rename = "type")]
    pub ship_type: ShipType,
    pub name: String,
    pub description: String,
    pub purchase_price: u32,
    pub frame: ShipFrame,
    pub reactor: ShipReactor,
    pub engine: ShipEngine,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipyardShipTypes {
    #[serde(rename = "type")]
    pub ship_type: ShipType,
}
