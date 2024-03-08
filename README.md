# Tic Tac Toe

A text-based tic tac toe game written in Rust

I wrote this as an excercise to learn Rust. If you have any comments or proposals for improvements, please don't hesitate to file an issue.

The game can be played with a field size of 2 by 2 up to 30 by 30. Default field size is 4.

## Run the game

In order to run the game you need to have [Rust](https://www.rust-lang.org/tools/install).

```sh
# in a directory of your choice
git clone https://github.com/binChris/tictactoe
cd tictactoe
# run with default settings
cargo run
# let computer begin and set field size to the classic 3x3
cargo run -- -c -d 3
```