// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod blocking;
mod models;
mod util;
use blocking::Block;
use models::*;
use quick_oxibooks::{
    actions::{QBCreate, QBQuery},
    error::APIError,
    types::{Customer, Invoice},
    {client::Quickbooks, Authorized, Environment},
};
use service_poxi::{ClaimHandler, Retreive, Submit};
use tauri::{generate_context, AppHandle, Manager, State, WindowEvent};
use util::*;

#[cfg(any(windows, target_os = "macos"))]
use window_shadows::set_shadow;

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
async fn submit_claim(claim: serde_json::Value, qb: State<'_, QBState>) -> Result<(), APIError> {
    let input_invoice: InputInvoice = claim.try_into()?;
    println!("{first_name} {last_name}");
    let st = format!("where DisplayName = '{first_name} {last_name}'");
    let cust = Customer::query_single(&qb.0, &st).await?;

    // println!("Before: {inv}");

    let next = generate_claim_number(&qb.0).await?;
    let inv = default_qb_invoice(cust.into(), &[], next);
    let inv = inv.create(&qb.0).await?;
    println!("After: {inv}");
    // dbg!(next);

    Ok(())
}

#[tauri::command]
async fn get_claim(
    claim_number: String,
    get_qb: bool,
    get_sb: bool,
    app_handle: AppHandle,
) -> Result<HAInvoice, String> {
    let retreive_handler: State<SPRetrieveState> = app_handle.state();
    let qb: State<QBState> = app_handle.state();

    let sb_claim = match get_sb {
        true => Some(
            retreive_handler
                .0
                .get_claim(&claim_number)
                .await
                .map_err(|e| e.to_string())?
                .claims
                .pop()
                .ok_or(format!("No claim found for claim number: {claim_number}"))?
                .into(),
        ),
        false => None,
    };

    let qb_invoice = match get_qb {
        true => Some(
            Invoice::query_single(&qb.0, &format!("where DocNumber = '{claim_number}'"))
                .await
                .map_err(|e| e.to_string())?,
        ),
        false => None,
    };

    qb_invoice.as_ref().is_some_and(|inv| {
        println!("{inv}");
        true
    });

    Ok(HAInvoice {
        qb_invoice,
        sb_claim,
    })
}

#[tauri::command]
async fn show_main(app_handle: AppHandle) {
    app_handle.get_window("main").unwrap().show().unwrap();
}

#[tokio::main]
async fn main() {
    env_logger::init();

    tauri::Builder::default()
        .manage(QBState(
            // Quickbooks::new_from_env("4620816365257778210", Environment::SANDBOX)
            Quickbooks::new_from_env("9130347246064456", Environment::PRODUCTION, "peepeepoopoo")
                .await
                .unwrap(),
        ))
        .manage(SPRetrieveState(ClaimHandler::<Retreive>::new()))
        .manage(SPSubmitState(ClaimHandler::<Submit>::new()))
        .invoke_handler(tauri::generate_handler![submit_claim, get_claim, show_main])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();
            Ok(())
        })
        .on_window_event(|event| {
            let window = event.window();
            let state: State<QBState> = window.state();

            match event.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    state.0.cleanup().wait();
                    window.close().unwrap();
                }
                WindowEvent::Destroyed => {
                    state.0.cleanup().wait();
                }
                _ => (),
            }
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
