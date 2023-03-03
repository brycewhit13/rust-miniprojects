// Import the crates
use linfa_trees::{DecisionTree, SplitQuality};
use linfa_datasets;
use linfa::prelude::{Fit, Predict, ToConfusionMatrix};
use rand::rngs::SmallRng;
use rand::SeedableRng;

// Function to load and split the dataset into train, validation and test sets
pub fn entropy_model(random_seed: u64) -> f32 {
    // Load the dataset
    let dataset = linfa_datasets::iris();

    // Split the dataset into training and testing
    let mut rng = SmallRng::seed_from_u64(random_seed);
    let (train, test) = dataset.shuffle(&mut rng).split_with_ratio(0.8);

    // instantiate the decision tree classifier
    let entropy_model = DecisionTree::params()
        .split_quality(SplitQuality::Entropy)
        .max_depth(Some(20))
        .min_weight_split(10.0)
        .min_weight_leaf(10.0);

    // Train the model
    let entropy_model = entropy_model.fit(&train).unwrap();

    // Get Predictions
    let predictions = entropy_model.predict(&test);

    // Get Accuracy
    let confusion_matrix = predictions.confusion_matrix(&test).unwrap();
    confusion_matrix.accuracy()
}

pub fn gini_model(random_seed: u64) -> f32 {
    // Load the dataset
    let dataset = linfa_datasets::iris();

    // Split the dataset into training and testing
    let mut rng = SmallRng::seed_from_u64(random_seed);
    let (train, test) = dataset.shuffle(&mut rng).split_with_ratio(0.8);

    // instantiate the decision tree classifier
    let entropy_model = DecisionTree::params()
        .split_quality(SplitQuality::Gini)
        .max_depth(Some(20))
        .min_weight_split(10.0)
        .min_weight_leaf(10.0);

    // Train the model
    let entropy_model = entropy_model.fit(&train).unwrap();

    // Get Predictions
    let predictions = entropy_model.predict(&test);

    // Get Accuracy
    let confusion_matrix = predictions.confusion_matrix(&test).unwrap();
    confusion_matrix.accuracy()
}
