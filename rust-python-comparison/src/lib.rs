// imports
use polars::prelude::*;

// Functions
pub fn count_to_billion() {
    let mut _count = 0;
    for _i in 0..1000000000 {
        _count += 1;
    }
}

// Load the data while ignoring rows that cause errors with polars
pub fn load_airport_data() -> PolarsResult<DataFrame> {
    CsvReader::from_path("../data/airport_data.csv")?
        .with_ignore_errors(true).finish()
}