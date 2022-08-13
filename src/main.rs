extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use convolve2d::*;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const SIZE: usize = (WIDTH * HEIGHT) as usize;

const STATES: u32 = 12;

trait Sum<T> {
    fn sum(&self) -> T;
}
impl<T: std::convert::From<i32> + for<'a> std::ops::AddAssign<&'a T> + 'static, const N: usize> Sum<T> for convolve2d::StaticMatrix<T, N> {
    fn sum(&self) -> T{
        let mut sum:T = 0.into();
        for i in self.get_data().iter(){
            sum += i;
        }
        sum
    }
}

fn growth(neighbours: f64) -> f64 {
    0. + {
        if (0.20..0.25).contains(&neighbours) {1.}
        else {0.}
    } - {
        if !(0.18..0.33).contains(&neighbours) {1.}
        else {0.}
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Lenia", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(sdl2::pixels::PixelFormatEnum::ARGB8888, WIDTH, HEIGHT).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut pxl_vec = vec![0.; SIZE];

    let mut rng = rand::thread_rng();
    for i in pxl_vec.iter_mut(){
        *i = rng.gen_range(0..STATES) as f64;
    }
    let mut dyn_mat: DynamicMatrix<f64> = DynamicMatrix::new(WIDTH as usize, HEIGHT as usize, pxl_vec).unwrap();
    let kern_stp: StaticMatrix<f64, 9> = StaticMatrix::new(3, 3, [1., 1., 1.,
                                                                1., 0., 1.,
                                                                1., 1., 1.]).unwrap();
    let sum: f64 = kern_stp.sum();
    let kernel = kern_stp.map(|x| x / (STATES as f64 * sum));
    let colorgrad = colorgrad::viridis();
    'running: loop {
        let result = convolve2d(&dyn_mat, &kernel);
        let result_data = result.get_data();
        let dyn_data = dyn_mat.get_data_mut();
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                dyn_data[(i * WIDTH + j) as usize] = (dyn_data[(i * WIDTH + j) as usize] as f64 + growth(result_data[(i * WIDTH + j) as usize])).clamp(0., STATES as f64) as f64;
            }
        }
        texture.with_lock(
            None,
            |bytearray, _|{
                for i in 0..HEIGHT {
                    for j in 0..WIDTH {
                        let offset: usize = (i * WIDTH * 4 + j * 4) as usize;
                        // WINDOWS: BGRA (endianess)
                        let color = colorgrad.at(dyn_mat.get_data()[(i * WIDTH + j) as usize] as f64 / STATES as f64).to_rgba8();
                        bytearray[offset    ] = color[2];
                        bytearray[offset + 1] = color[1];
                        bytearray[offset + 2] = color[0];
                        bytearray[offset + 3] = 255;
                    }
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

