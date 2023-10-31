// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod error;
mod models;
mod util;

use chrono::Utc;
use config::SessionConfig;
use error::ServiceBooksError;
use futures::lock::Mutex;
use models::*;
use quick_oxibooks::{
    actions::QBCreate, client::Quickbooks, qb_query, types::{Attachable, AttachableRef, Invoice, QBToRef, SalesReceipt}, Environment, QBAttachment
};
use service_poxi::ClaimHandler;
use tauri::{generate_context, AppHandle, GlobalWindowEvent, Manager, State, WindowEvent};
use util::*;

struct QBState(Mutex<Option<Quickbooks>>);
struct SPState(ClaimHandler);
struct SMTPState(LettrePack);
type Result<T> = std::result::Result<T, ServiceBooksError>;

#[tauri::command]
async fn submit_claim(
    claim: InputInvoice,
    get_sb: bool,
    app_handle: AppHandle,
) -> Result<HAInvoice> {
    let qb: State<QBState> = app_handle.state();
    let sp: State<SPState> = app_handle.state();

    let qb_ref = qb.0.lock().await;
    let qb_ref = qb_ref.as_ref().ok_or(ServiceBooksError::QBLockError)?;

    let (mut qb_invoice, claim_number) = if let Some(data) = &claim.claim_number {
        (None, data.clone())
    } else {
        let doc_number = generate_claim_number(qb_ref).await?;

        let cust = get_qb_customer(&claim, qb_ref).await?;

        let items = get_qb_items(&claim.parts, qb_ref).await?;

        let inv = default_qb_invoice(cust.to_ref()?, items, doc_number.clone(), &claim);
        (Some(inv.create(qb_ref).await?), doc_number)
    };

    let sp_claim = get_sb.then_some(send_sp(claim, claim_number, &sp.0).await?);

    if let Some(claim) = sp_claim.as_ref() {
        let mut qb_inv = qb_invoice.unwrap_or(qb_query!(
            qb_ref,
            Invoice | doc_number = &claim.claim_number
        )?);

        qb_inv = update_memo(
            qb_ref,
            &mut qb_inv,
            MemoUpdateDetail {
                claim_identifer: claim
                    .claim_identifier
                    .as_deref()
                    .ok_or(ServiceBooksError::MissingClaimIdentifier)?,
            },
        )
        .await?;

        qb_invoice = Some(qb_inv);
    }

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
    let mut out: HAInvoice = Default::default();

    if get_qb {
        let qb: State<QBState> = app_handle.state();
        let qb_ref = qb.0.lock().await;
        let qb_ref = qb_ref
            .as_ref()
            .ok_or(ServiceBooksError::UninitError("QB"))?;
        out.qb_invoice = Some(qb_query!(qb_ref, Invoice | doc_number = claim_number)?);
    }

    if get_sb {
        let retreive_handler: State<SPState> = app_handle.state();
        out.sp_claim = Some(
            retreive_handler
                .0
                .get_claim(claim_number)
                .await?
                .try_get_claim(0)
                .ok_or(ServiceBooksError::EmptyClaimResponse)?
                .into(),
        )
    }

    Ok(out)
}

#[tauri::command]
async fn login(app_handle: AppHandle, token: String) -> Result<()> {
    if let Some(login) = app_handle.get_window("login") {
        login.close()?;
    }

    let qb: State<QBState> = app_handle.state();
    *qb.0.lock().await = Some(
        Quickbooks::new_from_token(
            &token,
            "9130347246064456",
            Environment::PRODUCTION,
            "peepeepoopoo",
        )
        .await?,
    );

    app_handle
        .get_window("main")
        .ok_or(ServiceBooksError::MissingWindow("main"))?
        .show()?;
    println!("Main window shown");

    Ok(())
}

#[tauri::command]
async fn upload_document(
    file_path: String,
    upload_qb: bool,
    upload_sp: bool,
    claim_number: String,
    image_description: String,
    app_handle: AppHandle,
) -> Result<()> {
    let file_path = match file_path.contains('\\') {
        false => file_path,
        true => file_path.replace('\\', "/"),
    };

    if upload_qb {
        let qb: State<QBState> = app_handle.state();
        let qb_ref = qb.0.lock().await;
        let qb_ref = qb_ref
            .as_ref()
            .ok_or(ServiceBooksError::UninitError("QB"))?;

        let a_ref: AttachableRef = match claim_number.strip_suffix("SR") {
            Some(number) => qb_query!(qb_ref, SalesReceipt | doc_number = number)?
                .to_ref()?
                .into(),
            None => qb_query!(qb_ref, Invoice | doc_number = &claim_number)?
                .to_ref()?
                .into(),
        };

        let attach = Attachable {
            attachable_ref: Some(vec![a_ref]),
            file_name: Some(file_path.clone()),
            ..Default::default()
        };

        attach.upload(qb_ref).await?;
    }

    if upload_sp {
        let sp: State<SPState> = app_handle.state();
        sp.0.upload_document_by_claim_number(
            claim_number,
            file_path,
            "MIS".into(), // TODO Find out wth this is
            Some(image_description),
        )
        .await
        .map_err(ServiceBooksError::SPError)?;
    }

    Ok(())
}

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor
};

#[derive(Debug)]
struct LettrePack {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    session: SessionConfig
}

#[tauri::command]
async fn send_email(
    name: String,
    email: String,
    description: String,
    app_handle: AppHandle,
) -> Result<String> {
    let state: State<SMTPState> = app_handle.state();

    let email = Message::builder()
        .from(email.parse().expect("Could not parse email"))
        .to("iamabotforme@gmail.com".parse().unwrap())
        .subject(format!("{name} submitted a bug report"))
        .body(format!(
            "[{datetime}] {name} submitted a bug report from {email}: {description}",
            datetime = Utc::now()
        ))
        .expect("Could not make message");

    state.0.transport.send(email).await?;

    Ok("Successfully emailed bug report".into())
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let session = SessionConfig::load(r"\\10.1.10.24\HamiltonShare\session_config.dat").await?; // TODO Put this under a domain instead of localhost
    let transport = session.clone().try_into()?;

    let pack = LettrePack { transport, session };

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
            upload_document,
            send_email,
            login,
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
        .manage(SPState(
            ClaimHandler::new_from_env().expect("Could not find ServicePower credentials!"),
        ))
        .manage(SMTPState(pack))
        .on_window_event(handle_window_event)
        .run(generate_context!())
        .expect("error while starting tauri application");

    Ok(())
}

fn handle_window_event(event: GlobalWindowEvent) {
    let window = event.window();
    match event.event() {
        WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            handle_close_requested(window);
        }
        WindowEvent::Destroyed if window.label() == "main" => handle_close_requested(window),
        _ => {}
    }
}

fn handle_close_requested(window: &tauri::Window) {
    match window.label() {
        "login" => {
            if let Some(main) = window.get_window("main") {
                main.close().expect("Could not close main window");
            }
        }
        "main" => {
            cleanup(window);
        }
        _ => (),
    }

    window.close().expect("Could not close window!");
}

fn cleanup(window: &tauri::Window) {
  let qb: State<'_, QBState> = window.state();
  if let Some(lock) = qb.0.try_lock() {
    if let Some(qb) = lock.as_ref() {
      let this = qb.cleanup();
      futures::executor::block_on(this).expect("Could not clean up QB Manager");
    }
  }

  let smtp: State<SMTPState> = window.state();
  let this = smtp.0.session.save(r"\\10.1.10.24\HamiltonShare\session_config.dat");
  futures::executor::block_on(this).expect("Could not save session config!");
}