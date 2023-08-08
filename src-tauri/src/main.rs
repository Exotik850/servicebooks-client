// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
use quick_oxibooks::actions::QBQuery;
use quick_oxibooks::error::APIError;
use quick_oxibooks::types::Customer;
use quick_oxibooks::{client::Quickbooks, Authorized, Cache, Environment};
use service_poxi::{ClaimHandler, Retreive, RetrievedClaim, Submit};
use tauri::{generate_context, Manager, State, WindowEvent};

struct QBState(Quickbooks<Authorized>);
struct SPRetrieveState(ClaimHandler<Retreive>);
struct SPSubmitState(ClaimHandler<Submit>);

/// Access a string property, default to empty string if missing
macro_rules! get_str {
    ($obj:expr, $key:expr) => {
        $obj.get($key).map_or("", |v| v.as_str().unwrap())
    };
}

#[tauri::command]
async fn submit_claim(
    claim: serde_json::Value,
    qb: State<'_, QBState>,
) -> Result<Customer, APIError> {
    let first_name = get_str!(claim, "customer_first_name");
    let last_name = get_str!(claim, "customer_last_name");
    println!("{first_name} {last_name}");
    let st = format!("where GivenName = '{first_name}' and FamilyName = '{last_name}'");
    let custs = Customer::query_single(&qb.0, &st).await?;
    println!("{custs:?}");
    Ok(custs)
}

#[tauri::command]
async fn get_claim(
    claim_number: String,
    retreive_handler: State<'_, SPRetrieveState>,
) -> Result<RetrievedClaim, String> {
    let retreive_handler = &retreive_handler.0;
    let out = retreive_handler
        .get_claim(&claim_number)
        .await
        .map_err(|e| e.to_string())?
        .claims
        .pop()
        .ok_or(format!("No claim found for claim number: {claim_number}"));
    dbg!(&out);
    out
}

#[tokio::main]
async fn main() {
    let qb = Quickbooks::new_from_env("4620816365257778210", Environment::SANDBOX)
        .await
        .unwrap();

    tauri::Builder::default()
        .manage(QBState(qb))
        .manage(SPRetrieveState(ClaimHandler::<Retreive>::new()))
        .manage(SPSubmitState(ClaimHandler::<Submit>::new()))
        .invoke_handler(tauri::generate_handler![submit_claim, get_claim])
        .on_window_event(|event| {
            let window = event.window();
            let state: State<QBState> = window.state();

            match event.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    state.0.cleanup();
                    window.close().unwrap();
                }
                WindowEvent::Destroyed => {
                    state.0.cleanup();
                }
                _ => (),
            }
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
