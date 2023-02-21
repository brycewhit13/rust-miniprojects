# digital-clock

## Descriptions
This program is simply a digital clock within your terminal. It will always show you your local time and continuously update until you tell the program to stop. See the `How to Quit` section for more details on how to do that. 

## How to Run
1) cd into `digital-clock`
2) run the following command: `cargo run`

## How to Quit
The program uses an infinite loop while running so it will keep going until you stop it. The easiset way to do this is by hitting the `q` character on your keyboard, and the program will stop automatically with no errors. You can also use `Ctrl+c` to stop the program in the terminal, but this will cause a `KeyboardInterruptError`

## Next Steps
I hope to implement the following two changes moving forward:
1) Add functionality for any timezone
2) Add a GUI to the program