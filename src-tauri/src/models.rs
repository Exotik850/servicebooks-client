use quick_oxibooks::types::{
    common::{CustomField, NtRef, TxnTaxDetail},
    Invoice, InvoiceBuilder, LineBuilder, LineDetail, QBItem, SalesItemLineDetailBuilder,
    TaxLineDetail,
};
use serde::{Deserialize, Serialize};
use service_poxi::ClaimUnion;

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

pub(crate) fn default_invoice(customer_ref: NtRef, items: &[NtRef]) -> Invoice {
    let custom_field = vec![CustomField {
        definition_id: Some("2".into()),
        string_value: Some("SQ".into()),
        name: Some("Sales Rep".into()),
        field_type: None,
    }];

    let line = items.iter().fold(vec![LineBuilder::default()
        .line_detail(Some(LineDetail::SalesItemLineDetail(
            SalesItemLineDetailBuilder::default()
                .item_ref::<NtRef>(("Warranty - Speed Queen:SQ Warranty Call", "5489").into())
                .tax_code_ref::<NtRef>("TAX".into())
                .qty(1u32)
                .build()
                .unwrap(),
            )))
        .description(Some("All Speed Queen warranty call information - should never have a balance. *See Tammy for details. ALWAYS change tax for Shelby county/ MS".into()))
        .build()
        .unwrap()], |mut acc, next| {
            acc.push(LineBuilder::default()
            .line_detail(Some(LineDetail::SalesItemLineDetail(
                SalesItemLineDetailBuilder::default()
                    .item_ref(next.to_owned())
                    .tax_code_ref::<NtRef>("TAX".into())
                    .qty(1u32)
                    .build()
                    .unwrap(),
                )))
            .build()
            .unwrap());
            acc
        });

    let sales_term_ref = ("Net 15", "22").into();

    let txn_tax_detail = TxnTaxDetail {
        tax_line: Some(vec![
                    LineBuilder::default().line_detail(Some(LineDetail::TaxLineDetail(TaxLineDetail {
                        percent_based: true,
                        tax_percent: 9.75,
                        ..Default::default()
                    })))
                    .build()
                    .unwrap(),
                ]),
        txn_tax_code_ref: Some("35".into()),
        total_tax: Some(0.0),
    };

    InvoiceBuilder::default()
        .custom_field(Some(custom_field))
        .customer_ref(Some(customer_ref))
        .sales_term_ref(Some(sales_term_ref))
        .line(Some(line))
        .customer_memo(Some(NtRef {
            value: Some(
                "Warranty Claim Filed date w/Service Power: 8/xx/23\n
            Claim # CLAIM_PLACEHOLDER\n
            Claim paid 8/xx/23 $XXX ()\n
            Voucher # VOUCHER_PLACEHOLDER\n
            Parts paid via Marcone ($xx.xx)\n
            Invoice # PART_INVOICE_PLACEHOLDER dated 8/xx/23"
                    .into(),
            ),
            ..Default::default()
        }))
        .build()
        .unwrap()
}
