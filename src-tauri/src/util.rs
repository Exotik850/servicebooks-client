use quick_oxibooks::{
    actions::QBCreate, client::Quickbooks, error::APIError, qb_query, types::{
        common::{CustomField, NtRef, TxnTaxDetail}, Invoice, Item, LineBuilder, LineDetail, SalesItemLineDetailBuilder, TaxLineDetail
    }
};
use service_poxi::{Claim, ClaimBuilder, ClaimHandler, ClaimUnion, Retreive, Submit};

use crate::models::{InputInvoice, InputPart};

pub const HA_MANUFACTURER: &str = "ALLIANCE - SPEED QUEEN";
pub const HA_MODEL_BRAND: &str = "SPEED QUEEN";

pub(crate) fn default_qb_invoice(
    customer_ref: NtRef,
    items: Vec<NtRef>,
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
                percent_based: Some(true),
                tax_percent: Some(9.75),
                ..Default::default()
            }))
            .build()
            .unwrap()]),
        txn_tax_code_ref: Some("35".into()),
        total_tax: Some(0.0),
    };

    let customer_memo = format!(
        "Warranty Claim Filed date w/Service Power: {today}
        Claim # CLAIM_PLACEHOLDER
        Claim paid 8/xx/23 $CLAIM_PAID_AMT ()
        Voucher # VOUCHER_PLACEHOLDER
        Parts paid via Marcone ($PARTS_PAID_AMT)
        Invoice # PART_INVOICE_PLACEHOLDER dated PART_PAID_DATE",
        today = chrono::Utc::now().date_naive()
    );

    // RA Doesn't like this for some reason
    Invoice::new()
        .custom_field(custom_field)
        .customer_ref(customer_ref)
        .sales_term_ref(sales_term_ref)
        .line(line)
        .doc_number(doc_number)
        .txn_tax_detail(txn_tax_detail)
        .customer_memo::<NtRef>(customer_memo.as_str().into())
        .build()
        .unwrap()
}

pub(crate) fn default_sp_claim(
    claim: InputInvoice,
    pn: u64,
    claim_number: String,
) -> Result<Claim, String> {
    let InputInvoice {
        purchase_date,
        date_completed,
        date_requested,
        parts,
        customer_address_1,
        customer_city,
        customer_email,
        customer_first_name,
        customer_last_name,
        customer_state,
        customer_zip_code,
        defect_code,
        repair_code,
        model_number,
        miles_traveled,
        serial_number,
        service_performed,
        issue_description,
        ..
    } = claim;

    let parts: Vec<service_poxi::Part> = parts
        .into_iter()
        .map(|p| service_poxi::Part {
            distributor_number: Some(p.distributor_number),
            number: p.part_number,
            quantity: Some(1),
            invoice_number: Some(p.invoice_number),
            ..Default::default()
        })
        .collect();

    let purchase_date: String = purchase_date.split('-').collect();
    let requested_date: String = date_requested.split('-').collect();
    let completed_date: String = date_completed.split('-').collect();

    ClaimBuilder::default()
        .brand_name(HA_MODEL_BRAND)
        .manufacturer_name(HA_MANUFACTURER)
        .service_center_number("4683")
        .claim_number(claim_number)
        .model_number(model_number)
        .eia_defect_or_complaint_code(defect_code)
        .serial_number(serial_number)
        .customer_city(customer_city)
        .date_purchased(purchase_date.parse::<u32>().map_err(|e| e.to_string())?)
        .date_completed(completed_date.parse::<u32>().map_err(|e| e.to_string())?)
        .date_requested(requested_date.parse::<u32>().map_err(|e| e.to_string())?)
        .customer_first_name(customer_first_name)
        .customer_last_name(customer_last_name)
        .customer_email(customer_email)
        .travel_miles(miles_traveled)
        .eia_repair_code_1(repair_code)
        .service_performed_description(service_performed)
        .customer_address_1(customer_address_1)
        .customer_state(customer_state)
        .customer_zip_code(customer_zip_code)
        .customer_phone(pn)
        .defect_or_complaint_description(issue_description)
        .parts(parts)
        .build()
        .map_err(|e| e.to_string())
}

pub(crate) async fn send_sp(
    claim: InputInvoice,
    claim_number: String,
    sp_submit: &ClaimHandler<Submit>,
    sp_retrieve: &ClaimHandler<Retreive>,
) -> Result<ClaimUnion, String> {
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

pub(crate) async fn generate_claim_number(qb: &Quickbooks) -> Result<String, APIError> {
    let inv =
        qb_query!(qb, Invoice | doc_number like "%W" ; "orderby DocNumber desc startposition 2")?;

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

pub async fn get_qb_items(parts: &[InputPart], qb: &Quickbooks) -> Result<Vec<NtRef>, String> {
    let mut items = vec![];
    for part in parts.iter() {
        match quick_oxibooks::qb_query!(qb, Item | name = &part.part_number) {
            Ok(inv) => items.push(inv.into()),
            Err(_) => {
                let new_item = create_item(&part.part_number, qb).await?;
                items.push(new_item.into())
            }
        }
    }
    Ok(items)
}

async fn create_item(part_number: &str, qb: &Quickbooks) -> Result<Item, String> {
    let item = Item::new()
        .name(part_number)
        .build()
        .map_err(|e| e.to_string())?;

    item.create(qb).await.map_err(|e| e.to_string())
}
