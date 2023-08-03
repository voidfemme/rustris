mod shapes;

use chrono;
use fern::Dispatch;
use log::info;
use rand::Rng;
use std::io;
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

// Set up termion
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use shapes::get_shapes;

const N_FIELD_WIDTH: u8 = 12;
const N_FIELD_HEIGHT: u8 = 18;
const N_SCREEN_WIDTH: u8 = 80;
const N_SCREEN_HEIGHT: u8 = 30;
const LOOKUP: [char; 9] = [' ', 'A', 'B', 'C', 'D', 'F', 'G', '=', '#'];

fn setup_logger(log_file: &str) -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}:{} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(log_file)?)
        .apply()?;
    Ok(())
}

fn rotate(px: u8, py: u8, r: u8) -> u8 {
    match r % 4 {
        0 => return py * 4 + px,
        1 => return 12 + py - (px * 4),
        2 => return 15 - (py * 4) - px,
        3 => return 3 - py + (px * 4),
        _ => return 0,
    }
}

fn does_piece_fit(
    n_tetromino: u8,
    n_rotation: u8,
    n_pos_x: u8,
    n_pos_y: u8,
    field: &Vec<Vec<u8>>,
) -> bool {
    let tetrominos = get_shapes();
    for px in 0..4 {
        for py in 0..4 {
            // Get index into piece
            let pi = rotate(px, py, n_rotation);

            // Check that test is in bounds. Note out of bounds does not necessarily mean a fail,
            // as the long vertical piece can have cells that lie outside the boundary, so we'll
            // just ignore them.
            if n_pos_x + px < N_FIELD_WIDTH {
                if n_pos_y + py < N_FIELD_HEIGHT {
                    // In Bounds so do collision Check
                    if tetrominos[n_tetromino as usize].shape()[pi as usize] != '.'
                        && field[(n_pos_y + py) as usize][(n_pos_x + px) as usize] != 'X' as u8
                    {
                        return false; // Fail on first hit
                    }
                }
            }
        }
    }
    true
}

fn main() -> Result<(), std::io::Error> {
    setup_logger("output.log").expect("Failed to initialize logger");
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode()?;

    // Create play field buffer
    let mut field: Vec<Vec<u8>> =
        vec![vec![' ' as u8; N_FIELD_WIDTH as usize]; N_FIELD_HEIGHT as usize];

    // Create screen buffer
    let mut screen: Vec<Vec<char>> =
        vec![vec![' '; N_SCREEN_WIDTH as usize]; N_SCREEN_HEIGHT as usize];

    // Create Tetris play field and border
    for x in 0..N_FIELD_WIDTH {
        for y in 0..N_FIELD_HEIGHT {
            if x == 0 || x == N_FIELD_WIDTH - 1 || y == N_FIELD_HEIGHT - 1 {
                field[y as usize][x as usize] = '#' as u8; // ASCII for border
            } else {
                field[y as usize][x as usize] = ' ' as u8; // ASCII for empty space
            }
        }
    }

    // Game logic
    let tetrominos = get_shapes();
    let mut n_current_piece: u8 = 0;
    let mut n_current_rotation: u8 = 0;
    let mut n_current_x: u8 = N_FIELD_WIDTH / 2;
    let mut n_current_y: u8 = 0;
    let mut n_speed = 20;
    let mut n_speed_count: u8 = 0;
    let mut b_rotate_hold: bool = true;
    let mut n_piece_count = 0;
    let mut n_score = 0;
    let mut v_lines = Vec::<u8>::new();
    let mut rng = rand::thread_rng();
    let mut b_game_over: bool = false;

    // Create a thread for handling input
    let (tx, rx) = mpsc::channel();
    let input_tx = tx.clone();
    let game_over = Arc::new(AtomicBool::new(false));
    let game_over_clone = Arc::clone(&game_over);

    // Spawn a thread to handle user input
    thread::spawn(move || {
        info!("Spawned new thread!");
        let stdin = io::stdin();
        let mut stdout = io::stdout().into_raw_mode().unwrap();

        for key in stdin.keys() {
            match key {
                Ok(key) => {
                    info!("Input handling thread detected {:#?} input", key);
                    input_tx.send(key).unwrap();
                    if key == Key::Char('q') {
                        game_over_clone.store(true, Ordering::SeqCst);
                        break;
                    }
                }
                Err(err) => {
                    writeln!(stdout, "Input error: {}", err).unwrap();
                    break;
                }
            }
        }
    });

    loop
    /* Main game loop */
    {
        // TIMING =======================================
        sleep(Duration::from_millis(10));
        if n_speed_count == 255 {
            n_speed_count = 0;
        } else {
            n_speed_count += 1;
        }
        let b_force_down = n_speed_count == n_speed;
        if b_force_down == true {
            println!("Speeding up!");
            println!("{}", n_speed_count);
        }

        // INPUT ========================================
        match rx.try_recv() {
            Ok(key) => match key {
                Key::Char('d') => {
                    if does_piece_fit(
                        n_current_piece,
                        n_current_rotation,
                        n_current_x + 1,
                        n_current_y,
                        &field,
                    ) {
                        n_current_x += 1;
                        info!("'d' pressed; n_current_x = {n_current_x}");
                    }
                }
                Key::Char('a') => {
                    if does_piece_fit(
                        n_current_piece,
                        n_current_rotation,
                        n_current_x - 1,
                        n_current_y,
                        &field,
                    ) {
                        n_current_x -= 1;
                        info!("'a' pressed; n_current_x = {n_current_x}");
                    }
                }
                Key::Char('s') => {
                    if does_piece_fit(
                        n_current_piece,
                        n_current_rotation,
                        n_current_x,
                        n_current_y + 1,
                        &field,
                    ) {
                        n_current_y -= 1;
                        info!("'s' pressed; n_current_y = {n_current_y}");
                    }
                }
                Key::Char(' ') => {
                    if b_rotate_hold
                        && does_piece_fit(
                            n_current_piece,
                            n_current_rotation + 1,
                            n_current_x,
                            n_current_y,
                            &field,
                        )
                    {
                        // Rotate, but latch to stop wild spinning
                        n_current_rotation += 1;
                        info!("<Space> pressed; n_current_rotation = {n_current_rotation}");
                        b_rotate_hold = false;
                    } else {
                        b_rotate_hold = true;
                    }
                }
                _ => break,
            },
            Err(_e) => {
                // No message this time, or an error occurred
                // Just continue with the game loop
            }
        }
    }
    info!("Game state:\nn_current_x = {n_current_x}\nn_current_y = {n_current_y}\nn_current_rotation = {n_current_rotation}");
    for (y, row) in field.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            write!(
                stdout,
                "{}{}",
                cursor::Goto(x as u16 + 1, y as u16 + 1),
                &ch
            )?;
        }
    }

    Ok(())
}
