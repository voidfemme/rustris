struct Window {
    origin: (usize, usize),
    height: usize,
    width: usize,
    contents: Vec<Vec<char>>,
}

impl Window {
    fn draw(&self) -> Result<(), std::io::Error> {
        for (y, row) in self.contents.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                // Add the window's origin to the coordinates
                let screen_x = self.origin.0 + x + 1;
                let screen_y = self.origin.1 + y + 1;
                write!(
                    stdout(),
                    "{}{}",
                    cursor::Goto(screen_x as u16, screen_y as u16),
                    ch
                )?;
            }
        }
        Ok(())
    }

    fn update(
        &mut self,
        x: usize,
        y: usize,
        new_contents: &[Vec<char>],
    ) -> Result<(), std::io::Error> {
        // check if the coordinates are within bounds
        if x + new_contents.len() > self.contents.len()
            || y + new_contents[0].len() > self.contents[0].len()
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "coordinates are out of bounds",
            ));
        }

        // copy the new contents into the window's contents
        for (i, row) in new_contents.iter().enumerate() {
            for (j, &char) in row.iter().enumerate() {
                self.contents[x + i][y + j] = char;
            }
        }
        Ok(())
    }
}

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
    // set the corners
    contents[0][0] = '#';
    contents[0][width - 1] = '#';
    contents[height - 1][0] = '#';
    contents[height - 1][width - 1] = '#';
    contents
}

