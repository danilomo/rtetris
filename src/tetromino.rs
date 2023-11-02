use crate::patterns::{self, Pattern};
extern crate rand;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Tetromino {
    pub i: isize,
    pub j: isize,
    patterns: &'static [Pattern],
    selected_pattern: usize,
}

const LEFT: isize = -1;
const RIGHT: isize = 1;

#[allow(non_camel_case_types)]
pub enum Type {
    I_BLOCK,
    O_BLOCK,
    L_BLOCK,
    J_BLOCK,
    T_BLOCK,
    S_BLOCK,
    Z_BLOCK,
}

impl Type {
    fn random_type() -> Type {
        let mut rng = rand::thread_rng();

        match rng.gen_range(0..6) {
            0 => Type::I_BLOCK,
            1 => Type::O_BLOCK,
            2 => Type::L_BLOCK,
            3 => Type::J_BLOCK,
            4 => Type::T_BLOCK,
            5 => Type::S_BLOCK,
            _ => Type::Z_BLOCK,
        }
    }
}

fn type_to_patterns(t_type: Type) -> &'static [Pattern] {
    match t_type {
        Type::I_BLOCK => &patterns::I_BLOCK_PATTERNS,
        Type::O_BLOCK => &patterns::O_BLOCK_PATTERNS,
        Type::L_BLOCK => &patterns::L_BLOCK_PATTERNS,
        Type::J_BLOCK => &patterns::J_BLOCK_PATTERNS,
        Type::T_BLOCK => &patterns::T_BLOCK_PATTERNS,
        Type::S_BLOCK => &patterns::S_BLOCK_PATTERNS,
        Type::Z_BLOCK => &patterns::Z_BLOCK_PATTERNS,
    }
}

impl Tetromino {
    pub fn random() -> Tetromino {
        let t_type = Type::random_type();

        Tetromino {
            i: 2,
            j: 2,
            patterns: type_to_patterns(t_type),
            selected_pattern: 0,
        }
    }

    pub fn new(t_type: Type, i: isize, j: isize) -> Tetromino {
        let patterns = type_to_patterns(t_type);
        Tetromino {
            i,
            j,
            patterns,
            selected_pattern: 0,
        }
    }

    pub fn null() -> Tetromino {
        Tetromino {
            i: -100,
            j: -100,
            patterns: &[],
            selected_pattern: 0,
        }
    }

    fn rotate(&mut self, direction: isize) {
        let mut next_pattern = self.selected_pattern as isize + direction;

        if next_pattern >= self.patterns.len() as isize {
            next_pattern = 0;
        }

        if next_pattern < 0 {
            next_pattern = self.patterns.len() as isize - 1;
        }

        self.selected_pattern = next_pattern as usize;
    }

    pub fn move_down(&mut self) {
        self.i += 1;
    }

    pub fn move_left(&mut self) {
        if self.j == 0 {
            return;
        }

        self.j -= 1;
    }

    pub fn move_right(&mut self) {
        self.j += 1;
    }

    pub fn rotate_left(&mut self) {
        self.rotate(LEFT);
    }

    pub fn rotate_right(&mut self) {
        self.rotate(RIGHT);
    }

    pub fn actual_rotation(&self) -> &'static Pattern {
        &self.patterns[self.selected_pattern]
    }
}
