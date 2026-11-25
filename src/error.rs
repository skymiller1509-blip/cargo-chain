use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Shipment id must not be empty")]
    EmptyShipmentId {},

    #[error("Shipment {shipment_id} already exists")]
    ShipmentAlreadyExists { shipment_id: String },

    #[error("Shipment {shipment_id} not found")]
    ShipmentNotFound { shipment_id: String },

    #[error("Only the shipment registrant or the contract admin may add checkpoints")]
    Unauthorized {},

    #[error("Cannot migrate from a different contract")]
    InvalidContract {},

    #[error("Cannot migrate from version {current}")]
    InvalidMigrationVersion { current: String },
}
