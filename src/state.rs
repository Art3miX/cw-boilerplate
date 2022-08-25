use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

pub struct MapCustom {
    pub custom: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const MAP_CUSTOM: Map<&str, MapCustom> = Map::new("map_custom");
