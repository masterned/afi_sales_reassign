use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "us_customers_mapAssign.csv";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        print!("{}\n", line?);
    }

    Ok(())
}
