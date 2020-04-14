use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

#[derive(Debug, Copy, Clone)]
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

    (0..y)
        .map(|_| (0..x).map(|_| rng.gen::<Cell>()).collect::<Line>())
        .collect()
}

fn neighbor_count(board: &Board, (x, y): (usize, usize)) -> usize {
    let neighbors = [
        // top
        (x.checked_sub(1), y.checked_sub(1)),
        (Some(x), y.checked_sub(1)),
        (x.checked_add(1), y.checked_sub(1)),
        // sides
        (x.checked_sub(1), Some(y)),
        (x.checked_add(1), Some(y)),
        // bottom
        (x.checked_sub(1), y.checked_add(1)),
        (Some(x), y.checked_add(1)),
        (x.checked_add(1), y.checked_add(1)),
    ];

    neighbors
        .iter()
        .filter(|(x, y)| x.is_some() && y.is_some())
        .map(|(x, y)| (x.unwrap(), y.unwrap()))
        .filter(|(x, y)| board.get(*y).is_some() && board[*y].get(*x).is_some())
        .map(|(x, y)| board[y][x])
        .fold(
            0,
            |acc, cell| if let Cell::Live = cell { acc + 1 } else { acc },
        )
}

fn change_cell_state(board: &Board, (x, y): (usize, usize)) -> Cell {
    let count = neighbor_count(board, (x, y));

    match board[y][x] {
        Cell::Live => match count {
            2..=3 => Cell::Live,
            _ => Cell::Dead,
        },
        _ => match count {
            3 => Cell::Live,
            _ => Cell::Dead,
        },
    }
}

fn gen_2d_range(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..y).flat_map(move |a| (0..x).map(move |b| (b, a)))
}

pub fn next_gen(board: Board) -> Board {
    let (x, y) = termion::terminal_size().unwrap();
    let mut new_board = vec![vec![Cell::Dead; x as usize]; y as usize];
    for (x, y) in gen_2d_range(x as usize, y as usize) {
        new_board[y][x] = change_cell_state(&board, (x, y));
    }

    new_board
}
