use std::fmt;

use regex::Regex;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Cell {
    X,
    O,
    Blank,
}

impl Cell {
    fn opponent(&self) -> Cell {
        match self {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            _ => panic!("other called on Blank"),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Cell::X => "X",
            Cell::O => "O",
            Cell::Blank => " ",
        };
        let _ = write!(f, "{}", s);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    dim: usize,
    cells: Vec<Cell>,
    win_lines: Vec<Vec<usize>>,
    human_uses: Cell,
    moves: usize,
}

#[derive(Debug, PartialEq)]
pub enum GameOver {
    HumanWon,
    ComputerWon,
    Tie,
}

impl fmt::Display for GameOver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameOver::HumanWon => write!(f, "You won!"),
            GameOver::ComputerWon => write!(f, "Computer won!"),
            GameOver::Tie => write!(f, "It's a tie!"),
        }
    }
}

impl Board {
    /// Create a new board with the given number of rows and columns
    pub fn build(dim: usize, human_uses: Cell) -> Result<Board, &'static str> {
        assert!(human_uses != Cell::Blank);
        if !(2..=30).contains(&dim) {
            return Err("Invalid board dimension, must be between 2 and 30");
        }
        Ok(Board {
            dim,
            cells: vec![Cell::Blank; dim * dim],
            win_lines: Board::win_lines(dim),
            human_uses,
            moves: 0,
        })
    }

    /// Create a board from a string containing 'X', 'O' and '-' in lines. Empty lines are ignored.
    #[cfg(test)]
    fn from_string(s: &str, dim: usize, human_uses: Cell) -> Result<Board, &'static str> {
        let s = s.trim().replace(['\r', '\n', ' '], "");
        let mut moves = 0;
        let cells = s
            .chars()
            .map(|c| match c {
                '-' => Cell::Blank,
                'X' => {
                    moves += 1;
                    Cell::X
                }
                'O' => {
                    moves += 1;
                    Cell::O
                }
                _ => panic!("Invalid character in board string"),
            })
            .collect();

        Ok(Board {
            dim,
            cells,
            win_lines: Board::win_lines(dim),
            human_uses,
            moves,
        })
    }

    /// Get the list of winning lines
    fn win_lines(dim: usize) -> Vec<Vec<usize>> {
        let mut win_lines = Vec::new();
        for x in 0..dim {
            let mut line = Vec::new();
            for y in 0..dim {
                line.push(x + y * dim);
            }
            win_lines.push(line);
        }
        for y in 0..dim {
            let mut line = Vec::new();
            for x in 0..dim {
                line.push(x + y * dim);
            }
            win_lines.push(line);
        }
        let mut line = Vec::new();
        for x in 0..dim {
            line.push(x + x * dim);
        }
        win_lines.push(line);
        let mut line = Vec::new();
        for x in 0..dim {
            line.push(x + (dim - 1 - x) * dim);
        }
        win_lines.push(line);
        win_lines
    }

    /// Set the cell at the given coordinates and maintain the 'moves' count.
    ///
    /// Returns an error if the cell is already occupied
    fn set_cell(&mut self, x: usize, y: usize, cell: Cell) -> Result<(), &'static str> {
        assert!(x < self.dim);
        assert!(y < self.dim);
        if self.get_cell(x, y) != Cell::Blank {
            return Err("Cell already taken");
        };
        self.cells[x + y * self.dim] = cell;
        self.moves += 1;
        Ok(())
    }

    /// Get the cell at the given coordinates.
    fn get_cell(&self, x: usize, y: usize) -> Cell {
        assert!(x < self.dim);
        assert!(y < self.dim);
        self.cells[x + y * self.dim]
    }

    /// Accept input from the user and make a move
    pub fn user_move(&mut self) -> Option<GameOver> {
        let mut x: usize;
        let mut y: usize;
        loop {
            (x, y) = self.accept_input();
            if let Err(e) = self.set_cell(x, y, self.human_uses) {
                println!("{}", e);
                continue;
            }
            break;
        }
        self.check_game_over(x, y, self.human_uses)
    }

    pub fn computer_move(&mut self) -> Option<GameOver> {
        let comp_uses = self.human_uses.opponent();
        let (x, y) = self.best_move(comp_uses);
        self.set_cell(x, y, comp_uses).unwrap();
        self.check_game_over(x, y, comp_uses)
    }

    /// Find the best next move.
    //
    // Fills a field by row / column / diagonal with a sum of:
    // - if cell empty: 1
    //   - if line does not contain opponent piece: dim - empty on line
    fn best_move(&mut self, cell: Cell) -> (usize, usize) {
        let opponent = cell.opponent();
        let mut wins: Vec<usize> = self
            .cells
            .iter()
            .map(|c| if *c == Cell::Blank { 1 } else { 0 })
            .collect();
        'outer: for win_line in self.win_lines.iter() {
            let mut blanks: Vec<usize> = Vec::new();
            for idx in win_line {
                let c = self.cells[*idx];
                if c == opponent {
                    continue 'outer;
                }
                if c == Cell::Blank {
                    blanks.push(*idx);
                }
            }
            if blanks.len() == 1 {
                // win in 1 move, no need to continue
                return (blanks[0] % self.dim, blanks[0] / self.dim);
            }
            let moves = self.dim + 1 - blanks.len();
            for idx in blanks {
                wins[idx] += moves;
            }
        }
        // check for 1 move lose
        'outer: for win_line in self.win_lines.iter() {
            let mut blank = 0;
            let mut count = 0;
            for idx in win_line {
                let c = self.cells[*idx];
                if c == cell {
                    continue 'outer;
                }
                if c == Cell::Blank {
                    if count > 0 {
                        continue 'outer;
                    }
                    blank = *idx;
                    count += 1;
                }
            }
            if count == 1 {
                return (blank % self.dim, blank / self.dim);
            }
        }
        // determine move from wins calculation
        let max = wins
            .iter()
            .enumerate()
            .max_by_key(|(_idx, &val)| val)
            .unwrap()
            .0;
        (max % self.dim, max / self.dim)
    }

    /// Accept input from the user and validate it. On error, print an error message and loop.
    fn accept_input(&mut self) -> (usize, usize) {
        loop {
            println!("Enter x and y separated by a space: ");
            let mut input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut input) {
                println!("Failed to read line: {}", e);
                continue;
            }
            let re = Regex::new(r"^(\d+) (\d+)").unwrap();
            let cap = re.captures(&input);
            if cap.is_none() {
                println!("Invalid input: {}", input);
                continue;
            }
            let cap = cap.unwrap();
            let row: usize = cap[1].parse().unwrap();
            let col: usize = cap[2].parse().unwrap();
            if row < 1 || col < 1 || row > self.dim || col > self.dim {
                println!("Invalid coordinates");
                continue;
            }
            return (row - 1, col - 1);
        }
    }

    /// Check if the game is over and return the state:
    /// HumanWon, ComputerWon, Tie or None
    ///
    /// The game is over if one player has occupied cells in a full line (row, column or diagonal).
    /// If all cells are occupied, it's a tie.
    ///
    /// To reduce the complexity of the calculation, the function receives coordinates and player of the last move,
    /// as only the last move can lead to a win.
    fn check_game_over(&self, x: usize, y: usize, cell: Cell) -> Option<GameOver> {
        let idx = x + y * self.dim;
        let win_lines = self.win_lines.iter().filter(|v| v.contains(&idx));
        'outer: for win_line in win_lines {
            for idx in win_line {
                if self.cells[*idx] != cell {
                    continue 'outer;
                }
            }
            return self.won(cell);
        }
        if self.moves == self.dim * self.dim {
            Some(GameOver::Tie)
        } else {
            None
        }
    }

    // Translates the winning cell type (X or O) into the game over state
    fn won(&self, c: Cell) -> Option<GameOver> {
        if c == self.human_uses {
            Some(GameOver::HumanWon)
        } else {
            Some(GameOver::ComputerWon)
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sep = "+---".repeat(self.dim) + "+";
        let _ = writeln!(f, "{}", sep);
        for y in 0..self.dim {
            for x in 0..self.dim {
                let _ = write!(f, "| {} ", self.get_cell(x, y));
            }
            let _ = writeln!(f, "|");
            let _ = writeln!(f, "{}", sep);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tie() {
        let board = Board::from_string(
            "
            XXO
            OXX
            XOO",
            3,
            Cell::X,
        )
        .unwrap();
        assert_eq!(board.check_game_over(0, 0, Cell::X).unwrap(), GameOver::Tie);
    }

    #[test]
    fn test_check_game_over_win() {
        let tests = [
            (
                "row 1", // name
                "
                XXX
                OXX
                XOO", // board
                (2, 0),  // last move for X
            ),
            (
                "row 2",
                "
                OXO
                XXX
                OOX",
                (0, 1),
            ),
            (
                "row 3",
                "
                OXO
                OOX
                XXX",
                (1, 2),
            ),
            (
                "col 1",
                "
                XXO
                XOX
                XOO",
                (0, 0),
            ),
            (
                "dia 1",
                "
                XXO
                OXX
                XOX",
                (0, 0),
            ),
            (
                "dia 2",
                "
                OXX
                XXO
                XOO",
                (0, 2),
            ),
        ];
        for (name, board, (x, y)) in tests {
            let board = Board::from_string(board, 3, Cell::X).unwrap();
            assert_eq!(
                board.check_game_over(x, y, Cell::X).unwrap(),
                GameOver::HumanWon,
                "test case {} failed",
                name
            );
        }
    }

    #[test]
    fn test_best_move() {
        let tests = [
            (
                "first move center",
                // on an empty board, the best move is the center
                "
---
---
---",
                (1, 1),
            ),
            (
                "avoid loss",
                // need to avoid a loss if there is no winning move
                "
X--
XO-
---",
                (0, 2),
            ),
            (
                "win over avoid loss",
                // need to avoid a loss if there is no winning move
                "
X--
XO-
-O-",
                (1, 0),
            ),
        ];
        for (name, board, (x, y)) in tests {
            let mut board = Board::from_string(board, 3, Cell::X).unwrap();
            assert_eq!(
                board.best_move(Cell::O),
                (x, y),
                "test case '{}' failed",
                name
            );
        }
    }

    #[test]
    fn game_is_not_over() {
        let board = Board::from_string(
            "
            XXO
            O-X
            XOO",
            3,
            Cell::X,
        )
        .unwrap();
        assert!(board.check_game_over(0, 2, Cell::X).is_none());
    }
}
