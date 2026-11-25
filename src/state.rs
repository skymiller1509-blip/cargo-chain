use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::{Checkpoint, Shipment};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// shipment_id -> Shipment
pub const SHIPMENTS: Map<&str, Shipment> = Map::new("shipments");

/// shipment_id -> ordered list of checkpoints
pub const CHECKPOINTS: Map<&str, Vec<Checkpoint>> = Map::new("checkpoints");

/// All shipment ids, in registration order.
pub const SHIPMENT_IDS: Item<Vec<String>> = Item::new("shipment_ids");
