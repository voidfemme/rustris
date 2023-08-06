pub struct Tetromino {
    shape: Vec<u8>,
}

impl Tetromino {
    pub fn shape(&self) -> &Vec<u8> {
        &self.shape
    }
}

pub fn get_shapes() -> Vec<Tetromino> {
    vec![
        Tetromino {
            shape: vec![10, 10, 2, 10, 10, 10, 2, 10, 10, 10, 2, 10, 10, 10, 2, 10],
        }, // Straight line
        // ..X.
        // ..X.
        // ..X.
        // ..X.
        Tetromino {
            shape: vec![10, 10, 10, 10, 10, 3, 3, 10, 10, 3, 3, 10, 10, 10, 10, 10],
        }, // box
        // ....
        // ....
        // XX..
        // XX..
        Tetromino {
            shape: vec![10, 10, 10, 10, 10, 10, 10, 10, 4, 4, 4, 10, 10, 4, 10, 10],
        }, // Tee
        // ....
        // ....
        // XXX.
        // .X..
        Tetromino {
            shape: vec![10, 10, 10, 10, 10, 10, 4, 10, 10, 10, 4, 10, 10, 4, 4, 10],
        }, // Right Ell
        // ....
        // ..X.
        // ..X.
        // .XX.
        Tetromino {
            shape: vec![10, 10, 10, 10, 10, 5, 10, 10, 10, 5, 10, 10, 10, 5, 5, 10],
        }, // Left Ell
        // ....
        // .X..
        // .X..
        // .XX.
        Tetromino {
            shape: vec![10, 10, 10, 10, 10, 10, 10, 10, 6, 6, 10, 10, 10, 6, 6, 10],
        }, // Ess
        // ....
        // ....
        // XX..
        // .XX.
        Tetromino {
            shape: vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 7, 7, 10, 7, 7, 10, 10],
        }, // Zee
           // ....
           // ....
           // .XX.
           // XX..
    ]
}
