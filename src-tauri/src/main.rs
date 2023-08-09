// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
use quick_oxibooks::actions::QBQuery;
use quick_oxibooks::error::APIError;
use quick_oxibooks::types::Customer;
use quick_oxibooks::{client::Quickbooks, Authorized, Cache, Environment};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};
use service_poxi::{ClaimHandler, Retreive, RetrievedClaim, Submit};
use tauri::{generate_context, Manager, State, WindowEvent, AppHandle};

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

#[tauri::command]
async fn show_main(app_handle: AppHandle) {
    app_handle.get_window("main").unwrap().show().unwrap();
}

#[tokio::main]
async fn main() {
    env_logger::init();

    tauri::Builder::default()
        // .plugin(tauri_plugin_log::Builder::default().build())
        .manage(QBState(
            Quickbooks::new_from_env("4620816365257778210", Environment::SANDBOX)
                .await
                .unwrap(),
        ))
        .manage(SPRetrieveState(ClaimHandler::<Retreive>::new()))
        .manage(SPSubmitState(ClaimHandler::<Submit>::new()))
        .invoke_handler(tauri::generate_handler![submit_claim, get_claim, show_main])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            Ok(())
        })
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
