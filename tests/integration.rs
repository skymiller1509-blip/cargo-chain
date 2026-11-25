//! End-to-end demo of the CargoChain Provenance Ledger, run against the real
//! contract code with cw-multi-test (an in-memory CosmWasm chain).
//!
//! Flow: register a shipment in Lagos -> checkpoints at Apapa Port, mid-Atlantic,
//! Rotterdam Port -> verify the trail. Run with:
//!     cargo test --test integration -- --nocapture

use cosmwasm_std::{Addr, Coin, Empty};
use cw_multi_test::{App as MockApp, Contract, ContractWrapper, Executor};

use cargo_chain::contract::{execute, instantiate, query};
use cargo_chain::msg::{
    CheckpointsResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ShipmentResponse, VerifyResponse,
};

fn contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

const SHIPPER: &str = "shipper1";
const ADMIN: &str = "admin";

#[test]
fn demo_lagos_to_rotterdam() {
    let mut app = MockApp::default();
    let code_id = app.store_code(contract());

    // 1. Instantiate the ledger. Admin defaults to the sender.
    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ADMIN),
            &InstantiateMsg { admin: None },
            &[],
            "cargo-chain",
            None,
        )
        .unwrap();
    println!("ledger instantiated at {contract_addr}");

    // 2. Register the shipment: 1000 bags of cocoa, Lagos -> Rotterdam.
    app.execute_contract(
        Addr::unchecked(SHIPPER),
        contract_addr.clone(),
        &ExecuteMsg::RegisterShipment {
            shipment_id: "CC-2026-001".to_string(),
            origin: "Lagos, Nigeria".to_string(),
            destination: "Rotterdam, Netherlands".to_string(),
            description: "1000 bags of cocoa beans".to_string(),
            shipper: "Butterfly Exports Ltd".to_string(),
        },
        &[],
    )
    .unwrap();
    println!("registered CC-2026-001: Lagos -> Rotterdam (1000 bags of cocoa beans)");

    // 3. Verify before any checkpoint: NOT verified (0 checkpoints).
    let v: VerifyResponse = app
        .wrap()
        .query_wasm_smart(
            &contract_addr,
            &QueryMsg::VerifyShipment {
                shipment_id: "CC-2026-001".to_string(),
            },
        )
        .unwrap();
    println!(
        "verify after 0 checkpoints: verified={} ({})",
        v.verified, v.reason
    );
    assert!(!v.verified);

    // 4. Checkpoints, one per hop. The shipper records the first two; the
    //    admin (port authority) records the arrival.
    let hops = [
        (
            SHIPPER,
            "Apapa Port, Lagos",
            "departed",
            "Loaded onto MV Butterfly",
        ),
        (
            SHIPPER,
            "Mid-Atlantic",
            "in_transit",
            "Position report, day 9",
        ),
        (
            ADMIN,
            "Rotterdam Port",
            "arrived",
            "Unloaded, customs cleared",
        ),
    ];
    for (who, location, status, note) in hops {
        app.execute_contract(
            Addr::unchecked(who),
            contract_addr.clone(),
            &ExecuteMsg::AddCheckpoint {
                shipment_id: "CC-2026-001".to_string(),
                location: location.to_string(),
                status: status.to_string(),
                note: note.to_string(),
            },
            &[],
        )
        .unwrap();
        println!("checkpoint: {location} [{status}] by {who}");
    }

    // 5. Read the full trail back, in order.
    let trail: CheckpointsResponse = app
        .wrap()
        .query_wasm_smart(
            &contract_addr,
            &QueryMsg::GetCheckpoints {
                shipment_id: "CC-2026-001".to_string(),
            },
        )
        .unwrap();
    assert_eq!(trail.checkpoints.len(), 3);
    assert_eq!(trail.checkpoints[0].location, "Apapa Port, Lagos");
    assert_eq!(trail.checkpoints[2].location, "Rotterdam Port");
    println!(
        "trail has {} checkpoints, in order:",
        trail.checkpoints.len()
    );
    for (i, cp) in trail.checkpoints.iter().enumerate() {
        println!("  {}. {} [{}] - {}", i + 1, cp.location, cp.status, cp.note);
    }

    // 6. Verify now: 3 checkpoints >= 2, so the trail is verifiable.
    let v: VerifyResponse = app
        .wrap()
        .query_wasm_smart(
            &contract_addr,
            &QueryMsg::VerifyShipment {
                shipment_id: "CC-2026-001".to_string(),
            },
        )
        .unwrap();
    println!(
        "verify after 3 checkpoints: verified={} ({})",
        v.verified, v.reason
    );
    assert!(v.verified);
    assert_eq!(v.checkpoint_count, 3);

    // 7. A stranger cannot forge a checkpoint.
    let forged = app.execute_contract(
        Addr::unchecked("stranger"),
        contract_addr.clone(),
        &ExecuteMsg::AddCheckpoint {
            shipment_id: "CC-2026-001".to_string(),
            location: "Fake Port".to_string(),
            status: "in_transit".to_string(),
            note: "forged".to_string(),
        },
        &[],
    );
    assert!(forged.is_err());
    println!("forged checkpoint by a stranger: rejected");

    // 8. Read the shipment back.
    let s: ShipmentResponse = app
        .wrap()
        .query_wasm_smart(
            &contract_addr,
            &QueryMsg::GetShipment {
                shipment_id: "CC-2026-001".to_string(),
            },
        )
        .unwrap();
    assert_eq!(s.shipment.origin, "Lagos, Nigeria");
    assert_eq!(s.shipment.destination, "Rotterdam, Netherlands");
    println!(
        "shipment: {} -> {}, shipper {}",
        s.shipment.origin, s.shipment.destination, s.shipment.shipper
    );
    println!("DEMO OK: register -> 3 checkpoints -> verify=true");
}

#[test]
fn demo_verify_threshold() {
    // The verify rule, on its own: 0 and 1 checkpoint fail, 2 passes.
    let mut app = MockApp::default();
    let code_id = app.store_code(contract());
    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ADMIN),
            &InstantiateMsg { admin: None },
            &[],
            "cargo-chain",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked(SHIPPER),
        contract_addr.clone(),
        &ExecuteMsg::RegisterShipment {
            shipment_id: "CC-2026-002".to_string(),
            origin: "Lagos, Nigeria".to_string(),
            destination: "Rotterdam, Netherlands".to_string(),
            description: "500 bags of cocoa beans".to_string(),
            shipper: "Butterfly Exports Ltd".to_string(),
        },
        &[],
    )
    .unwrap();

    let check = |app: &MockApp| -> VerifyResponse {
        app.wrap()
            .query_wasm_smart(
                &contract_addr,
                &QueryMsg::VerifyShipment {
                    shipment_id: "CC-2026-002".to_string(),
                },
            )
            .unwrap()
    };

    assert!(!check(&app).verified); // 0 checkpoints
    app.execute_contract(
        Addr::unchecked(SHIPPER),
        contract_addr.clone(),
        &ExecuteMsg::AddCheckpoint {
            shipment_id: "CC-2026-002".to_string(),
            location: "Apapa Port, Lagos".to_string(),
            status: "departed".to_string(),
            note: "Loaded".to_string(),
        },
        &[],
    )
    .unwrap();
    assert!(!check(&app).verified); // 1 checkpoint
    app.execute_contract(
        Addr::unchecked(SHIPPER),
        contract_addr.clone(),
        &ExecuteMsg::AddCheckpoint {
            shipment_id: "CC-2026-002".to_string(),
            location: "Rotterdam Port".to_string(),
            status: "arrived".to_string(),
            note: "Unloaded".to_string(),
        },
        &[],
    )
    .unwrap();
    let v = check(&app);
    assert!(v.verified); // 2 checkpoints
    assert_eq!(v.checkpoint_count, 2);
    println!("threshold OK: 0=false, 1=false, 2=true");
}

#[allow(dead_code)]
fn _unused(_c: Coin) {}
