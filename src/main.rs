//! A text-based tic tac toe game written in Rust

use tictactoe::{Board, Cell};

const HELP: &str = "\
tictactoe

USAGE:
  tictactoe [OPTIONS]

OPTIONS:
  -h, --help     Prints help information
  -d [n]         Board dimension (default: 3)
  -c             Computer has first move
  -o             Player uses O instead of X (which is the default)
";

#[derive(Debug)]
struct AppArgs {
    dimension: usize,
    computer_begins: bool,
    player_uses_o: bool,
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let human_uses = if args.player_uses_o { Cell::O } else { Cell::X };
    let mut board = Board::build(args.dimension, human_uses).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });

    // loop to display the board, player and computer moves
    let mut human_move = !args.computer_begins;
    if args.computer_begins {
        println!("Computer has the first move.")
    }
    let won = loop {
        if human_move {
            println!("{}", board);
            if let Some(won) = board.user_move() {
                break won;
            }
        }
        human_move = true;
        if let Some(won) = board.computer_move() {
            break won;
        }
    };
    println!("{}\n", won);
    println!("{}", board);
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        dimension: pargs.opt_value_from_str("-d")?.unwrap_or(4),
        computer_begins: pargs.contains("-c"),
        player_uses_o: pargs.contains("-o"),
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        println!("Invalid arguments: {:?}.\n", remaining);
        print!("{}", HELP);
        std::process::exit(1);
    }

    Ok(args)
}
