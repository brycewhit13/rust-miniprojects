// Crates
use polars;

// Functions
pub fn load_data() -> polars::prelude::DataFrame {
    let df = polars::csv::Reader::from_path("data/heart.csv")
        .has_header(true)
        .infer_schema(Some(100))
        .finish()
        .unwrap();
    println!("{:?}", df);
    df
}