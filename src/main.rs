use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{error, fmt};

#[derive(Debug)]
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

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "us_customers_mapAssign.csv";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mais: Vec<MapAssignInput> = buffered
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

    for mai in mais {
        print!("{:?}\n", mai);
    }

    Ok(())
}
