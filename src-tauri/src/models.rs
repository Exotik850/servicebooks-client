use quick_oxibooks::{
    actions::QBQuery,
    client::Quickbooks,
    error::APIError,
    types::{
        common::{CustomField, NtRef, TxnTaxDetail},
        Invoice, InvoiceBuilder, LineBuilder, LineDetail, QBItem, SalesItemLineDetailBuilder,
        TaxLineDetail,
    },
    Authorized,
};
use serde::{Deserialize, Serialize};
use service_poxi::{Claim, ClaimBuilder, ClaimBuilderError, ClaimUnion};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Deserialize, Serialize)]
pub(crate) struct InputPart {
    part_number: String,
    invoice_number: i64,
    distributor_number: i64,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct InputInvoice {
    customer_first_name: String,
    customer_last_name: String,
    customer_address_1: String,
    customer_state: String,
    customer_city: String,
    customer_zip_code: String,
    customer_email: Option<String>,
    customer_phone_number: String,
    product_code: String,
    serial_number: i64,
    model_number: String,
    purchase_date: String,
    requested_date: String,
    completed_date: String,
    miles_traveled: i64,
    repair_code: Option<i64>,
    defect_code: Option<i64>,
    issue_description: String,
    service_performed: String,
    parts: Vec<Part>,
}

pub const HA_MANUFACTURER: &'static str = "ALLIANCE - SPEED QUEEN";
pub const HA_MODEL_BRAND: &'static str = "SPEED QUEEN";

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub(crate) struct HAInvoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qb_invoice: Option<Invoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sb_claim: Option<ClaimUnion>,
}

impl QBItem for HAInvoice {
    fn id(&self) -> Option<&String> {
        self.qb_invoice.as_ref().and_then(|i| i.id())
    }

    fn clone_id(&self) -> Option<String> {
        self.qb_invoice.as_ref().and_then(|i| i.clone_id())
    }

    fn sync_token(&self) -> Option<&String> {
        self.qb_invoice.as_ref().and_then(|i| i.sync_token())
    }

    fn meta_data(&self) -> Option<&quick_oxibooks::types::common::MetaData> {
        self.qb_invoice.as_ref().and_then(|i| i.meta_data())
    }

    fn name() -> &'static str {
        Invoice::name()
    }

    fn qb_id() -> &'static str {
        Invoice::qb_id()
    }
}