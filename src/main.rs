extern crate termion;

use std::io::{stdin, stdout, Write};
// use std::thread::sleep;
// use std::time::Duration;

// Set up termion
// use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor};

struct Window {
    origin: (usize, usize),
    height: usize,
    width: usize,
    contents: Vec<Vec<char>>,
}

impl Window {
    fn draw(&self) {
        for (y, row) in self.contents.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                // Add the window's origin to the coordinates
                let screen_x = self.origin.0 + x;
                let screen_y = self.origin.1 + y;
                write!(stdout(), "{}{}", cursor::Goto(screen_x as u16, screen_y as u16), ch).unwrap();
            }
        }
    }
    // Other methods
}

// fn clear_terminal(stdout: &mut RawTerminal<std::io::Stdout>) {
//     write!(stdout, "{}", termion::clear::All).unwrap();
//     stdout.flush().unwrap();
// }

fn create_empty_window(height: usize, width: usize) -> Vec<Vec<char>> {
    let mut contents = vec![vec![' '; width]; height];

    for row in &mut contents {
        row[0] = '|';
        row[width - 1] = '|';
    }
    for i in 0..width {
        contents[0][i] = '-';
        contents[height - 1][i] = '-';
    }
    contents
}

fn main() -> Result<(), std::io::Error> {
    let root_window = Window {
        origin: (0, 0),
        height: 25,
        width: 25,
        contents: create_empty_window(25, 25),
    };

    root_window.draw();

    Ok(())
}
