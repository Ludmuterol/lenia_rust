extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use convolve2d::*;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const SIZE: usize = (WIDTH * HEIGHT) as usize;

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

    let mut pxl_vec = vec![0; SIZE];
        
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            pxl_vec[(i * WIDTH + j) as usize] = {
                let num = rand::random::<f64>();
                if num >= 0.5 { 1 }
                else{ 0 }
            }
        }
    }
    let mut dyn_mat: DynamicMatrix<u32> = DynamicMatrix::new(WIDTH as usize, HEIGHT as usize, pxl_vec).unwrap();
    let kernel: StaticMatrix<u32, 9> = StaticMatrix::new(3, 3, [1, 1, 1,
                                                                1, 0, 1,
                                                                1, 1, 1]).unwrap();
    'running: loop {
        let result = convolve2d(&dyn_mat, &kernel);
        let result_data = result.get_data();
        let dyn_data = dyn_mat.get_data_mut();
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                dyn_data[(i * WIDTH + j) as usize] = {
                    if result_data[(i * WIDTH + j) as usize] == 3 || (result_data[(i * WIDTH + j) as usize] == 2 && dyn_data[(i * WIDTH + j) as usize] == 1) {1}
                    else {0}
                }
            }
        }
        texture.with_lock(
            None,
            |bytearray, _|{
                for (i, x) in &mut bytearray.iter_mut().enumerate() {
                    *x = (dyn_mat.get_data()[(i / 4) as usize] * 255) as u8;
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

