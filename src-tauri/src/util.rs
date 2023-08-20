use chrono::{NaiveDate, DateTime};
use quick_oxibooks::{
    actions::QBQuery,
    client::Quickbooks,
    error::APIError,
    types::{
        common::{CustomField, NtRef, TxnTaxDetail},
        Invoice, LineBuilder, LineDetail, SalesItemLineDetailBuilder,
        TaxLineDetail,
    },
    Authorized,
};
use service_poxi::{Claim, ClaimBuilder, ClaimBuilderError};

pub const HA_MANUFACTURER: &'static str = "ALLIANCE - SPEED QUEEN";
pub const HA_MODEL_BRAND: &'static str = "SPEED QUEEN";

pub(crate) fn default_qb_invoice(
    customer_ref: NtRef,
    items: &[NtRef],
    doc_number: String,
) -> Invoice {
    let custom_field = vec![CustomField {
        definition_id: Some("2".into()),
        string_value: Some("SQ".into()),
        name: Some("Sales Rep".into()),
        field_type: None,
    }];

    let line = items.into_iter().fold(vec![LineBuilder::default()
        .line_detail(LineDetail::SalesItemLineDetail(
            SalesItemLineDetailBuilder::default()
            .item_ref::<NtRef>(("Warranty - Speed Queen:SQ Warranty Call","5489").into())
            .tax_code_ref::<NtRef>("TAX".into())
            .qty(1u32)
            .build()
            .unwrap()
        ))
        .description("All Speed Queen warranty call information - should never have a balance. *See Tammy for details. ALWAYS change tax for Shelby county/ MS")
        .amount(0.0)
        .build()
        .unwrap()], |mut acc, next| {
            acc.push(LineBuilder::default()
            .line_detail(LineDetail::SalesItemLineDetail(
                SalesItemLineDetailBuilder::default()
                    .item_ref(next.to_owned())
                    .tax_code_ref::<NtRef>("TAX".into())
                    .qty(1u32)
                    .build()
                    .unwrap(),
                ))
            .amount(0.0)
            .build()
            .unwrap());
            acc
        });

    let sales_term_ref: NtRef = ("Net 15", "22").into();

    let txn_tax_detail = TxnTaxDetail {
        tax_line: Some(vec![LineBuilder::default()
            .line_detail(LineDetail::TaxLineDetail(TaxLineDetail {
                percent_based: true,
                tax_percent: 9.75,
                ..Default::default()
            }))
            .build()
            .unwrap()]),
        txn_tax_code_ref: Some("35".into()),
        total_tax: Some(0.0),
    };

    // RA Doesn't like this for some reason
    Invoice::new()
        .custom_field(custom_field)
        .customer_ref(customer_ref)
        .sales_term_ref(sales_term_ref)
        .line(line)
        .doc_number(doc_number)
        .customer_memo("Warranty Claim Filed date w/Service Power: 8/xx/23\nClaim # CLAIM_PLACEHOLDER\nClaim paid 8/xx/23 $XXX ()\nVoucher # VOUCHER_PLACEHOLDER\nParts paid via Marcone ($xx.xx)\nInvoice # PART_INVOICE_PLACEHOLDER dated 8/xx/23")
        .build()
        .unwrap()
}

pub(crate) fn default_sb_claim(claim_number: String) -> Result<Claim, ClaimBuilderError> {
    ClaimBuilder::default()
        .distributor_number("4683")
        .brand_name(HA_MODEL_BRAND)
        .manufacturer_name(HA_MANUFACTURER)
        .claim_number(claim_number)
        .build()
}

pub(crate) async fn generate_claim_number(qb: &Quickbooks<Authorized>) -> Result<String, APIError> {
    let inv = Invoice::query_single(
        qb,
        "where DocNumber like '%W' orderby DocNumber desc startposition 2",
    )
    .await?;

    let num = inv.doc_number.unwrap(); // Protected by query, always safe
    let index = num
        .chars()
        .position(|w| w == 'W')
        .expect("No W in retrieved DocNumber");

    let num = num[0..index]
        .parse::<u64>()
        .expect("Couldn't Parse DocNumber")
        + 1;
        
    let num = format!("{}W", num);

    Ok(num)
}
