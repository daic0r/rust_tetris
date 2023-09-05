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
}

impl PlayingField {
    const WIDTH_BLOCKS: u32 = 10;
    const HEIGHT_BLOCKS: u32 = 20;

    fn new(x: i32, y: i32) -> Self {
        PlayingField{ base_x: x, base_y: y }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
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
}

fn draw_block_absolute(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Color) {
    let mut rect = Rect::new(x, y, PIXEL_SIZE, PIXEL_SIZE);
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_rect(rect);
    rect = Rect::new(x + 1i32, y + 1i32, PIXEL_SIZE-2, PIXEL_SIZE-2);
    canvas.set_draw_color(color);
    canvas.fill_rect(rect);
}

fn draw_piece(pf: &PlayingField, canvas: &mut Canvas<Window>, x: i32, y: i32, piece: &dyn Piece) {
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

    let pf = PlayingField::new(20, 20);

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

        if elapsed > Duration::from_millis(800) {
            elapsed = time::Duration::new(0, 0);
            piece_pos += 1;
        }
        if piece_pos > 20 {
            piece_pos = 0;
            cur_piece = tetris::make_random_piece();
        }
        draw_piece(&pf, &mut canvas, 0, piece_pos, cur_piece.as_ref());

        let cur_piece = tetris::make_random_piece();
        //draw_piece(&pf, &mut canvas, 0, 0, cur_piece.as_ref().);

        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
