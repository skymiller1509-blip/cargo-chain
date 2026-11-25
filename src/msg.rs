use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Timestamp};

/// A registered shipment: one consignment moving from origin to destination.
#[cw_serde]
pub struct Shipment {
    pub id: String,
    pub origin: String,
    pub destination: String,
    pub description: String,
    /// Human-readable shipper name (e.g. the exporting company).
    pub shipper: String,
    /// On-chain address that registered the shipment. Only the registrant
    /// (or the contract admin) may add checkpoints to it.
    pub registrant: Addr,
    pub created_at: Timestamp,
}

/// One on-chain checkpoint: a port or handler recording that the shipment
/// passed through their hands.
#[cw_serde]
pub struct Checkpoint {
    pub location: String,
    pub status: String,
    pub note: String,
    pub recorded_by: Addr,
    pub recorded_at: Timestamp,
}

#[cw_serde]
pub struct InstantiateMsg {
    /// Optional admin address. Defaults to the sender. The admin may add
    /// checkpoints to any shipment (e.g. a port authority).
    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Register a new shipment. The shipment id must be unique.
    RegisterShipment {
        shipment_id: String,
        origin: String,
        destination: String,
        description: String,
        shipper: String,
    },
    /// Record a checkpoint for an existing shipment. Only the shipment's
    /// registrant or the contract admin may call this.
    AddCheckpoint {
        shipment_id: String,
        location: String,
        status: String,
        note: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Full details of one shipment.
    #[returns(ShipmentResponse)]
    GetShipment { shipment_id: String },
    /// Every checkpoint recorded for a shipment, in the order recorded.
    #[returns(CheckpointsResponse)]
    GetCheckpoints { shipment_id: String },
    /// Is this shipment's trail verifiable? A shipment is verified when it
    /// has at least 2 checkpoints (origin + at least one more hop).
    #[returns(VerifyResponse)]
    VerifyShipment { shipment_id: String },
    /// All registered shipment ids, in registration order.
    #[returns(ListShipmentsResponse)]
    ListShipments {},
}

#[cw_serde]
pub struct ShipmentResponse {
    pub shipment: Shipment,
}

#[cw_serde]
pub struct CheckpointsResponse {
    pub shipment_id: String,
    pub checkpoints: Vec<Checkpoint>,
}

#[cw_serde]
pub struct VerifyResponse {
    pub shipment_id: String,
    pub verified: bool,
    pub checkpoint_count: u32,
    pub reason: String,
}

#[cw_serde]
pub struct ListShipmentsResponse {
    pub shipment_ids: Vec<String>,
}

#[cw_serde]
pub struct MigrateMsg {}
