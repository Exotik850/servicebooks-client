use chrono::NaiveDate;
use serde::{Deserialize, Serialize};


#[derive(Default, Serialize, Deserialize)]
pub(crate) struct HAInvoice {
    doc_number: String,
    part_invoice_number: String,
    voucher_number: String,
    claim_number: String,
    filed_date: NaiveDate,
}