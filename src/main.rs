mod shapes;

use chrono;
use fern::Dispatch;
use log::{info, warn};
use rand::Rng;
use std::io as std_io;
use std::io::{self, Write};
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use termion::raw::IntoRawMode;

// Set up termion
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, cursor};

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
    let mut handle = stdout.lock().into_raw_mode()?;

    write!(handle, "{}", termion::cursor::Hide)?;
    handle.flush()?;

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
                field[y as usize][x as usize] = 8; // ASCII for border
            } else {
                field[y as usize][x as usize] = 0; // ASCII for empty space
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
    let mut n_score: i32 = 0;
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
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            info!("Spawned new thread!");
            let stdin = std_io::stdin();

            for key in stdin.keys() {
                match key {
                    Ok(key) => {
                        info!("Input handling thread detected {:#?} input", key);
                        input_tx.send(key)?;
                        if key == Key::Char('q') {
                            game_over_clone.store(true, Ordering::SeqCst);
                            break;
                        }
                    }
                    Err(err) => {
                        info!("Input error: {}", err);
                        break;
                    }
                }
            }
            Ok(())
        })();
        if let Err(e) = result {
            // Handle the error here
            warn!("An error occurred: {}", e);
        }
    });

    loop
    /* Main game loop */
    {
        // TIMING =======================================
        sleep(Duration::from_millis(14));
        if n_speed_count == 255 {
            n_speed_count = 0;
        } else {
            n_speed_count += 1;
        }
        let b_force_down = n_speed_count == n_speed;

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

        // Force the piece down the playfield if it's time
        if b_force_down {
            // Update the difficulty every 50 pieces.
            n_speed_count = 0;
            n_piece_count += 1;
            if n_piece_count % 50 == 0 {
                if n_speed >= 10 {
                    n_speed -= 1;
                }
            }

            // Test if the piece can be moved down
            if does_piece_fit(
                n_current_piece,
                n_current_rotation,
                n_current_x,
                n_current_y,
                &field,
            ) {
                if n_current_y > 0 {
                    n_current_y -= 1
                };
            } else {
                // It cant! lock the piece in place
                for px in 0..4 {
                    for py in 0..4 {
                        if px == 0 || px == N_FIELD_WIDTH - 1 || py == N_FIELD_HEIGHT - 1 {
                            field[py as usize][px as usize] = 8; // ASCII for border
                        } else {
                            field[py as usize][px as usize] = 0; // ASCII for empty space
                        }
                    }
                }

                // Check for lines
                for py in 0..4 {
                    if n_current_y + py < N_FIELD_HEIGHT - 1 {
                        let mut b_line: bool = true;
                        for px in 0..N_FIELD_WIDTH - 1 {
                            b_line &= field[(n_current_y + py) as usize][px as usize] != '.' as u8;
                        }

                        if b_line {
                            // Remove lines, set to =
                            for px in 0..N_FIELD_WIDTH - 1 {
                                field[(n_current_y + py) as usize][px as usize] = 7;
                                v_lines.push(n_current_y + py);
                            }
                        }
                    }
                }

                n_score += 25;
                if !v_lines.is_empty() {
                    let lines = v_lines.len().min(10);
                    n_score += 2_i32.pow(lines as u32) * 100;
                }

                // Pick new piece at random
                n_current_x = N_FIELD_WIDTH / 2;
                n_current_y = 0;
                n_current_rotation = 0;
                n_current_piece = rng.gen_range(0..7);

                // If a piece does not fit straight away, game over!
                b_game_over = !does_piece_fit(
                    n_current_piece,
                    n_current_rotation,
                    n_current_x,
                    n_current_y,
                    &field,
                );
                if b_game_over {}
            }
        }

        // DISPLAY ===========================
        // Draw field
        for x in 0..N_FIELD_WIDTH {
            for y in 0..N_FIELD_HEIGHT {
                screen[(y + 2) as usize][(x + 2) as usize] = ' ';
            }
        }

        // Draw current piece
        for px in 0..4 {
            for py in 0..4 {
                if tetrominos[n_current_piece as usize].shape()
                    [rotate(px, py, n_current_rotation) as usize]
                    != '.'
                {
                    screen[(n_current_y + py + 2) as usize][(n_current_x + px + 2) as usize] =
                        (n_current_piece + 65) as char;
                }
            }
        }

        // Draw score
        write!(
            handle,
            "{}SCORE: {}",
            cursor::Goto(N_FIELD_WIDTH as u16 + 6, 2),
            n_score
        )?;

        write!(handle, "{}", clear::All)?;
        for (y, row) in field.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                write!(
                    handle,
                    "{}{}",
                    cursor::Goto(x as u16 + 2, y as u16 + 2),
                    LOOKUP[ch as usize]
                )?;
                handle.flush()?;
            }
        }
    } // END MAIN LOOP

    write!(handle, "{}", cursor::Show)?;
    handle.flush()?;
    Ok(())
}
