# iris-classification

## Description

This project compares the effectiveness of two models in their ability to classify the well known iris dataset. This dataset contains 4 features of measurements for flowers that are classified into 3 unique classes. The first model uses [Entropy](https://towardsdatascience.com/entropy-how-decision-trees-make-decisions-2946b9c18c8) to make splits in the decision tree and the second uses [Gini Impurity](https://www.learndatasci.com/glossary/gini-impurity/#:~:text=a%20simple%20dataset-,What%20is%20Gini%20Impurity%3F,nodes%20to%20form%20the%20tree.). The accuracy for each model is output to the console and the user has the ability to change the random seed used in creating each model. This was accomplished using the [linfa](https://crates.io/crates/linfa), [linfa-trees](https://crates.io/crates/linfa-datasets), and [linfa-datasets](https://crates.io/crates/linfa-trees) crates.

## How to Run

1) cd into `iris-classification`
2) Run the following command: `cargo run -- --random-seed [RANDOM_SEED]`
    - NOTE: The random-seed defaults to 1 if nothing is passed and must be an integer

If you need help or more information, please run the following command: `cargo run -- --help`