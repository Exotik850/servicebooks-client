use quick_oxibooks::types::{Invoice, QBItem, QBToRef};
use serde::{Deserialize, Serialize};
use service_poxi::ClaimUnion;

#[derive(Deserialize, Serialize)]
pub struct InputPart {
    pub part_number: String,
    pub invoice_number: String,
    pub distributor_number: String,
    desc_opt: bool,
    pub description: String,
    id: u32,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct InputInvoice {
    pub first_name: String,
    pub last_name: String,
    pub address_1: String,
    pub state: String,
    pub city: String,
    pub zip_code: String,
    pub email: String,
    pub phone_number: String,
    pub claim_number: Option<String>,
    pub product_code: String,
    pub serial_number: String,
    pub model_number: String,
    pub purchase_date: String,
    pub date_completed: String,
    pub date_requested: String,
    pub miles_traveled: u32,
    pub repair_code: String,
    pub defect_code: String,
    pub issue_description: String,
    pub service_performed: String,
    pub parts: Vec<InputPart>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub(crate) struct HAInvoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qb_invoice: Option<Invoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_claim: Option<ClaimUnion>,
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

impl QBToRef for HAInvoice {
    fn to_ref(
        &self,
    ) -> Result<quick_oxibooks::types::common::NtRef, quick_oxibooks::types::QBError> {
        match self.qb_invoice.as_ref() {
            Some(inv) => inv.to_ref(),
            None => panic!("No QB Invoice on reference!"),
        }
    }
}
