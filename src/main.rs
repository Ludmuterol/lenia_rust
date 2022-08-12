extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("GOL", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(sdl2::pixels::PixelFormatEnum::ARGB8888, WIDTH, HEIGHT).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut pxl_vec = vec![0; (WIDTH * HEIGHT) as usize].into_boxed_slice();
    let mut tmp_vec = vec![0; (WIDTH * HEIGHT) as usize].into_boxed_slice();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            pxl_vec[(i * WIDTH + j) as usize] = {
                let num = rand::random::<f64>();
                if num >= 0.5 { 1 }
                else{ 0 }
            }
        }
    }
    'running: loop {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let mut neigbour_cells = 0;            
                if i > 0 && j > 0 {neigbour_cells += pxl_vec[((i - 1) * WIDTH + j - 1) as usize];}
                if i > 0 {neigbour_cells += pxl_vec[((i - 1) * WIDTH + j) as usize];}
                if i > 0 && j < WIDTH - 1 {neigbour_cells += pxl_vec[((i - 1) * WIDTH + j + 1) as usize];}
                if j > 0 {neigbour_cells += pxl_vec[(i * WIDTH + j - 1) as usize];}
                if j < WIDTH - 1 {neigbour_cells += pxl_vec[(i * WIDTH + j + 1) as usize];}
                if i < HEIGHT - 1 && j > 0 {neigbour_cells += pxl_vec[((i + 1) * WIDTH + j - 1) as usize];}
                if i < HEIGHT - 1 {neigbour_cells += pxl_vec[((i + 1) * WIDTH + j) as usize];}
                if i < HEIGHT - 1 && j < WIDTH - 1 {neigbour_cells += pxl_vec[((i + 1) * WIDTH + j + 1) as usize];}
                if neigbour_cells == 3 || (neigbour_cells == 2 && pxl_vec[(i * WIDTH + j) as usize] == 1) { 
                    tmp_vec[(i * WIDTH + j) as usize] = 1;
                } else {
                    tmp_vec[(i * WIDTH + j) as usize] = 0;
                }
            }
        }
        pxl_vec.copy_from_slice(&tmp_vec);
        texture.with_lock(
            None,
            |bytearray, _|{
                for i in 0..bytearray.len() {
                    bytearray[i] = pxl_vec[i / 4] * 255;
                }
            }
        ).unwrap();
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => ()
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0,1_000_000_000u32 / 60));
    }
}

