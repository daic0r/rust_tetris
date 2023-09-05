// Purpose: Tetris game logic
use sdl2::pixels::Color;

pub trait Piece {
    fn get_color(&self) -> Color;
    fn get_shape(&self) -> Vec<Vec<char>>;
    fn rotate(&self) -> Vec<Vec<char>> {
        let mut new_shape = vec![vec![' '; 4]; 4];
        let shape = self.get_shape();
        for i in 0..4 {
            for j in 0..4 {
                new_shape[i][j] = shape[j][i];
            }
        }
        new_shape
    }
}

impl Piece for Long {
    fn get_color(&self) -> Color {
        Color::RGB(0, 255, 255)
    }

    fn get_shape(&self) -> Vec<Vec<char>> {
        vec![
             vec![' ', ' ', ' ', ' '],
             vec!['*', '*', '*', '*'],
             vec![' ', ' ', ' ', ' '],
             vec![' ', ' ', ' ', ' ']
        ]
    }
}

impl Piece for Square {
    fn get_color(&self) -> Color {
        Color::RGB(255, 255, 0)
    }

    fn get_shape(&self) -> Vec<Vec<char>> {
        vec![
             vec![' ', ' ', ' ', ' '],
             vec![' ', '*', '*', ' '],
             vec![' ', '*', '*', ' '],
             vec![' ', ' ', ' ', ' '],
        ]
    }
}

impl Piece for Tee {
    fn get_color(&self) -> Color {
        Color::RGB(255, 0, 255)
    }

    fn get_shape(&self) -> Vec<Vec<char>> {
        vec![
             vec![' ', ' ', ' ', ' '],
             vec![' ', '*', '*', '*'],
             vec![' ', ' ', '*', ' '],
             vec![' ', ' ', ' ', ' '],
        ]
    }
}

impl Piece for Zee {
    fn get_color(&self) -> Color {
        Color::RGB(255, 0, 0)
    }

    fn get_shape(&self) -> Vec<Vec<char>> {
        vec![
             vec![' ', ' ', ' ', ' '],
             vec![' ', '*', '*', ' '],
             vec![' ', ' ', '*', '*'],
             vec![' ', ' ', ' ', ' '],
        ]
    }
}

impl Piece for InverseZee {
    fn get_color(&self) -> Color {
        Color::RGB(0, 255, 0)
    }

    fn get_shape(&self) -> Vec<Vec<char>> {
        vec![
             vec![' ', ' ', ' ', ' '],
             vec![' ', '*', '*', ' '],
             vec!['*', '*', ' ', ' '],
             vec![' ', ' ', ' ', ' '],
        ]
    }
}

impl Piece for Jay {
    fn get_color(&self) -> Color {
        Color::RGB(0, 0, 255)
    }

    fn get_shape(&self) -> Vec<Vec<char>> {
        vec![
             vec![' ', ' ', ' ', ' '],
             vec![' ', ' ', '*', ' '],
             vec![' ', ' ', '*', ' '],
             vec![' ', '*', '*', ' '],
        ]
    }
}

impl Piece for El {
    fn get_color(&self) -> Color {
        Color::RGB(255, 165, 0)
    }

    fn get_shape(&self) -> Vec<Vec<char>> {
        vec![
             vec![' ', ' ', ' ', ' '],
             vec![' ', '*', ' ', ' '],
             vec![' ', '*', ' ', ' '],
             vec![' ', '*', '*', ' '],
        ]
    }
}

#[derive(Clone)]
pub struct Long {
}

#[derive(Clone)]
pub struct Square {
}

#[derive(Clone)]
pub struct Tee {
}

#[derive(Clone)]
pub struct Zee {
}

#[derive(Clone)]
pub struct InverseZee {
}

#[derive(Clone)]
pub struct Jay {
}

#[derive(Clone)]
pub struct El {
}
