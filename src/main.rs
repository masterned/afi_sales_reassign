use std::{collections::HashMap, error};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct MapRecord {
    company_name: String,
    state_code: String,
    country_code: String,
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

#[derive(Debug)]
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
    let map_path = "us_customers_mapAssign.csv";
    let mut map_rdr = csv::Reader::from_path(map_path)?;

    let map_records: Vec<MapRecord> = map_rdr
        .deserialize()
        .filter_map(|record| record.ok())
        .collect();

    let mut result: HashMap<String, ResultRecord> = map_records
        .iter()
        .map(|r| (r.company_name.clone(), r.clone().into()))
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

    println!("{:#?}", result);

    Ok(())
}
