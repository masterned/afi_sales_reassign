use std::error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MapRecord {
    company_name: String,
    state_code: String,
    country_code: String,
    sales_rep: String,
}

#[derive(Debug, Deserialize)]
struct SalesRecord {
    sales_rep: String,
    company_name: String,
    prev_amount_sum: f32,
    prev_invoice_count: usize,
    cur_amount_sum: f32,
    cur_invoice_count: usize,
    amount_variance: f32,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let map_path = "us_customers_mapAssign.csv";
    let mut map_rdr = csv::Reader::from_path(map_path)?;

    let map_records: Vec<MapRecord> = map_rdr
        .deserialize()
        .filter_map(|record| record.ok())
        .collect();
    println!("{:#?}", map_records);

    let sales_path = "customers_with_sales.csv";
    let mut sales_rdr = csv::Reader::from_path(sales_path)?;

    let sales_records: Vec<SalesRecord> =
        sales_rdr.deserialize().filter_map(|rec| rec.ok()).collect();
    println!("{:#?}", sales_records);

    Ok(())
}
