# Integer Converison

## Description

This is a CLI that takes an integer as an input and prints out the integer in binary, hex, and octal. For those who may not remember, those are a base 2, base 16, and base 8 counting system respectively. The user has the option to print out all the results or just the ones they specify. Please read below for more info about the arguments.

## How to run

There are four arguments you need to be aware of:

1. `-i`: Allows you to input the integer you want to convert. By default this is set to 15 if nothing is passed.
2. `-b`: A flag that lets you enable the binary equivalent to be returned. By default is is set to false.
3. `-n`: A flag that lets you enable the hex equivalent to be returned. By default is is set to false. This could not be set to `-h` because that is reserved for the help function.
4. `-o`: A flag that lets you enable the octal equivalent to be returned. By default is is set to false.

To run the program, do the following:

1. cd into the `int_conversion` directory
2. `cargo run -- -i 15 -b -n -o`