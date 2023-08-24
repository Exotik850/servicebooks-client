// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod blocking;
mod models;
mod util;

use blocking::Block;
use models::*;
use quick_oxibooks::{
    actions::{QBCreate, QBQuery},
    qb_query,
    types::{Customer, Invoice, Item},
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

type Result<T> = std::result::Result<T, String>;

#[tauri::command]
async fn submit_claim(claim: InputInvoice, app_handle: AppHandle) -> Result<HAInvoice> {
    let qb: State<QBState> = app_handle.state();
    let sp_submit: State<SPSubmitState> = app_handle.state();
    let sp_retrieve: State<SPRetrieveState> = app_handle.state();
    
    let next = generate_claim_number(&qb.0)
    .await
    .map_err(|e| e.to_string())?;
    
    let phone_number: u64 = match claim.customer_phone_number.parse() {
        Ok(num) => num,
        Err(_) => return Err("Could not parse phone number, do not use anything other than numbers in the phone number field".into())
    };

    let cust = qb_query!(
        &qb.0,
        Customer | given_name = &claim.customer_first_name,
            family_name = &claim.customer_last_name
    )
    .map_err(|e| e.to_string())?;

    let mut items = vec![];

    for part in claim.parts.iter() {
        match qb_query!(&qb.0, Item | name = &part.part_number) {
            Ok(inv) => items.push(inv.into()),
            Err(_) => {
                let new_item = create_item(&part.part_number, &qb.0).await?;
                items.push(new_item.into())
            }
        }
    }

    let sp_claim = default_sp_claim(claim, phone_number, next.clone())?;

    let mut sp_claim_sub = sp_submit
        .0
        .submit_claim(sp_claim.clone())
        .await
        .map_err(|e| e.to_string())?;

    if sp_claim_sub.claims.is_empty() {
        return Err("No claims in response when submitting servicepower claim".into());
    }

    if !sp_claim_sub.messages.is_empty() {
        let msg = sp_claim_sub
            .messages
            .into_iter()
            .fold(String::new(), |mut acc, next| {
                acc += &next.message;
                acc
            });
        return Err(format!("Errors upon submitting servicepower claim {}", msg));
    }

    let sent = sp_claim_sub.claims.remove(0);

    match sent.messages {
        Some(messages) if !messages.is_empty() => {
            let msg = messages.into_iter().fold(String::new(), |mut acc, next| {
                acc += &next.message;
                acc += "\n";
                acc
            });
            return Err(format!("Errors in submitted claim: {}", msg));
        }
        _ => (),
    }

    let mut sp_claim_ret = sp_retrieve
        .0
        .get_claim(&next)
        .await
        .map_err(|e| e.to_string())?;

    if sp_claim_ret.claims.is_empty() {
        return Err("No claims in response when retreiving submitted servicepower claim".into());
    }

    if !sp_claim_ret.messages.is_empty() {
        let msg = sp_claim_ret
            .messages
            .into_iter()
            .fold(String::new(), |mut acc, next| {
                acc += &next.message;
                acc
            });
        return Err(format!(
            "Errors upon retreiving submitted servicepower claim {}",
            msg
        ));
    }

    let sp_claim_ret = sp_claim_ret.claims.remove(0);

    let sp_claim = Some((sp_claim, sp_claim_ret).into());

    let inv = default_qb_invoice(cust.into(), &items, next.clone());
    let qb_invoice = Some(inv.create(&qb.0).await.map_err(|e| e.to_string())?);

    Ok(HAInvoice {
        qb_invoice,
        sp_claim,
    })
}

async fn create_item(part_number: &str, qb: &Quickbooks<Authorized>) -> Result<Item> {
    let item = Item::new()
        .name(part_number)
        .build()
        .map_err(|e| e.to_string())?;

    item.create(qb).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_claim(
    claim_number: String,
    get_qb: bool,
    get_sb: bool,
    app_handle: AppHandle,
) -> Result<HAInvoice> {
    let retreive_handler: State<SPRetrieveState> = app_handle.state();
    let qb: State<QBState> = app_handle.state();

    let sp_claim = match get_sb {
        true => retreive_handler.0.get_claim(&claim_number).await.ok(),
        false => None,
    };

    let qb_invoice = match get_qb {
        true => {
            Some(qb_query!(&qb.0, Invoice | doc_number = claim_number).map_err(|e| e.to_string())?)
        }
        false => None,
    };

    let _ = qb_invoice.as_ref().is_some_and(|inv| {
        println!("{inv}");
        true
    });

    let sp_claim = match sp_claim {
        Some(mut c) => Some(c.claims.remove(0).into()),
        None => None,
    };

    Ok(HAInvoice {
        qb_invoice,
        sp_claim,
    })
}

#[tauri::command]
async fn show_main(app_handle: AppHandle) {
    app_handle
        .get_window("main")
        .expect("Could not get main window from app handle")
        .show()
        .expect("Could not reveal the main window");
    println!("Main window shown");
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
                    state
                        .0
                        .cleanup()
                        .wait()
                        .expect("Couldn't clean up quickbooks client");
                    window.close().unwrap();
                }
                WindowEvent::Destroyed => {
                    state
                        .0
                        .cleanup()
                        .wait()
                        .expect("Couldn't clean up quickbooks client");
                }
                _ => (),
            }
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
