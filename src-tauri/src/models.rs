use quick_oxibooks::types::{Invoice, QBItem};
use serde::{Deserialize, Serialize};
use service_poxi::{ClaimUnion, RetrievedClaim};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub(crate) struct HAInvoice {
    qb_invoice: Invoice,
    sb_claim: ClaimUnion,
}

impl QBItem for HAInvoice {
    fn id(&self) -> Option<&String> {
        self.qb_invoice.id()
    }

    fn clone_id(&self) -> Option<String> {
        self.qb_invoice.clone_id()
    }

    fn sync_token(&self) -> Option<&String> {
        self.qb_invoice.sync_token()
    }

    fn meta_data(&self) -> Option<&quick_oxibooks::types::common::MetaData> {
        self.qb_invoice.meta_data.as_ref()
    }

    fn name() -> &'static str {
        Invoice::name()
    }

    fn qb_id() -> &'static str {
        Invoice::qb_id()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RetreiveUnion {
    pub invoice: Invoice,
    pub service_claim: RetrievedClaim,
}
