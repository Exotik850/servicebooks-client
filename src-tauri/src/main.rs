// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod blocking;
mod models;
mod util;

use blocking::Block;
use futures::lock::Mutex;
use models::*;
use quick_oxibooks::{
    actions::QBCreate,
    qb_query,
    types::{Customer, Invoice, Item},
    {client::Quickbooks, Environment},
};
use service_poxi::{ClaimHandler, ClaimUnion, Retreive, Submit};
use tauri::{generate_context, AppHandle, Manager, State, WindowEvent};
use util::*;

#[cfg(any(windows, target_os = "macos"))]
use window_shadows::set_shadow;

struct QBState(Mutex<Option<Quickbooks>>);
struct SPRetrieveState(ClaimHandler<Retreive>);
struct SPSubmitState(ClaimHandler<Submit>);

type Result<T> = std::result::Result<T, String>;

#[tauri::command]
async fn submit_claim(
    claim: InputInvoice,
    get_sb: bool,
    app_handle: AppHandle,
) -> Result<HAInvoice> {
    let qb: State<QBState> = app_handle.state();
    let sp_submit: State<SPSubmitState> = app_handle.state();
    let sp_retrieve: State<SPRetrieveState> = app_handle.state();

    let qb_ref = qb.0.lock().await;
    let qb_ref = qb_ref.as_ref().unwrap();

    let (next, qb_invoice) = match &claim.claim_number {
        Some(number) => (number.clone(), None),
        None => {
            let doc_number = generate_claim_number(qb_ref)
                .await
                .map_err(|e| e.to_string())?;

            let cust = qb_query!(
                qb_ref,
                Customer | given_name = &claim.customer_first_name,
                family_name = &claim.customer_last_name
            )
            .map_err(|e| e.to_string())?;

            let mut items = vec![];

            for part in claim.parts.iter() {
                match qb_query!(qb_ref, Item | name = &part.part_number) {
                    Ok(inv) => items.push(inv.into()),
                    Err(_) => {
                        let new_item = create_item(&part.part_number, qb_ref).await?;
                        items.push(new_item.into())
                    }
                }
            }
            let inv = default_qb_invoice(cust.into(), &items, doc_number.clone());
            (
                doc_number,
                Some(inv.create(qb_ref).await.map_err(|e| e.to_string())?),
            )
        }
    };

    let sp_claim = if get_sb {
        Some(send_sp(claim, next, &sp_submit.0, &sp_retrieve.0).await?)
    } else {
        None
    };

    Ok(HAInvoice {
        qb_invoice,
        sp_claim,
    })
}

async fn create_item(part_number: &str, qb: &Quickbooks) -> Result<Item> {
    let item = Item::new()
        .name(part_number)
        .build()
        .map_err(|e| e.to_string())?;

    item.create(qb).await.map_err(|e| e.to_string())
}

async fn send_sp(
    claim: InputInvoice,
    claim_number: String,
    sp_submit: &ClaimHandler<Submit>,
    sp_retrieve: &ClaimHandler<Retreive>,
) -> Result<ClaimUnion> {
    let Ok(phone_number) = claim.customer_phone_number.parse::<u64>() else {
        return Err("Could not parse phone number, do not use anything other than numbers in the phone number field".into());
    };

    let sp_claim = default_sp_claim(claim, phone_number, claim_number.clone())?;

    let mut sp_claim_sub = sp_submit
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

    if let Some(messages) = sent.messages {
        if !messages.is_empty() {
            let msg = messages.into_iter().fold(String::new(), |mut acc, next| {
                acc += &next.message;
                acc += "\n";
                acc
            });
            return Err(format!(
                "Errors in submitted claim: {}",
                &msg[..msg.len() - 1]
            ));
        }
    }

    let mut sp_claim_ret = sp_retrieve
        .get_claim(&claim_number)
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

    Ok((sp_claim, sp_claim_ret).into())
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

    let qb_ref = qb.0.lock().await;
    let qb_ref = qb_ref.as_ref().unwrap();

    let qb_invoice = match get_qb {
        true => Some(
            qb_query!(qb_ref, Invoice | doc_number = claim_number).map_err(|e| e.to_string())?,
        ),
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
async fn login(app_handle: AppHandle, token: String) -> Result<()> {
    if let Some(login) = app_handle.get_window("login") {
        login.close().unwrap();
    }

    app_handle
        .get_window("main")
        .expect("Could not get main window from app handle")
        .show()
        .expect("Could not reveal the main window");
    println!("Main window shown");

    let qb: State<QBState> = app_handle.state();
    *qb.0.lock().await = Some(
        Quickbooks::new_from_token(
            &token,
            "9130347246064456",
            Environment::PRODUCTION,
            "peepeepoopoo",
        )
        .await
        .map_err(|e| e.to_string())?,
    );

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let qb = if let Ok(data) = intuit_oxi_auth::TokenSession::grab_from_cache_async(intuit_oxi_auth::Environment::PRODUCTION.cache_name(), "peepeepoopoo").await {
        Some(Quickbooks::new_from_session(data, "9130347246064456",
        Environment::PRODUCTION,
        "peepeepoopoo").await.expect("Could not open quickbooks client"))
    } else {
        open::that_detached("https://developer.intuit.com/v2/OAuth2Playground").expect("Could not open URL!");
        None
    };

    let cache_hit = qb.is_some();

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![submit_claim, get_claim, login])
    .setup(move |app| {
        let window = app.get_window("main").unwrap();
        #[cfg(any(windows, target_os = "macos"))]
        set_shadow(&window, true).unwrap();
        
        if cache_hit {
            let login = app.get_window("login").unwrap();
            login.close().expect("Could not close login window");
            window.show().expect("Could not show main window");
        }
        
        Ok(())
    })
    .manage(QBState(qb.into()))
    .manage(SPRetrieveState(ClaimHandler::<Retreive>::new()))
    .manage(SPSubmitState(ClaimHandler::<Submit>::new()))
    .on_window_event(|event| {
            let window = event.window();
            let state: State<QBState> = window.state();

            match event.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    if let Some(lock) = state.0.try_lock() {
                        if let Some(qb) = lock.as_ref() {
                            qb.cleanup()
                                .wait()
                                .expect("Couldn't clean up quickbooks client");
                        }
                    }
                    window.close().unwrap();
                }
                WindowEvent::Destroyed => {
                    if let Some(lock) = state.0.try_lock() {
                        if let Some(qb) = lock.as_ref() {
                            qb.cleanup()
                                .wait()
                                .expect("Couldn't clean up quickbooks client");
                        }
                    }
                }
                _ => (),
            }
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
