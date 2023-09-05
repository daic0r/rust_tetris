// Purpose: Tetris game logic
use sdl2::{pixels::Color, render::Canvas, video::Window};
use rand::distributions::{Distribution, Uniform};

pub fn make_random_piece() -> Box<dyn Piece> {
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(0..=6);
    match uniform.sample(&mut rng) {
        0 => Box::new(Long::new()),
        1 => Box::new(Square::new()),
        2 => Box::new(Tee::new()),
        3 => Box::new(Zee::new()),
        4 => Box::new(InverseZee::new()),
        5 => Box::new(Jay::new()),
        6 => Box::new(El::new()),
        _ => panic!("Invalid piece index!")
    }
}

#[derive(Clone, Default)]
pub struct Bounds(u32, u32, u32, u32);

#[derive(Default)]
pub struct Shape {
    pub shape: Vec<Vec<char>>,
    alignment: usize // number of rotations
}

impl Shape {
    pub fn rotate(&mut self) {
        self.alignment = (self.alignment + 1) % 4;
        let mut new_shape = vec![vec![' '; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let new_i; 
                let new_j;
                if self.alignment % 2 == 0 {
                    new_i = i;
                    new_j = j;
                } else {
                    new_i = 3-i;
                    new_j = 3-j;
                }
                new_shape[i][j] = self.shape[new_j][new_i];
            }
        }
        self.shape = new_shape;
    }

    pub fn bounds(&self) -> Bounds {
        let mut ret = Bounds(u32::MAX, u32::MAX, u32::MIN, u32::MIN);
        for j in 0..4 {
            for i in 0..4 {
                if self.shape[j][i] == '*' {
                    if i < ret.0 as usize {
                        ret.0 = i as u32;
                    }
                    if j < ret.1 as usize {
                        ret.1 = j as u32;
                    }
                    if i > ret.2 as usize {
                        ret.2 = i as u32;
                    }
                    if j > ret.3 as usize {
                        ret.3 = j as u32;
                    }
                }
            }
        }
        ret
    }

}

pub trait Piece {
    fn get_color(&self) -> Color;
    fn get_shape(&self) -> &Shape;
    fn get_shape_mut(&mut self) -> &mut Shape;
}

macro_rules! impl_piece {
    ($name:ident, $color:expr, $shape:expr) => {
        pub struct $name {
            shape: Shape
        }
        impl Piece for $name {
            fn get_color(&self) -> Color {
                $color
            }

            fn get_shape(&self) -> &Shape {
                &self.shape
            }
            fn get_shape_mut(&mut self) -> &mut Shape {
                &mut self.shape
            }
        }

        impl $name {
            fn new() -> $name {
                $name {
                    shape: Shape {
                        shape: $shape,
                        alignment: 0
                    }
                }
            } 
        }
    }
}

impl_piece!(Long, Color::RGB(0, 255, 255), vec![
     vec![' ', ' ', ' ', ' '],
     vec!['*', '*', '*', '*'],
     vec![' ', ' ', ' ', ' '],
     vec![' ', ' ', ' ', ' ']
]);
impl_piece!(Square, Color::RGB(255, 255, 0), vec![
     vec![' ', ' ', ' ', ' '],
     vec![' ', '*', '*', ' '],
     vec![' ', '*', '*', ' '],
     vec![' ', ' ', ' ', ' '],
]);
impl_piece!(Tee, Color::RGB(255, 0, 255), vec![
     vec![' ', ' ', ' ', ' '],
     vec![' ', '*', '*', '*'],
     vec![' ', ' ', '*', ' '],
     vec![' ', ' ', ' ', ' '],
]);
impl_piece!(Zee, Color::RGB(255, 0, 0), vec![
     vec![' ', ' ', ' ', ' '],
     vec![' ', '*', '*', ' '],
     vec![' ', ' ', '*', '*'],
     vec![' ', ' ', ' ', ' '],
]);
impl_piece!(InverseZee, Color::RGB(0, 255, 0), vec![
     vec![' ', ' ', ' ', ' '],
     vec![' ', '*', '*', ' '],
     vec!['*', '*', ' ', ' '],
     vec![' ', ' ', ' ', ' '],
]);
impl_piece!(Jay, Color::RGB(0, 0, 255), vec![
     vec![' ', ' ', ' ', ' '],
     vec![' ', ' ', '*', ' '],
     vec![' ', ' ', '*', ' '],
     vec![' ', '*', '*', ' '],
]);
impl_piece!(El, Color::RGB(255, 165, 0), vec![
     vec![' ', ' ', ' ', ' '],
     vec![' ', '*', ' ', ' '],
     vec![' ', '*', ' ', ' '],
     vec![' ', '*', '*', ' '],
]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_bounds() {
        let mut long = Long::new();
        let bounds = long.get_shape().bounds();
        assert_eq!(bounds.0, 0);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 3);
        assert_eq!(bounds.3, 1);
        long.get_shape_mut().rotate();
        let bounds = long.get_shape().bounds();
        assert_eq!(bounds.0, 2);
        assert_eq!(bounds.1, 0);
        assert_eq!(bounds.2, 2);
        assert_eq!(bounds.3, 3);
        long.get_shape_mut().rotate();
        let bounds = long.get_shape().bounds();
        assert_eq!(bounds.0, 0);
        assert_eq!(bounds.1, 2);
        assert_eq!(bounds.2, 3);
        assert_eq!(bounds.3, 2);
        long.get_shape_mut().rotate();
        let bounds = long.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 0);
        assert_eq!(bounds.2, 1);
        assert_eq!(bounds.3, 3);
        long.get_shape_mut().rotate();
        let bounds = long.get_shape().bounds();
        assert_eq!(bounds.0, 0);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 3);
        assert_eq!(bounds.3, 1);
    }

    #[test]
    fn test_square_bounds() {
        let mut square = Square::new();
        let bounds = square.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 2);
        assert_eq!(bounds.3, 2);
        square.get_shape_mut().rotate();
        let bounds = square.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 2);
        assert_eq!(bounds.3, 2);
        square.get_shape_mut().rotate();
        let bounds = square.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 2);
        assert_eq!(bounds.3, 2);
        square.get_shape_mut().rotate();
        let bounds = square.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 2);
        assert_eq!(bounds.3, 2);
        square.get_shape_mut().rotate();
        let bounds = square.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 2);
        assert_eq!(bounds.3, 2);
    }

    #[test]
    fn test_tee_bounds() {
        let mut tee = Tee::new();
        let bounds = tee.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 3);
        assert_eq!(bounds.3, 2);
        tee.get_shape_mut().rotate();
        let bounds = tee.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 0);
        assert_eq!(bounds.2, 2);
        assert_eq!(bounds.3, 2);
        /*
        tee.get_shape_mut().rotate();
        let bounds = tee.get_shape().bounds();
        assert_eq!(bounds.0, 0);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 1);
        assert_eq!(bounds.3, 3);
        tee.get_shape_mut().rotate();
        let bounds = tee.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 1);
        assert_eq!(bounds.3, 2);
        tee.get_shape_mut().rotate();
        let bounds = tee.get_shape().bounds();
        assert_eq!(bounds.0, 1);
        assert_eq!(bounds.1, 1);
        assert_eq!(bounds.2, 2);
        assert_eq!(bounds.3, 2);
        */
    }

}

/*
pub struct Long {
    shape: Shape
}
impl Piece for Long {
    fn get_color(&self) -> Color {
        Color::RGB(0, 255, 255)
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }
    fn get_shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }
}

impl Long {
    fn new() -> Long {
        Long {
            shape: Shape {
                shape: vec![
                     vec![' ', ' ', ' ', ' '],
                     vec!['*', '*', '*', '*'],
                     vec![' ', ' ', ' ', ' '],
                     vec![' ', ' ', ' ', ' ']
                ]
            }
        }
    } 
}
*/

/*

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


#[derive(Clone, Default)]
pub struct Square {
    shape: Shape;
}

#[derive(Clone, Default)]
pub struct Tee {
    shape: Shape;
}

#[derive(Clone, Default)]
pub struct Zee {
    shape: Shape;
}

#[derive(Clone, Default)]
pub struct InverseZee {
}

#[derive(Clone, Default)]
pub struct Jay {
}

#[derive(Clone, Default)]
pub struct El {
}
*/
