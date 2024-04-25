use std::{collections::HashMap, error};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct AddressBookRecord {
    full_name: String,
    first_name: String,
    last_name: String,
    company_name: String,
    phone_number: String,
    email_address: String,
    username: String,
    address_line_1: String,
    address_line_2: String,
    city: String,
    state: String,
    postal_code: String,
    country_code: String,
    sales_rep: String,
    industry: String,
}

#[derive(Clone, Debug, Deserialize)]
struct MapRecord {
    company_name: String,
    state_code: String,
    country_code: String,
    sales_rep: String,
}

#[derive(Clone, Debug, Deserialize)]
struct StateRepRecord {
    state_code: String,
    sales_rep: String,
}

#[derive(Clone, Debug, Deserialize)]
struct SalesRecord {
    sales_rep: String,
    company_name: String,
    prev_amount_sum: f32,
    prev_invoice_count: usize,
    cur_amount_sum: f32,
    cur_invoice_count: usize,
    amount_variance: f32,
}

#[derive(Clone, Debug)]
struct ResultRecord {
    company_name: String,
    sales_rep: String,
}

impl From<MapRecord> for ResultRecord {
    fn from(value: MapRecord) -> Self {
        ResultRecord {
            company_name: value.company_name,
            sales_rep: value.sales_rep,
        }
    }
}

impl From<SalesRecord> for ResultRecord {
    fn from(value: SalesRecord) -> Self {
        ResultRecord {
            company_name: value.company_name,
            sales_rep: value.sales_rep,
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let state_reps_path = "state_reps.csv";
    let mut state_reps_rdr = csv::Reader::from_path(state_reps_path)?;

    let state_reps: HashMap<String, String> = state_reps_rdr
        .deserialize()
        .filter_map(|r| r.ok())
        .map(|state_rep_record: StateRepRecord| {
            (state_rep_record.state_code, state_rep_record.sales_rep)
        })
        .collect();

    let address_book_path = "address_book.csv";
    let mut address_book_rdr = csv::Reader::from_path(address_book_path)?;

    let mut result: HashMap<String, ResultRecord> = address_book_rdr
        .deserialize()
        .filter_map(|r| r.ok())
        .filter(|r: &AddressBookRecord| r.country_code == "US")
        .map(|address_book_record| {
            let sales_rep;

            if let Some(s0) = state_reps.get(&address_book_record.state) {
                sales_rep = s0.to_string();
            } else {
                sales_rep = address_book_record.sales_rep;
            }

            (
                address_book_record.company_name.clone(),
                ResultRecord {
                    company_name: address_book_record.company_name,
                    sales_rep,
                },
            )
        })
        .collect();

    let sales_path = "customers_with_sales.csv";
    let mut sales_rdr = csv::Reader::from_path(sales_path)?;

    let sales_records: Vec<SalesRecord> =
        sales_rdr.deserialize().filter_map(|rec| rec.ok()).collect();

    sales_records.iter().for_each(|sales_record| {
        result
            .entry(sales_record.company_name.clone())
            .and_modify(|e| e.sales_rep = sales_record.sales_rep.clone())
            .or_insert_with(|| (*sales_record).clone().into());
    });

    let result_path = "result.csv";
    let mut wtr = csv::Writer::from_path(result_path)?;

    wtr.write_record(&["Company Name", "Sales Rep"])?;

    for value in result.values() {
        wtr.write_record(&[value.company_name.clone(), value.sales_rep.clone()])?;
    }

    wtr.flush()?;

    Ok(())
}
