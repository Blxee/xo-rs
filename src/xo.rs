use std::fmt;

use rand::{rng, seq::IndexedRandom};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct XOGame {
    grid: [[Cell; 3]; 3],
    turn: Symbol,
    state: GameState,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Symbol(Symbol),
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Symbol {
    X,
    O,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum GameState {
    Playing,
    Over(Option<Symbol>),
}

#[derive(Serialize, Deserialize)]
pub struct Move {
    x: u8,
    y: u8,
}

impl XOGame {
    pub fn new() -> Self {
        let mut rng = rng();
        let turn = *[Symbol::X, Symbol::O].choose(&mut rng).unwrap();
        Self {
            grid: [[Cell::Empty; 3]; 3],
            turn,
            state: GameState::Playing,
        }
    }

    pub fn play(&mut self, Move { x, y }: Move) -> Result<(), &'static str> {
        let (mut x, mut y) = (x as usize, y as usize);

        if matches!(self.state, GameState::Over(_)) {
            return Err("The game is over!");
        }

        if !matches!((x, y), (1..=3, 1..=3)) {
            return Err("This cell is out of the grid bounds!");
        }

        (x, y) = (x - 1, y - 1);

        if matches!(self.grid[y][x], Cell::Symbol(_)) {
            return Err("This cell is already occupied!");
        }

        self.grid[y][x] = Cell::Symbol(self.turn);

        self._update_state();

        self.turn = match self.turn {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
        };

        Ok(())
    }

    fn _update_state(&mut self) {
        const WIN_STATES: [[(usize, usize); 3]; 8] = [
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for state in WIN_STATES {
            for symbol in [Symbol::X, Symbol::O] {
                if state
                    .iter()
                    .all(|&(x, y)| self.grid[y][x] == Cell::Symbol(symbol))
                {
                    self.state = GameState::Over(Some(symbol));
                    return;
                }
            }
        }

        if self.grid.iter().flatten().all(|&cell| cell != Cell::Empty) {
            self.state = GameState::Over(None);
        }
    }

    pub fn get_turn(&self) -> Symbol {
        self.turn
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }
}

impl TryFrom<&str> for Move {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split_whitespace().collect();
        let [x, y] = split.as_slice() else {
            return Err("A move should have a row and a column (ex: \"1 2\")");
        };
        let (Ok(x), Ok(y)) = (x.parse(), y.parse()) else {
            return Err("row and col should a valid int");
        };
        if !(1..=3).contains(&x) || !(1..=3).contains(&y) {
            return Err("row and col should be and int between [1..3]");
        }
        Ok(Self { x, y })
    }
}

impl fmt::Display for XOGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "+-----+")?;
        for row in self.grid {
            write!(f, "\n|")?;
            for cell in row {
                write!(
                    f,
                    "{}|",
                    match cell {
                        Cell::Empty => '.',
                        Cell::Symbol(Symbol::X) => 'X',
                        Cell::Symbol(Symbol::O) => 'O',
                    }
                )?;
            }
            write!(f, "")?;
        }
        write!(f, "\n+-----+")
    }
}
