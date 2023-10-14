use quick_oxibooks::{
    actions::QBCreate, client::Quickbooks, qb_query, types::{
        common::{Addr, CustomField, Email, NtRef, PhoneNumber, TxnTaxDetail}, Customer, Invoice, Item, LineBuilder, LineDetail, QBToRef, SalesItemLineDetailBuilder, TaxLineDetail
    }
};
use service_poxi::{Claim, ClaimBuilder, ClaimHandler, ClaimUnion, MessageContainer};

use crate::models::{InputInvoice, InputPart};

pub const HA_MANUFACTURER: &str = "ALLIANCE - SPEED QUEEN";
pub const HA_MODEL_BRAND: &str = "SPEED QUEEN";

pub(crate) struct MemoUpdateDetail<'a> {
    pub claim_identifer: &'a str,
}

pub(crate) async fn update_memo(
    qb_ref: &Quickbooks,
    invoice: &mut Invoice,
    update: MemoUpdateDetail<'_>,
) -> Result<Invoice, String> {
    let MemoUpdateDetail { claim_identifer } = update;

    let Some(memo) = invoice.customer_memo.as_mut() else {
        return Err("Invoice has no memo to change".into());
    };

    let Some(v) = memo.value.as_mut() else {
        return Err("Memo has no value to change".into());
    };

    *v = v.replace("CLAIM_PLACEHOLDER", claim_identifer);
    Ok(invoice.create(qb_ref).await.map_err(|e| e.to_string())?)
}

pub(crate) async fn get_qb_customer(
    claim: &InputInvoice,
    qb: &Quickbooks,
) -> Result<Customer, String> {
    if let Ok(cust) = qb_query!(
        qb,
        Customer | given_name = &claim.first_name,
        family_name = &claim.last_name
    ) {
        return Ok(cust);
    }

    let cust = Customer::new()
        .given_name(claim.first_name.clone())
        .family_name(claim.last_name.clone())
        .bill_addr(Addr {
            city: Some(claim.city.clone()),
            country: Some("USA".into()),
            country_sub_division_code: Some(claim.state.clone()),
            line1: Some(claim.address_1.clone()),
            postal_code: Some(claim.zip_code.clone()),
            id: None,
        })
        .primary_email_addr(Email {
            address: Some(claim.email.clone()),
        })
        .primary_phone(PhoneNumber {
            free_form_number: Some(claim.phone_number.clone()),
        })
        .build()
        .map_err(|e| e.to_string())?;

    cust.create(qb).await.map_err(|e| e.to_string())
}

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
        address_1,
        city,
        email,
        first_name,
        last_name,
        state,
        zip_code,
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
        .customer_city(city)
        .date_purchased(purchase_date.parse::<u32>().map_err(|e| e.to_string())?)
        .date_completed(completed_date.parse::<u32>().map_err(|e| e.to_string())?)
        .date_requested(requested_date.parse::<u32>().map_err(|e| e.to_string())?)
        .customer_first_name(first_name)
        .customer_last_name(last_name)
        .customer_email(email)
        .travel_miles(miles_traveled)
        .eia_repair_code_1(repair_code)
        .service_performed_description(service_performed)
        .customer_address_1(address_1)
        .customer_state(state)
        .customer_zip_code(zip_code)
        .customer_phone(pn)
        .defect_or_complaint_description(issue_description)
        .parts(parts)
        .build()
        .map_err(|e| e.to_string())
}

pub(crate) async fn send_sp(
    claim: InputInvoice,
    claim_number: String,
    sp: &ClaimHandler,
) -> Result<ClaimUnion, String> {
    let Ok(phone_number) = claim.phone_number.parse::<u64>() else {
        return Err("Could not parse phone number, do not use anything other than numbers in the phone number field".into());
    };

    let sp_claim = default_sp_claim(claim, phone_number, claim_number.clone())?;

    let mut sp_claim_sub = sp
        .submit_claim(sp_claim.clone())
        .await
        .map_err(|e| e.to_string())?;

    if let Some(text) = sp_claim_sub.error_text() {
        return Err(format!("Error on Submitting Servicepower Claim:{text}"));
    }
    let sent = sp_claim_sub.get_claim(0);

    if let Some(messages) = sent.messages {
        if !messages.is_empty() {
            return Err(format!(
                "Errors in submitted claim: {}",
                messages.join_messages()
            ));
        }
    }

    let mut sp_claim_ret = sp
        .get_claim(claim_number)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(text) = sp_claim_ret.error_text() {
        return Err(format!("Error on Retreiving Servicepower Claim:{text}"));
    }

    let sp_claim_ret = sp_claim_ret.get_claim(0);

    Ok((sp_claim, sp_claim_ret).into())
}

pub(crate) async fn generate_claim_number(qb: &Quickbooks) -> Result<String, String> {
    let inv =
        qb_query!(qb, Invoice | doc_number like "%W" ; "orderby DocNumber desc startposition 2")
            .map_err(|e| e.to_string())?;

    let num = inv.doc_number.unwrap(); // Protected by query, always safe

    let num = num[0..num.len() - 1]
        .parse::<u64>()
        .map_err(|e| e.to_string())?
        + 1;

    let num = format!("{}W", num);

    Ok(num)
}

pub async fn get_qb_items(parts: &[InputPart], qb: &Quickbooks) -> Result<Vec<NtRef>, String> {
    let mut items = vec![];
    for part in parts.iter() {
        match quick_oxibooks::qb_query!(qb, Item | name = &part.part_number) {
            Ok(inv) => items.push(inv.to_ref().map_err(|e| e.to_string())?),
            Err(_) => {
                let new_item = create_item(&part.part_number, qb).await?;
                items.push(new_item.to_ref().map_err(|e| e.to_string())?)
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
