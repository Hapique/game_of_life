use game_of_life::{generate_board, next_gen};
use std::io::Read;
use std::io::{stdout, Write};
use std::{thread, time};
use termion::async_stdin;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

const QUIT_KEY: u8 = b'q';
const SLEEP_DURATION: u64 = 70;

fn main() {
    let mut stdin = async_stdin().bytes();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let duration = time::Duration::from_millis(SLEEP_DURATION);
    let mut board = generate_board();

    loop {
        let event = stdin.next();
        if let Some(Ok(QUIT_KEY)) = event {
            break;
        }

        for (index, line) in board.clone().iter().enumerate() {
            let line = line.iter().map(|x| x.to_string()).collect::<String>();
            write!(stdout, "{}{}", termion::cursor::Goto(1, index as u16), line).unwrap();
        }
        stdout.flush().unwrap();

        thread::sleep(duration);

        board = next_gen(board);
    }
}
