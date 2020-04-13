use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

#[derive(Debug)]
pub enum Cell {
    Live,
    Dead,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if let Cell::Live = self { 'O' } else { ' ' })
    }
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0, 2) {
            0 => Cell::Live,
            _ => Cell::Dead,
        }
    }
}

type Line = Vec<Cell>;
type Board = Vec<Line>;

pub fn generate_board() -> Board {
    let mut rng = rand::thread_rng();
    let (x, y) = termion::terminal_size().unwrap();
    let mut board: Board = vec![];
    for _ in 0..y {
        let line: Line = (0..x).map(|_| rng.gen::<Cell>()).collect::<Line>();
        board.push(line);
    }

    board
}
