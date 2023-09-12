// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod blocking;
mod models;
mod util;

use blocking::Block;
use futures::lock::Mutex;
use models::*;
use quick_oxibooks::{
    actions::QBCreate, client::Quickbooks, qb_query, types::Invoice, Environment,
};
use service_poxi::{ClaimHandler, Retreive, Submit};
use tauri::{generate_context, AppHandle, GlobalWindowEvent, Manager, State, WindowEvent};
use util::*;

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
    let qb_ref = qb_ref.as_ref().ok_or("Couldn't get QB Lock!".to_owned())?;

    let (qb_invoice, claim_number) = if let Some(data) = &claim.claim_number {
        (None, data.clone())
    } else {
        let doc_number = generate_claim_number(qb_ref)
            .await
            .map_err(|e| e.to_string())?;

        let cust = get_qb_customer(&claim, qb_ref).await?;

        let items = get_qb_items(&claim.parts, qb_ref).await?;

        let inv = default_qb_invoice(cust.into(), items, doc_number.clone());
        (
            Some(inv.create(qb_ref).await.map_err(|e| e.to_string())?),
            doc_number,
        )
    };

    let sp_claim = if get_sb {
        Some(send_sp(claim, claim_number, &sp_submit.0, &sp_retrieve.0).await?)
    } else {
        None
    };

    Ok(HAInvoice {
        qb_invoice,
        sp_claim,
    })
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

    let qb_ref = qb.0.lock().await;
    let qb_ref = qb_ref
        .as_ref()
        .ok_or("Could not grab lock on QB State!".to_string())?;

    let qb_invoice = match get_qb {
        true => Some(
            qb_query!(qb_ref, Invoice | doc_number = claim_number).map_err(|e| e.to_string())?,
        ),
        false => None,
    };

    let sp_claim = match get_sb {
        true => Some(
            retreive_handler
                .0
                .get_claim(&claim_number)
                .await
                .map_err(|e| e.to_string())?
                .claims
                .remove(0)
                .into(),
        ),
        false => None,
    };

    Ok(HAInvoice {
        qb_invoice,
        sp_claim,
    })
}

#[tauri::command]
async fn login(app_handle: AppHandle, token: String) -> Result<()> {
    if let Some(login) = app_handle.get_window("login") {
        login.close().map_err(|e| e.to_string())?;
    }

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

    app_handle
        .get_window("main")
        .ok_or("Could not get main window from app handle".to_string())?
        .show()
        .map_err(|e| e.to_string())?;
    println!("Main window shown");

    Ok(())
}

#[tauri::command]
async fn show_main(app_handle: AppHandle) -> Result<()> {
    let state: State<QBState> = app_handle.state();
    if state.0.lock().await.is_some() {
        let window = app_handle
            .get_window("main")
            .ok_or::<String>("No Main window found!".into())?;
        window.show().map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let qb = if let Ok(data) = intuit_oxi_auth::TokenSession::grab_from_cache_async(
        intuit_oxi_auth::Environment::PRODUCTION.cache_name(),
        "peepeepoopoo",
    )
    .await
    {
        Some(
            Quickbooks::new_from_session(
                data,
                "9130347246064456",
                Environment::PRODUCTION,
                "peepeepoopoo",
            )
            .await
            .expect("Could not open quickbooks client"),
        )
    } else {
        open::that_detached("https://developer.intuit.com/app/developer/playground")
            .expect("Could not open URL!");
        None
    };

    let cache_hit = qb.is_some();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            submit_claim,
            get_claim,
            login,
            show_main
        ])
        .setup(move |app| {
            let window = app.get_window("main").expect("No main window on startup");
            window_shadows::set_shadow(&window, true).expect("Couldn't set shadow on window!");

            if cache_hit {
                let login = app.get_window("login").expect("No Login window to close");
                login.close().expect("Could not close login window");
            }

            Ok(())
        })
        .manage(QBState(qb.into()))
        .manage(SPRetrieveState(ClaimHandler::<Retreive>::new()))
        .manage(SPSubmitState(ClaimHandler::<Submit>::new()))
        .on_window_event(handle_window_event)
        .run(generate_context!())
        .expect("error while running tauri application");
}

fn handle_window_event(event: GlobalWindowEvent) {
    let window = event.window();
    let state = window.state();

    match event.event() {
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            handle_close_requested(window, state);
        }
        WindowEvent::Destroyed if window.label() == "main" => {
            close(state.inner());
        }
        _ => {}
    }
}

fn handle_close_requested(window: &tauri::Window, state: State<QBState>) {
    match window.label() {
        "login" => {
            if let Some(main) = window.get_window("main") {
                main.close().expect("Could not close main window");
            }
        }
        "main" => close(state.inner()),
        _ => (),
    }

    window.close().expect("Could not close window!");
}

fn close(qb: &QBState) {
    if let Some(lock) = qb.0.try_lock() {
        if let Some(qb) = lock.as_ref() {
            qb.cleanup()
                .wait()
                .expect("Couldn't clean up quickbooks client");
        }
    }
}
