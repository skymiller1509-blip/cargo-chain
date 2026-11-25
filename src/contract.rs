use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::msg::{
    Checkpoint, CheckpointsResponse, ExecuteMsg, InstantiateMsg, ListShipmentsResponse, MigrateMsg,
    QueryMsg, Shipment, ShipmentResponse, VerifyResponse,
};
use crate::state::{Config, CHECKPOINTS, CONFIG, SHIPMENTS, SHIPMENT_IDS};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// A shipment counts as verified once its trail has at least this many
/// checkpoints (origin + at least one more hop). This is the whole rule:
/// verifyShipment() checks the checkpoint count, nothing else.
const MIN_CHECKPOINTS: u32 = 2;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin = match msg.admin {
        Some(addr) => deps.api.addr_validate(&addr)?,
        None => info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(
        deps.storage,
        &Config {
            admin: admin.clone(),
        },
    )?;
    SHIPMENT_IDS.save(deps.storage, &Vec::<String>::new())?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", admin))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterShipment {
            shipment_id,
            origin,
            destination,
            description,
            shipper,
        } => execute_register_shipment(
            deps,
            env,
            info,
            shipment_id,
            origin,
            destination,
            description,
            shipper,
        ),
        ExecuteMsg::AddCheckpoint {
            shipment_id,
            location,
            status,
            note,
        } => execute_add_checkpoint(deps, env, info, shipment_id, location, status, note),
    }
}

#[allow(clippy::too_many_arguments)]
fn execute_register_shipment(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    shipment_id: String,
    origin: String,
    destination: String,
    description: String,
    shipper: String,
) -> Result<Response, ContractError> {
    if shipment_id.trim().is_empty() {
        return Err(ContractError::EmptyShipmentId {});
    }
    if SHIPMENTS.has(deps.storage, &shipment_id) {
        return Err(ContractError::ShipmentAlreadyExists { shipment_id });
    }

    let shipment = Shipment {
        id: shipment_id.clone(),
        origin,
        destination,
        description,
        shipper,
        registrant: info.sender.clone(),
        created_at: env.block.time,
    };
    SHIPMENTS.save(deps.storage, &shipment_id, &shipment)?;
    CHECKPOINTS.save(deps.storage, &shipment_id, &Vec::<Checkpoint>::new())?;
    SHIPMENT_IDS.update(deps.storage, |mut ids| -> StdResult<Vec<String>> {
        ids.push(shipment_id.clone());
        Ok(ids)
    })?;

    Ok(Response::new()
        .add_attribute("method", "register_shipment")
        .add_attribute("shipment_id", shipment_id)
        .add_attribute("registrant", info.sender))
}

fn execute_add_checkpoint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    shipment_id: String,
    location: String,
    status: String,
    note: String,
) -> Result<Response, ContractError> {
    let shipment =
        SHIPMENTS
            .may_load(deps.storage, &shipment_id)?
            .ok_or(ContractError::ShipmentNotFound {
                shipment_id: shipment_id.clone(),
            })?;

    let config = CONFIG.load(deps.storage)?;
    if info.sender != shipment.registrant && info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let checkpoint = Checkpoint {
        location,
        status,
        note,
        recorded_by: info.sender.clone(),
        recorded_at: env.block.time,
    };
    let mut checkpoints = CHECKPOINTS.load(deps.storage, &shipment_id)?;
    checkpoints.push(checkpoint);
    let count = checkpoints.len() as u32;
    CHECKPOINTS.save(deps.storage, &shipment_id, &checkpoints)?;

    Ok(Response::new()
        .add_attribute("method", "add_checkpoint")
        .add_attribute("shipment_id", shipment_id)
        .add_attribute("recorded_by", info.sender)
        .add_attribute("checkpoint_count", count.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetShipment { shipment_id } => {
            to_json_binary(&query_shipment(deps, shipment_id)?)
        }
        QueryMsg::GetCheckpoints { shipment_id } => {
            to_json_binary(&query_checkpoints(deps, shipment_id)?)
        }
        QueryMsg::VerifyShipment { shipment_id } => {
            to_json_binary(&query_verify(deps, shipment_id)?)
        }
        QueryMsg::ListShipments {} => to_json_binary(&query_list_shipments(deps)?),
    }
}

fn query_shipment(deps: Deps, shipment_id: String) -> StdResult<ShipmentResponse> {
    let shipment = SHIPMENTS.load(deps.storage, &shipment_id)?;
    Ok(ShipmentResponse { shipment })
}

fn query_checkpoints(deps: Deps, shipment_id: String) -> StdResult<CheckpointsResponse> {
    if !SHIPMENTS.has(deps.storage, &shipment_id) {
        return Err(cosmwasm_std::StdError::not_found(format!(
            "Shipment {shipment_id} not found"
        )));
    }
    let checkpoints = CHECKPOINTS.load(deps.storage, &shipment_id)?;
    Ok(CheckpointsResponse {
        shipment_id,
        checkpoints,
    })
}

fn query_verify(deps: Deps, shipment_id: String) -> StdResult<VerifyResponse> {
    if !SHIPMENTS.has(deps.storage, &shipment_id) {
        return Err(cosmwasm_std::StdError::not_found(format!(
            "Shipment {shipment_id} not found"
        )));
    }
    let checkpoints = CHECKPOINTS.load(deps.storage, &shipment_id)?;
    let count = checkpoints.len() as u32;
    let verified = count >= MIN_CHECKPOINTS;
    let reason = if verified {
        format!("Shipment has {count} checkpoints (minimum {MIN_CHECKPOINTS}); trail is verifiable")
    } else {
        format!(
            "Shipment has only {count} checkpoint(s); needs at least {MIN_CHECKPOINTS} to verify"
        )
    };
    Ok(VerifyResponse {
        shipment_id,
        verified,
        checkpoint_count: count,
        reason,
    })
}

fn query_list_shipments(deps: Deps) -> StdResult<ListShipmentsResponse> {
    let shipment_ids = SHIPMENT_IDS.load(deps.storage)?;
    Ok(ListShipmentsResponse { shipment_ids })
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = get_contract_version(deps.storage)?;
    if ver.contract != CONTRACT_NAME {
        return Err(ContractError::InvalidContract {});
    }
    if !ver.version.starts_with("0.1.") {
        return Err(ContractError::InvalidMigrationVersion {
            current: ver.version,
        });
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "migrate")
        .add_attribute("from_version", ver.version)
        .add_attribute("to_version", CONTRACT_VERSION))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, Addr};

    fn do_instantiate(deps: DepsMut) {
        let msg = InstantiateMsg { admin: None };
        let info = mock_info("admin", &coins(0, "umec"));
        instantiate(deps, mock_env(), info, msg).unwrap();
    }

    fn register(deps: DepsMut, sender: &str, id: &str) {
        let msg = ExecuteMsg::RegisterShipment {
            shipment_id: id.to_string(),
            origin: "Lagos, Nigeria".to_string(),
            destination: "Rotterdam, Netherlands".to_string(),
            description: "1000 bags of cocoa beans".to_string(),
            shipper: "Butterfly Exports Ltd".to_string(),
        };
        let info = mock_info(sender, &coins(0, "umec"));
        execute(deps, mock_env(), info, msg).unwrap();
    }

    fn checkpoint(deps: DepsMut, sender: &str, id: &str, location: &str) {
        let msg = ExecuteMsg::AddCheckpoint {
            shipment_id: id.to_string(),
            location: location.to_string(),
            status: "in_transit".to_string(),
            note: "Checked in".to_string(),
        };
        let info = mock_info(sender, &coins(0, "umec"));
        execute(deps, mock_env(), info, msg).unwrap();
    }

    fn verify(deps: Deps, id: &str) -> VerifyResponse {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::VerifyShipment {
                shipment_id: id.to_string(),
            },
        )
        .unwrap();
        from_json(&res).unwrap()
    }

    #[test]
    fn register_and_read_shipment() {
        let mut deps = mock_dependencies();
        do_instantiate(deps.as_mut());
        register(deps.as_mut(), "shipper1", "CC-2026-001");

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetShipment {
                shipment_id: "CC-2026-001".to_string(),
            },
        )
        .unwrap();
        let resp: ShipmentResponse = from_json(&res).unwrap();
        assert_eq!(resp.shipment.origin, "Lagos, Nigeria");
        assert_eq!(resp.shipment.destination, "Rotterdam, Netherlands");
        assert_eq!(resp.shipment.shipper, "Butterfly Exports Ltd");
        assert_eq!(resp.shipment.registrant, Addr::unchecked("shipper1"));
    }

    #[test]
    fn duplicate_registration_fails() {
        let mut deps = mock_dependencies();
        do_instantiate(deps.as_mut());
        register(deps.as_mut(), "shipper1", "CC-2026-001");

        let msg = ExecuteMsg::RegisterShipment {
            shipment_id: "CC-2026-001".to_string(),
            origin: "Lagos".to_string(),
            destination: "Rotterdam".to_string(),
            description: "dup".to_string(),
            shipper: "X".to_string(),
        };
        let err = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("shipper1", &coins(0, "umec")),
            msg,
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::ShipmentAlreadyExists { .. }));
    }

    #[test]
    fn empty_shipment_id_fails() {
        let mut deps = mock_dependencies();
        do_instantiate(deps.as_mut());
        let msg = ExecuteMsg::RegisterShipment {
            shipment_id: "   ".to_string(),
            origin: "Lagos".to_string(),
            destination: "Rotterdam".to_string(),
            description: "x".to_string(),
            shipper: "X".to_string(),
        };
        let err = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("shipper1", &coins(0, "umec")),
            msg,
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::EmptyShipmentId {}));
    }

    #[test]
    fn checkpoints_recorded_in_order() {
        let mut deps = mock_dependencies();
        do_instantiate(deps.as_mut());
        register(deps.as_mut(), "shipper1", "CC-2026-001");
        checkpoint(
            deps.as_mut(),
            "shipper1",
            "CC-2026-001",
            "Apapa Port, Lagos",
        );
        checkpoint(
            deps.as_mut(),
            "shipper1",
            "CC-2026-001",
            "Atlantic crossing",
        );
        checkpoint(deps.as_mut(), "shipper1", "CC-2026-001", "Rotterdam Port");

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetCheckpoints {
                shipment_id: "CC-2026-001".to_string(),
            },
        )
        .unwrap();
        let resp: CheckpointsResponse = from_json(&res).unwrap();
        assert_eq!(resp.checkpoints.len(), 3);
        assert_eq!(resp.checkpoints[0].location, "Apapa Port, Lagos");
        assert_eq!(resp.checkpoints[2].location, "Rotterdam Port");
        assert_eq!(resp.checkpoints[0].recorded_by, Addr::unchecked("shipper1"));
    }

    #[test]
    fn verify_needs_two_checkpoints() {
        let mut deps = mock_dependencies();
        do_instantiate(deps.as_mut());
        register(deps.as_mut(), "shipper1", "CC-2026-001");

        let v = verify(deps.as_ref(), "CC-2026-001");
        assert!(!v.verified);
        assert_eq!(v.checkpoint_count, 0);

        checkpoint(
            deps.as_mut(),
            "shipper1",
            "CC-2026-001",
            "Apapa Port, Lagos",
        );
        let v = verify(deps.as_ref(), "CC-2026-001");
        assert!(!v.verified);
        assert_eq!(v.checkpoint_count, 1);

        checkpoint(
            deps.as_mut(),
            "shipper1",
            "CC-2026-001",
            "Atlantic crossing",
        );
        let v = verify(deps.as_ref(), "CC-2026-001");
        assert!(v.verified);
        assert_eq!(v.checkpoint_count, 2);

        checkpoint(deps.as_mut(), "shipper1", "CC-2026-001", "Rotterdam Port");
        let v = verify(deps.as_ref(), "CC-2026-001");
        assert!(v.verified);
        assert_eq!(v.checkpoint_count, 3);
    }

    #[test]
    fn only_registrant_or_admin_can_checkpoint() {
        let mut deps = mock_dependencies();
        do_instantiate(deps.as_mut()); // admin = "admin"
        register(deps.as_mut(), "shipper1", "CC-2026-001");

        // Stranger cannot add a checkpoint.
        let msg = ExecuteMsg::AddCheckpoint {
            shipment_id: "CC-2026-001".to_string(),
            location: "Fake Port".to_string(),
            status: "in_transit".to_string(),
            note: "forged".to_string(),
        };
        let err = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("stranger", &coins(0, "umec")),
            msg,
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::Unauthorized {}));

        // Admin can (port authority case).
        checkpoint(deps.as_mut(), "admin", "CC-2026-001", "Apapa Port, Lagos");
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetCheckpoints {
                shipment_id: "CC-2026-001".to_string(),
            },
        )
        .unwrap();
        let resp: CheckpointsResponse = from_json(&res).unwrap();
        assert_eq!(resp.checkpoints.len(), 1);
    }

    #[test]
    fn checkpoint_on_unknown_shipment_fails() {
        let mut deps = mock_dependencies();
        do_instantiate(deps.as_mut());
        let msg = ExecuteMsg::AddCheckpoint {
            shipment_id: "NOPE".to_string(),
            location: "Nowhere".to_string(),
            status: "lost".to_string(),
            note: "x".to_string(),
        };
        let err = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("admin", &coins(0, "umec")),
            msg,
        )
        .unwrap_err();
        assert!(matches!(err, ContractError::ShipmentNotFound { .. }));
    }

    #[test]
    fn list_shipments_in_registration_order() {
        let mut deps = mock_dependencies();
        do_instantiate(deps.as_mut());
        register(deps.as_mut(), "shipper1", "CC-2026-001");
        register(deps.as_mut(), "shipper1", "CC-2026-002");

        let res = query(deps.as_ref(), mock_env(), QueryMsg::ListShipments {}).unwrap();
        let resp: ListShipmentsResponse = from_json(&res).unwrap();
        assert_eq!(resp.shipment_ids, vec!["CC-2026-001", "CC-2026-002"]);
    }
}
