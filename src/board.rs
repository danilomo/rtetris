use crate::tetromino::Tetromino;

type BoolMatrix = Vec<Vec<bool>>;

#[derive(Clone)]
pub struct Board {
    pub matrix: BoolMatrix,
    pub tetromino: Tetromino,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Movement {
    Down,
    Left,
    Right,
    RotateLeft,
    RotateRight,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        let range_rows = 0..rows;

        let matrix: BoolMatrix = range_rows
            .map(|i| {
                let range_cols = 0..cols;

                range_cols
                    .map(|j| match (i, j) {
                        (0, _) => true,
                        (_, 0) => true,
                        (a, b) if a == rows - 1 || b == cols - 1 => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect();

        Board {
            matrix,
            tetromino: Tetromino::random(),
        }
    }

    pub fn apply_movement(&mut self, movement: Movement) -> bool {
        let previous_position = self.tetromino;

        match movement {
            Movement::Down => {
                self.tetromino.move_down();
            }
            Movement::Left => {
                self.tetromino.move_left();
            }
            Movement::Right => {
                self.tetromino.move_right();
            }
            Movement::RotateLeft => {
                self.tetromino.rotate_left();
            }
            Movement::RotateRight => {
                self.tetromino.rotate_left();
            }
        };

        if self.overlaps() {
            self.tetromino = previous_position;
            return false;
        }

        true
    }

    pub fn overlaps(&self) -> bool {
        let rotation = self.tetromino.actual_rotation();
        for i in 0..4 {
            for j in 0..4 {
                let (i_board, j_board) = (self.tetromino.i + i, self.tetromino.j + j);

                if j_board as usize >= self.matrix[i_board as usize].len() {
                    continue;
                }

                if rotation[i as usize][j as usize]
                    && self.matrix[i_board as usize][j_board as usize]
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn merge(&mut self) {
        let rotation = self.tetromino.actual_rotation();
        for i in 0..4 {
            for j in 0..4 {
                let (i_board, j_board) = (self.tetromino.i + i, self.tetromino.j + j);

                if j_board as usize >= self.matrix[i_board as usize].len() {
                    continue;
                }

                self.matrix[i_board as usize][j_board as usize] = rotation[i as usize][j as usize]
                    || self.matrix[i_board as usize][j_board as usize];
            }
        }
    }

    pub fn to_str(&self) -> String {
        let mut copy = self.clone();
        copy.merge();
        let mut result = String::new();

        for row in copy.matrix {
            for column in row {
                let char = if column { 'X' } else { '_' };
                result.push(char);
            }
            result.push('\n');
        }

        result
    }

    pub fn blocks(&self) -> Vec<(isize, isize)> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(i, vec)| {
                vec.iter()
                    .enumerate()
                    .filter(|(_, val)| **val)
                    .map(move |(j, _)| (i as isize, j as isize))
            })
            .collect::<Vec<_>>()
    }

    pub fn check_completed_rows(&mut self) -> bool {
        let len = self.matrix.len() - 2;
        let mut return_val = false;

        for i in (2..=len).rev() {
            if self.completed_row(i) {
                self.move_rows(i);
                self.check_completed_rows();
                return_val = true;
            }
        }

        return_val
    }

    fn move_rows(&mut self, row: usize) {
        for i in (3..=row).rev() {
            let len = self.matrix[i].len() - 1;
            for j in 1..len {
                self.matrix[i][j] = self.matrix[i-1][j];
            }
        }
    }

    fn completed_row(&self, i: usize) -> bool {
        for val in &self.matrix[i] {
            if ! *val {
                return false;
            }
        }

        true
    }
}
