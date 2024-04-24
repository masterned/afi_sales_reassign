use std::{
    collections::HashMap,
    error, fmt,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Clone, Debug)]
struct MapAssignInput {
    customer: String,
    state_code: String,
    sales_rep: String,
}

#[derive(Debug)]
enum MapAssignInputParseError {
    MissingField(&'static str),
    ImproperFormat(&'static str, &'static str),
}

impl fmt::Display for MapAssignInputParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MapAssignInput parse Error: {}",
            match self {
                MapAssignInputParseError::MissingField(field) =>
                    format!("Missing required field: {}", field),
                MapAssignInputParseError::ImproperFormat(field, format) =>
                    format!("{} should be in format: {}", field, format),
            }
        )
    }
}

impl error::Error for MapAssignInputParseError {}

impl FromStr for MapAssignInput {
    type Err = MapAssignInputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(',');
        let customer = tokens
            .next()
            .ok_or(MapAssignInputParseError::MissingField("Customer"))?
            .to_string();
        let state_code = tokens
            .next()
            .ok_or(MapAssignInputParseError::MissingField("State Code"))?
            .to_string();
        let sales_rep = tokens
            .skip(1)
            .next()
            .ok_or(MapAssignInputParseError::MissingField("Sales Rep"))?
            .to_string();
        Ok(MapAssignInput {
            customer,
            state_code,
            sales_rep,
        })
    }
}

fn read_map_into_hashmap(path: &str) -> Result<HashMap<String, MapAssignInput>, io::Error> {
    let map_input = File::open(path)?;
    let map_buffer = io::BufReader::new(map_input);

    let mais: Vec<MapAssignInput> = map_buffer
        .lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                let line_string = format!("{}", line);
                (&line_string).parse().ok()
            } else {
                None
            }
        })
        .collect();

    Ok(mais
        .iter()
        .map(|mai| (mai.customer.clone(), mai.clone()))
        .collect())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let map_path = "us_customers_mapAssign.csv";
    let customer_map = read_map_into_hashmap(map_path)?;

    dbg!(customer_map);

    Ok(())
}
