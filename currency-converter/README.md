# currency-converter
## Overview
This command-line tool takes an input of two currencies and calls the [CurrencyConverter API](https://www.currencyconverterapi.com/) to return the conversion rate between the two currencies

## Get an API Key
This application requires an API key to use. You can get a free API key by signing up [here](https://free.currencyconverterapi.com/). This process make take 3-5 business days until they email you with your personal API key. 

Once you have an API key, create a file called `credentials.cfg` within the `currency_converter` folder. Then place you API key in the file with the following structure: `API_KEY = "[INSERT_API_KEY]"`. There should be nothing else in the file. Once this is done, you should be good to run the program yourself.

## How to use
1) cd into `currency_converter`
2) Determine which two currencies you want to compare. A complete list can be found [here](https://www.iban.com/currency-codes).
3) Run the following command `cargo run -- -o [CODE1] -d [CODE2]` where `CODE1` and `CODE2` are your chosen currency codes

Use the command `cargo run -- --help` for a description of how to run the program

