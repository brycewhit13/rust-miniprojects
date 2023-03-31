// Crates
use polars::prelude::*;

// Functions
pub fn load_data() -> DataFrame {
    CsvReader::from_path("data/fetal_health.csv")
        .expect("REASON")
        .has_header(true)
        .finish()
        .unwrap()
}

pub fn split_data(df: &DataFrame, test_size: f64, shuffle: bool, random_seed: u64) -> (DataFrame, DataFrame, Series, Series) {
    // Get the number of columns
    let (nrows, ncols) = df.shape();
    
    // Separate the features and target
    let features = df.select_by_range(0..ncols - 1).unwrap();
    let target = df.select_at_idx(ncols - 1).unwrap();

    // Split the features and target into training and testing sets
    let num_train = (nrows as f64 * test_size) as usize;
    let num_test = nrows - num_train;

    let xtrain = features.sample_n(num_train, false, shuffle, Some(random_seed)).unwrap();
    let xtest = features.sample_n(num_test, false, shuffle, Some(random_seed)).unwrap();
    let ytrain = target.sample_n(num_train, false, shuffle, Some(random_seed)).unwrap();
    let ytest = target.sample_n(num_test, false, shuffle, Some(random_seed)).unwrap();

    // Return the results
    (xtrain, xtest, ytrain, ytest)
}