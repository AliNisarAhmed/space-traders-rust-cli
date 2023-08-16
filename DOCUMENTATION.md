# Command-Line Help for `space_traders_rust`

This document contains the help content for the `space_traders_rust` command-line program.

**Command Overview:**

* [`space_traders_rust`↴](#space_traders_rust)
* [`space_traders_rust generate-doc`↴](#space_traders_rust-generate-doc)
* [`space_traders_rust status`↴](#space_traders_rust-status)
* [`space_traders_rust register`↴](#space_traders_rust-register)
* [`space_traders_rust contract`↴](#space_traders_rust-contract)
* [`space_traders_rust contract list`↴](#space_traders_rust-contract-list)
* [`space_traders_rust contract accept`↴](#space_traders_rust-contract-accept)
* [`space_traders_rust contract deliver`↴](#space_traders_rust-contract-deliver)
* [`space_traders_rust contract fulfill`↴](#space_traders_rust-contract-fulfill)
* [`space_traders_rust who-am-i`↴](#space_traders_rust-who-am-i)
* [`space_traders_rust waypoint`↴](#space_traders_rust-waypoint)
* [`space_traders_rust waypoint get`↴](#space_traders_rust-waypoint-get)
* [`space_traders_rust waypoint list`↴](#space_traders_rust-waypoint-list)
* [`space_traders_rust waypoint market`↴](#space_traders_rust-waypoint-market)
* [`space_traders_rust waypoint shipyard`↴](#space_traders_rust-waypoint-shipyard)
* [`space_traders_rust ship`↴](#space_traders_rust-ship)
* [`space_traders_rust ship navigate`↴](#space_traders_rust-ship-navigate)
* [`space_traders_rust ship navigate status`↴](#space_traders_rust-ship-navigate-status)
* [`space_traders_rust ship navigate waypoint`↴](#space_traders_rust-ship-navigate-waypoint)
* [`space_traders_rust ship cargo`↴](#space_traders_rust-ship-cargo)
* [`space_traders_rust ship cargo status`↴](#space_traders_rust-ship-cargo-status)
* [`space_traders_rust ship cargo sell`↴](#space_traders_rust-ship-cargo-sell)
* [`space_traders_rust ship list`↴](#space_traders_rust-ship-list)
* [`space_traders_rust ship purchase`↴](#space_traders_rust-ship-purchase)
* [`space_traders_rust ship orbit`↴](#space_traders_rust-ship-orbit)
* [`space_traders_rust ship dock`↴](#space_traders_rust-ship-dock)
* [`space_traders_rust ship status`↴](#space_traders_rust-ship-status)
* [`space_traders_rust ship refuel`↴](#space_traders_rust-ship-refuel)
* [`space_traders_rust ship extract`↴](#space_traders_rust-ship-extract)
* [`space_traders_rust ship survey`↴](#space_traders_rust-ship-survey)

## `space_traders_rust`

**Usage:** `space_traders_rust [COMMAND]`

###### **Subcommands:**

* `generate-doc` — 
* `status` — 
* `register` — Register a new player (NOTE: will override your current user's token)
* `contract` — 
* `who-am-i` — Show current player's details
* `waypoint` — 
* `ship` — 



## `space_traders_rust generate-doc`

**Usage:** `space_traders_rust generate-doc`



## `space_traders_rust status`

**Usage:** `space_traders_rust status`



## `space_traders_rust register`

Register a new player (NOTE: will override your current user's token)

**Usage:** `space_traders_rust register [OPTIONS] --username <USERNAME>`

###### **Options:**

* `-u`, `--username <USERNAME>`
* `-f`, `--faction <FACTION>`

  Default value: `COSMIC`



## `space_traders_rust contract`

**Usage:** `space_traders_rust contract
       contract <COMMAND>`

###### **Subcommands:**

* `list` — 
* `accept` — 
* `deliver` — 
* `fulfill` — 



## `space_traders_rust contract list`

**Usage:** `space_traders_rust contract list`



## `space_traders_rust contract accept`

**Usage:** `space_traders_rust contract accept --contract-id <CONTRACT_ID>`

###### **Options:**

* `-c`, `--contract-id <CONTRACT_ID>`



## `space_traders_rust contract deliver`

**Usage:** `space_traders_rust contract deliver --ship-symbol <SHIP_SYMBOL> --contract-id <CONTRACT_ID> --trade-symbol <TRADE_SYMBOL> --units <UNITS>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`
* `-c`, `--contract-id <CONTRACT_ID>`
* `-t`, `--trade-symbol <TRADE_SYMBOL>`

  Possible values: `precious-stones`, `quartz-sand`, `silicon-crystals`, `ammonia-ice`, `liquid-hydrogen`, `liquid-nitrogen`, `ice-water`, `exotic-matter`, `advanced-circuitry`, `graviton-emitters`, `iron`, `iron-ore`, `copper`, `copper-ore`, `aluminum`, `aluminum-ore`, `silver`, `silver-ore`, `gold`, `gold-ore`, `platinum`, `platinum-ore`, `diamonds`, `uranite`, `uranite-ore`, `meritium`, `meritium-ore`, `hydrocarbon`, `antimatter`, `fertilizers`, `fabrics`, `food`, `jewelry`, `machinery`, `firearms`, `assault-rifles`, `military-equipment`, `explosives`, `lab-instruments`, `ammunition`, `electronics`, `ship-plating`, `equipment`, `fuel`, `medicine`, `drugs`, `clothing`, `microprocessors`, `plastics`, `polynucleotides`, `biocomposites`, `nanobots`, `ai-mainframes`, `quantum-drives`, `robotic-drones`, `cyber-implants`, `gene-therapeutics`, `neural-chips`, `mood-regulators`, `viral-agents`, `micro-fusion-generators`, `supergrains`, `laser-rifles`, `holographics`, `ship-salvage`, `relic-tech`, `novel-lifeforms`, `botanical-specimens`, `cultural-artifacts`, `reactor-solar-i`, `reactor-fusion-i`, `reactor-fission-i`, `reactor-chemical-i`, `reactor-antimatter-i`, `engine-impulse-drive-i`, `engine-ion-drive-i`, `engine-ion-drive-ii`, `engine-hyper-drive-i`, `module-mineral-processor-i`, `module-cargo-hold-i`, `module-crew-quarters-i`, `module-envoy-quarters-i`, `module-passenger-cabin-i`, `module-micro-refinery-i`, `module-ore-refinery-i`, `module-fuel-refinery-i`, `module-science-lab-i`, `module-jump-drive-i`, `module-jump-drive-ii`, `module-jump-drive-iii`, `module-warp-drive-i`, `module-warp-drive-ii`, `module-warp-drive-iii`, `module-shield-generator-i`, `module-shield-generator-ii`, `mount-gas-siphon-i`, `mount-gas-siphon-ii`, `mount-gas-siphon-iii`, `mount-surveyor-i`, `mount-surveyor-ii`, `mount-surveyor-iii`, `mount-sensor-array-i`, `mount-sensor-array-ii`, `mount-sensor-array-iii`, `mount-mining-laser-i`, `mount-mining-laser-ii`, `mount-mining-laser-iii`, `mount-laser-cannon-i`, `mount-missile-launcher-i`, `mount-turret-i`

* `-u`, `--units <UNITS>`



## `space_traders_rust contract fulfill`

**Usage:** `space_traders_rust contract fulfill --contract-id <CONTRACT_ID>`

###### **Options:**

* `-c`, `--contract-id <CONTRACT_ID>`



## `space_traders_rust who-am-i`

Show current player's details

**Usage:** `space_traders_rust who-am-i`



## `space_traders_rust waypoint`

**Usage:** `space_traders_rust waypoint
       waypoint <COMMAND>`

###### **Subcommands:**

* `get` — 
* `list` — 
* `market` — 
* `shipyard` — 



## `space_traders_rust waypoint get`

**Usage:** `space_traders_rust waypoint get --waypoint-symbol <WAYPOINT_SYMBOL>`

###### **Options:**

* `-w`, `--waypoint-symbol <WAYPOINT_SYMBOL>`



## `space_traders_rust waypoint list`

**Usage:** `space_traders_rust waypoint list [OPTIONS]`

###### **Options:**

* `-t`, `--filter-by-trait <FILTER_BY_TRAIT>`

  Possible values: `uncharted`, `marketplace`, `shipyard`, `outpost`, `overcrowded`, `corrupt`, `bureaucratic`, `industrial`, `jovian`, `rocky`, `volcanic`, `frozen`, `swamp`, `barren`, `temperate`, `jungle`, `ocean`, `stripped`, `toxic-atmosphere`, `scattered-settlements`, `sprawling-cities`, `mega-structures`, `high-tech`, `trading-hub`, `black-market`, `research-facility`, `military-base`, `surveillance-outpost`, `exploration-outpost`, `mineral-deposits`, `common-metal-deposits`, `precious-metal-deposits`, `rare-metal-deposits`, `methane-pools`, `ice-crystals`, `explosive-gases`, `strong-magnetosphere`, `vibrant-auroras`, `salt-flats`, `canyons`, `perpetual-daylight`, `perpetual-overcast`, `dry-seabeds`, `magma-seas`, `supervolcanoes`, `ash-clouds`, `vast-ruins`, `mutated-flora`, `terraformed`, `extreme-temperatures`, `extreme-pressure`, `diverse-life`, `scarce-life`, `fossils`, `weak-gravity`, `strong-gravity`, `crushing-gravity`, `corrosive-atmosphere`, `breathable-atmosphere`

* `-w`, `--filter-by-type <FILTER_BY_TYPE>`

  Possible values: `planet`, `gas-giant`, `moon`, `orbital-station`, `jump-gate`, `asteroid-field`, `nebula`, `debris-field`, `gravity-well`




## `space_traders_rust waypoint market`

**Usage:** `space_traders_rust waypoint market --waypoint-symbol <WAYPOINT_SYMBOL>`

###### **Options:**

* `-w`, `--waypoint-symbol <WAYPOINT_SYMBOL>`



## `space_traders_rust waypoint shipyard`

**Usage:** `space_traders_rust waypoint shipyard --waypoint-symbol <WAYPOINT_SYMBOL>`

###### **Options:**

* `-w`, `--waypoint-symbol <WAYPOINT_SYMBOL>`



## `space_traders_rust ship`

**Usage:** `space_traders_rust ship
       ship <COMMAND>`

###### **Subcommands:**

* `navigate` — 
* `cargo` — 
* `list` — 
* `purchase` — 
* `orbit` — 
* `dock` — 
* `status` — 
* `refuel` — 
* `extract` — 
* `survey` — 



## `space_traders_rust ship navigate`

**Usage:** `space_traders_rust ship navigate <COMMAND>`

###### **Subcommands:**

* `status` — 
* `waypoint` — 



## `space_traders_rust ship navigate status`

**Usage:** `space_traders_rust ship navigate status --ship-symbol <SHIP_SYMBOL>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`



## `space_traders_rust ship navigate waypoint`

**Usage:** `space_traders_rust ship navigate waypoint --ship-symbol <SHIP_SYMBOL> --waypoint-symbol <WAYPOINT_SYMBOL>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`
* `-w`, `--waypoint-symbol <WAYPOINT_SYMBOL>`



## `space_traders_rust ship cargo`

**Usage:** `space_traders_rust ship cargo <COMMAND>`

###### **Subcommands:**

* `status` — 
* `sell` — 



## `space_traders_rust ship cargo status`

**Usage:** `space_traders_rust ship cargo status --ship-symbol <SHIP_SYMBOL>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`



## `space_traders_rust ship cargo sell`

**Usage:** `space_traders_rust ship cargo sell --ship-symbol <SHIP_SYMBOL> --good-symbol <GOOD_SYMBOL> --units <UNITS>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`
* `-g`, `--good-symbol <GOOD_SYMBOL>`

  Possible values: `precious-stones`, `quartz-sand`, `silicon-crystals`, `ammonia-ice`, `liquid-hydrogen`, `liquid-nitrogen`, `ice-water`, `exotic-matter`, `advanced-circuitry`, `graviton-emitters`, `iron`, `iron-ore`, `copper`, `copper-ore`, `aluminum`, `aluminum-ore`, `silver`, `silver-ore`, `gold`, `gold-ore`, `platinum`, `platinum-ore`, `diamonds`, `uranite`, `uranite-ore`, `meritium`, `meritium-ore`, `hydrocarbon`, `antimatter`, `fertilizers`, `fabrics`, `food`, `jewelry`, `machinery`, `firearms`, `assault-rifles`, `military-equipment`, `explosives`, `lab-instruments`, `ammunition`, `electronics`, `ship-plating`, `equipment`, `fuel`, `medicine`, `drugs`, `clothing`, `microprocessors`, `plastics`, `polynucleotides`, `biocomposites`, `nanobots`, `ai-mainframes`, `quantum-drives`, `robotic-drones`, `cyber-implants`, `gene-therapeutics`, `neural-chips`, `mood-regulators`, `viral-agents`, `micro-fusion-generators`, `supergrains`, `laser-rifles`, `holographics`, `ship-salvage`, `relic-tech`, `novel-lifeforms`, `botanical-specimens`, `cultural-artifacts`, `reactor-solar-i`, `reactor-fusion-i`, `reactor-fission-i`, `reactor-chemical-i`, `reactor-antimatter-i`, `engine-impulse-drive-i`, `engine-ion-drive-i`, `engine-ion-drive-ii`, `engine-hyper-drive-i`, `module-mineral-processor-i`, `module-cargo-hold-i`, `module-crew-quarters-i`, `module-envoy-quarters-i`, `module-passenger-cabin-i`, `module-micro-refinery-i`, `module-ore-refinery-i`, `module-fuel-refinery-i`, `module-science-lab-i`, `module-jump-drive-i`, `module-jump-drive-ii`, `module-jump-drive-iii`, `module-warp-drive-i`, `module-warp-drive-ii`, `module-warp-drive-iii`, `module-shield-generator-i`, `module-shield-generator-ii`, `mount-gas-siphon-i`, `mount-gas-siphon-ii`, `mount-gas-siphon-iii`, `mount-surveyor-i`, `mount-surveyor-ii`, `mount-surveyor-iii`, `mount-sensor-array-i`, `mount-sensor-array-ii`, `mount-sensor-array-iii`, `mount-mining-laser-i`, `mount-mining-laser-ii`, `mount-mining-laser-iii`, `mount-laser-cannon-i`, `mount-missile-launcher-i`, `mount-turret-i`

* `-u`, `--units <UNITS>`



## `space_traders_rust ship list`

**Usage:** `space_traders_rust ship list`



## `space_traders_rust ship purchase`

**Usage:** `space_traders_rust ship purchase --ship-type <SHIP_TYPE> --waypoint-symbol <WAYPOINT_SYMBOL>`

###### **Options:**

* `-s`, `--ship-type <SHIP_TYPE>`

  Possible values: `ship-probe`, `ship-mining-drone`, `ship-interceptor`, `ship-light-hauler`, `ship-command-frigate`, `ship-explorer`, `ship-heavy-freighter`, `ship-light-shuttle`, `ship-ore-hound`, `ship-refining-freighter`

* `-w`, `--waypoint-symbol <WAYPOINT_SYMBOL>`



## `space_traders_rust ship orbit`

**Usage:** `space_traders_rust ship orbit --ship-symbol <SHIP_SYMBOL>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`



## `space_traders_rust ship dock`

**Usage:** `space_traders_rust ship dock --ship-symbol <SHIP_SYMBOL>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`



## `space_traders_rust ship status`

**Usage:** `space_traders_rust ship status --ship-symbol <SHIP_SYMBOL>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`



## `space_traders_rust ship refuel`

**Usage:** `space_traders_rust ship refuel [OPTIONS] --ship-symbol <SHIP_SYMBOL>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`
* `-u`, `--units <UNITS>`



## `space_traders_rust ship extract`

**Usage:** `space_traders_rust ship extract --ship-symbol <SHIP_SYMBOL>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`



## `space_traders_rust ship survey`

**Usage:** `space_traders_rust ship survey --ship-symbol <SHIP_SYMBOL>`

###### **Options:**

* `-s`, `--ship-symbol <SHIP_SYMBOL>`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
