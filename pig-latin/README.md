# pig-latin

## Description
This project is a command-line tool that takes a sentence (or word) as an argument and translates it to pig latin. The translated sentence is printed to the terminal in pig latin. If you do not pass a sentence, the program will complain and nothing will be translated.

## Pig Latin Rules
There rules for speaking pig latin are described [here](https://web.ics.purdue.edu/~morelanj/RAO/prepare2.html). They are the following:

1) If a word starts with a consonant and a vowel, put the first letter of the word at the end of the word and add "ay."
2) If a word starts with two consonants move the two consonants to the end of the word and add "ay."
3) If a word starts with a vowel add the word "way" at the end of the word.

## How to Run 
1) cd into `pig-latin`
2) Run the following command: `cargo run -- --sentence "[INSERT SENTENCE HERE]"`
    - NOTE: You need the quotation marks if you are using translating more than one word, otherwise it will throw an error. 

If you need help or more information, please run the following command: `cargo run -- --help`

## Next Steps
- Allow the program to take a file as an input and output the translation in a different file
