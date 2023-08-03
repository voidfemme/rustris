#[derive(Debug)]
pub struct Tetromino {
    shape: Vec<char>,
    width: usize,
    height: usize,
}

impl Tetromino {
    pub fn shape(&self) -> &Vec<char> {
        &self.shape
    }
}

pub fn get_shapes() -> Vec<Tetromino> {
    vec![
        Tetromino {
            shape: "..X...X...X...X.".chars().collect(),
            width: 4,
            height: 4,
        },
        Tetromino {
            shape: "........XX..XX..".chars().collect(),
            width: 4,
            height: 4,
        },
        Tetromino {
            shape: "........XXX..X..".chars().collect(),
            width: 4,
            height: 4,
        },
        Tetromino {
            shape: "........X...X..XX.".chars().collect(),
            width: 4,
            height: 4,
        },
        Tetromino {
            shape: ".........X...X..XX".chars().collect(),
            width: 4,
            height: 4,
        },
        Tetromino {
            shape: "........XX...XX..".chars().collect(),
            width: 4,
            height: 4,
        },
        Tetromino {
            shape: "........XX..XX...".chars().collect(),
            width: 4,
            height: 4,
        },
    ]
}
