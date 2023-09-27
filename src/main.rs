extern crate sdl2;

mod tetris;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use std::time;
use tetris::Piece;

const PIXEL_SIZE: u32 = 16;

struct PlayingField {
    base_x: i32,
    base_y: i32,
    field: [[Option<Color>; Self::WIDTH_BLOCKS as usize]; Self::HEIGHT_BLOCKS as usize],
}

enum Direction {
    Left,
    Right,
    Bottom
}

impl PlayingField {
    const WIDTH_BLOCKS: u32 = 10;
    const HEIGHT_BLOCKS: u32 = 20;

    fn new(x: i32, y: i32) -> Self {
        PlayingField{ base_x: x, base_y: y, field: [[None; Self::WIDTH_BLOCKS as usize]; Self::HEIGHT_BLOCKS as usize] }
    }

    fn draw_bounds(&self, canvas: &mut Canvas<Window>) {
        let bound_x = (Self::WIDTH_BLOCKS as i32);
        let bound_y = (Self::HEIGHT_BLOCKS as i32);
        for j in 0i32..bound_y {
            self.draw_block(canvas, -1, j, Color::RGB(127,127,127));
            self.draw_block(canvas, bound_x, j, Color::RGB(127,127,127));
        }
        for i in -1i32..bound_x+1 {
            self.draw_block(canvas, i, bound_y, Color::RGB(127,127,127));
        }
    }

    fn draw_block(&self, canvas: &mut Canvas<Window>, i: i32, j: i32, color: Color) {
        let from_x = self.base_x + (i as i32) * (PIXEL_SIZE as i32);
        let from_y = self.base_y + (j as i32) * (PIXEL_SIZE as i32);
        draw_block_absolute(canvas, from_x, from_y, color);
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        self.draw_bounds(canvas);
        for j in 0..Self::HEIGHT_BLOCKS {
            for i in 0..Self::WIDTH_BLOCKS {
                if let Some(color) = self.field[j as usize][i as usize] {
                    self.draw_block(canvas, i as i32, j as i32, color);
                }
            }
        }
    }

    fn test_collision(&self, piece: &dyn Piece, dir: Direction) -> bool {
        let bounds = piece.get_shape().get_bounds();
        println!("Bounds: {:?}", bounds);
        println!("piece.position =  {:?}", piece.get_position());
        let check_line = piece.get_position().1 + (bounds.3 as i32);
        if check_line < 0 {
            return false;
        } else
            if check_line >= Self::HEIGHT_BLOCKS as i32 {
                return true;
            }
        for i in piece.get_position().0..piece.get_position().0+piece.get_shape().get_bounds().2 as i32 {
            if i-piece.get_position().0 < 0 {
                continue;
            } else
                if piece.get_shape().shape[bounds.3 as usize][(i-piece.get_position().0) as usize] == '*' {
                    if let Some(_) = self.field[check_line as usize][i as usize] {
                        println!("Collide!");
                        return true;
                    }
                }
        }
        false
    }

    fn place_piece(&mut self, piece: Box<dyn Piece>) {
        println!("{:?}", piece.get_position());
        for j in 0..4 {
            for i in 0..4 {
                let y = (piece.get_position().1 + j) as usize;
                if y >= Self::HEIGHT_BLOCKS as usize {
                    break;
                }
                let x = (piece.get_position().0 + i) as usize;
                if x >= Self::WIDTH_BLOCKS as usize {
                    continue;
                }
                if piece.get_shape().shape[j as usize][i as usize] == '*' {
                    self.field[y][x] = Some(piece.get_color());
                }
            }
        }
    }

}

fn draw_block_absolute(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Color) {
    let mut rect = Rect::new(x, y, PIXEL_SIZE, PIXEL_SIZE);
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(rect);
    rect = Rect::new(x + 1i32, y + 1i32, PIXEL_SIZE-2, PIXEL_SIZE-2);
    canvas.set_draw_color(color);
    canvas.fill_rect(rect);
}

fn draw_piece_ex(pf: &PlayingField, canvas: &mut Canvas<Window>, x: i32, y: i32, piece: &dyn Piece) {
    let shape = piece.get_shape();
    let color = piece.get_color();
    for j in 0..4 {
        for i in 0..4 {
            if piece.get_shape().shape[j][i] == '*' {
                pf.draw_block(canvas, x+i as i32, y+j as i32, color);
            }
        }
    } 
}

fn draw_piece(pf: &PlayingField, canvas: &mut Canvas<Window>, piece: &dyn Piece) {
    draw_piece_ex(pf, canvas, piece.get_position().0, piece.get_position().1, piece);
}

async fn game() {
    //let cur_piece = tetris::make_random_piece();
    //let mut i = 0;
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    //let mut sharedCanvas = Arc::new(Mutex::new(canvas));

    let mut pf = PlayingField::new(20, 20);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    /*
    let handle = task::spawn(async {
        let mut cur_piece: Arc<dyn Piece> = tetris::make_random_piece();
        loop {
            let never = future::pending::<()>();
            let duration = Duration::from_millis(800);
            future::timeout(duration, never).await;

        }
    });
    */
    let mut now = time::Instant::now();
    let mut elapsed = time::Duration::new(0, 0);
    let mut cur_piece = tetris::make_random_piece();
    let mut piece_pos = 0;

    'running: loop {
        elapsed += now.elapsed();
        now = time::Instant::now();
        /*
        i = (i + 1) % 255;
           {
           let mut c = sharedCanvas.lock().unwrap();
           c.set_draw_color(Color::RGB(i, 64, 255 - i));
           c.clear();
           }
           */
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    cur_piece.get_shape_mut().rotate();
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        pf.draw(&mut canvas);

        if elapsed > Duration::from_millis(100) {
            elapsed = time::Duration::new(0, 0);
            cur_piece.get_position_mut().1 += 1;
            let collide = pf.test_collision(cur_piece.as_ref(), Direction::Bottom);
            if collide || (cur_piece.get_position().1 + (cur_piece.get_shape().get_bounds().3 as i32) > 19) {
                cur_piece.get_position_mut().1 -= 1;
                pf.place_piece(cur_piece); 
                cur_piece = tetris::make_random_piece();
            }
        }
        draw_piece(&pf, &mut canvas, cur_piece.as_ref());

        //let cur_piece = tetris::make_random_piece();
        //draw_piece(&pf, &mut canvas, 0, 0, cur_piece.as_ref().);

        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
