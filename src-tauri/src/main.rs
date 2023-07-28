// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
use quick_oxibooks::actions::QBQuery;
use quick_oxibooks::error::APIError;
use quick_oxibooks::{Quickbooks, Authorized, Environment};
use quick_oxibooks::types::models::Customer;
use tauri::{State, WindowEvent};

struct QBState(Quickbooks<Authorized>);

/// Access a string property, default to empty string if missing
macro_rules! get_str {
  ($obj:expr, $key:expr) => {
    $obj.get($key)
         .map_or("", |v| v.as_str().unwrap())
  };
}

#[tauri::command]
async fn submit_claim(claim: serde_json::Value, qb: State<'_, QBState>) -> Result<Customer, APIError> {
  let first_name = get_str!(claim, "customer_first_name");
  let last_name = get_str!(claim, "customer_last_name");
  println!("{first_name} {last_name}");
  let mut custs = Customer::query(&qb.0, "where GivenName = '{first_name}' and FamilyName = '{last_name}'").await?;
  println!("{custs:?}");
  Ok(custs.remove(0))
}

#[tokio::main]
async fn main() {
  let qb = Quickbooks::new_from_env("4620816365257778210", Environment::SANDBOX).await.unwrap();

  tauri::Builder::default()
    .manage(QBState(qb))
    .invoke_handler(tauri::generate_handler![submit_claim])
    // .on_window_event(|event| {
    //   match event.event() {
    //     &WindowEvent::CloseRequested { api, .. } => {
          
    //     },
    //     _ => ()
    //   }
    // })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
