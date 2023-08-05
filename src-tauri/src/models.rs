use chrono::NaiveDate;
use quick_oxibooks::types::{Invoice, QBItem};
use serde::{Deserialize, Serialize};
use service_poxi::Claim;


#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct HAInvoice {
    qb_invoice: Invoice,
    sb_claim: Claim
}

impl QBItem for HAInvoice {

}