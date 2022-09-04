extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

use fft2d::slice::{fft_2d, fftshift, ifft_2d};
use num_complex::Complex;

const WIDTH: u32 = 750;
const HEIGHT: u32 = 750;
const PIXEL_EDGE_SIZE: u32 = 1;

const UPDATE_FREQ: f64 = 10.;
const KERNEL_RAD: u32 = 13;
const BELL_M: f64 = 0.15;
const BELL_S: f64 = 0.015;

//calculated at compiletime
const A_WIDTH: u32 = WIDTH / PIXEL_EDGE_SIZE;
const A_HEIGHT: u32 = HEIGHT / PIXEL_EDGE_SIZE;
const A_SIZE: usize = (A_WIDTH * A_HEIGHT) as usize;

trait Sum<T> {
	fn sum(&self) -> T;
}
impl<T: std::convert::From<i32> + for<'a> std::ops::AddAssign<&'a T> + 'static> Sum<T> for Vec<T> {
	fn sum(&self) -> T{
		let mut sum:T = 0.into();
		for i in self.iter(){
			sum += i;
		}
		sum
	}
}

fn growth(neighbours: f64) -> f64 {
	bell(neighbours, BELL_M, BELL_S) * 2. - 1.
}

fn bell(x: f64, m: f64, s: f64) -> f64 {
	f64::exp(-((x - m)/s).powi(2) / 2.)
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

	//"name":"Orbium","R":13,"T":10,"m":0.15,"s":0.015,"b":[1] widt = 20 height = 20
	let orbium = [
		0.  ,0.  ,0.  ,0.  ,0.  ,0.  ,0.1 ,0.14,0.1 ,0.  ,0.  ,0.03,0.03,0.  ,0.  ,0.3 ,0.  ,0.  ,0.  ,0.  ,
		0.  ,0.  ,0.  ,0.  ,0.  ,0.08,0.24,0.3 ,0.3 ,0.18,0.14,0.15,0.16,0.15,0.09,0.2 ,0.  ,0.  ,0.  ,0.  ,
		0.  ,0.  ,0.  ,0.  ,0.  ,0.15,0.34,0.44,0.46,0.38,0.18,0.14,0.11,0.13,0.19,0.18,0.45,0.  ,0.  ,0.  ,
		0.  ,0.  ,0.  ,0.  ,0.06,0.13,0.39,0.5 ,0.5 ,0.37,0.06,0.  ,0.  ,0.  ,0.02,0.16,0.68,0.  ,0.  ,0.  ,
		0.  ,0.  ,0.  ,0.11,0.17,0.17,0.33,0.4 ,0.38,0.28,0.14,0.  ,0.  ,0.  ,0.  ,0.  ,0.18,0.42,0.  ,0.  ,
		0.  ,0.  ,0.09,0.18,0.13,0.06,0.08,0.26,0.32,0.32,0.27,0.  ,0.  ,0.  ,0.  ,0.  ,0.  ,0.82,0.  ,0.  ,
		0.27,0.  ,0.16,0.12,0.  ,0.  ,0.  ,0.25,0.38,0.44,0.45,0.34,0.  ,0.  ,0.  ,0.  ,0.  ,0.22,0.17,0.  ,
		0.  ,0.07,0.2 ,0.02,0.  ,0.  ,0.  ,0.31,0.48,0.57,0.6 ,0.57,0.  ,0.  ,0.  ,0.  ,0.  ,0.  ,0.49,0.  ,
		0.  ,0.59,0.19,0.  ,0.  ,0.  ,0.  ,0.2 ,0.57,0.69,0.76,0.76,0.49,0.  ,0.  ,0.  ,0.  ,0.  ,0.36,0.  ,
		0.  ,0.58,0.19,0.  ,0.  ,0.  ,0.  ,0.  ,0.67,0.83,0.9 ,0.92,0.87,0.12,0.  ,0.  ,0.  ,0.  ,0.22,0.07,
		0.  ,0.  ,0.46,0.  ,0.  ,0.  ,0.  ,0.  ,0.7 ,0.93,1.  ,1.  ,1.  ,0.61,0.  ,0.  ,0.  ,0.  ,0.18,0.11,
		0.  ,0.  ,0.82,0.  ,0.  ,0.  ,0.  ,0.  ,0.47,1.  ,1.  ,0.98,1.  ,0.96,0.27,0.  ,0.  ,0.  ,0.19,0.1 ,
		0.  ,0.  ,0.46,0.  ,0.  ,0.  ,0.  ,0.  ,0.25,1.  ,1.  ,0.84,0.92,0.97,0.54,0.14,0.04,0.1 ,0.21,0.05,
		0.  ,0.  ,0.  ,0.4 ,0.  ,0.  ,0.  ,0.  ,0.09,0.8 ,1.  ,0.82,0.8 ,0.85,0.63,0.31,0.18,0.19,0.2 ,0.01,
		0.  ,0.  ,0.  ,0.36,0.1 ,0.  ,0.  ,0.  ,0.05,0.54,0.86,0.79,0.74,0.72,0.6 ,0.39,0.28,0.24,0.13,0.  ,
		0.  ,0.  ,0.  ,0.01,0.3 ,0.07,0.  ,0.  ,0.08,0.36,0.64,0.7 ,0.64,0.6 ,0.51,0.39,0.29,0.19,0.04,0.  ,
		0.  ,0.  ,0.  ,0.  ,0.1 ,0.24,0.14,0.1 ,0.15,0.29,0.45,0.53,0.52,0.46,0.4 ,0.31,0.21,0.08,0.  ,0.  ,
		0.  ,0.  ,0.  ,0.  ,0.  ,0.08,0.21,0.21,0.22,0.29,0.36,0.39,0.37,0.33,0.26,0.18,0.09,0.  ,0.  ,0.  ,
		0.  ,0.  ,0.  ,0.  ,0.  ,0.  ,0.03,0.13,0.19,0.22,0.24,0.24,0.23,0.18,0.13,0.05,0.  ,0.  ,0.  ,0.  ,
		0.  ,0.  ,0.  ,0.  ,0.  ,0.  ,0.  ,0.  ,0.02,0.06,0.08,0.09,0.07,0.05,0.01,0.  ,0.  ,0.  ,0.  ,0.
	];

	let mut pxl_vec = vec![0.; A_SIZE];
	for i in 0..20 {
		for j in 0..20 {
			pxl_vec[(i * A_WIDTH + j) as usize] = orbium[(i * 20 + j) as usize];
		}
	}
	let mut rng = rand::thread_rng();
	for i in pxl_vec.iter_mut(){
		*i = rng.gen();
	}
	let mut kern_stp: Vec<f64> = vec![0.; A_SIZE];
	for (i, row) in kern_stp.chunks_exact_mut(A_WIDTH as usize).enumerate() {
		for (j, pix) in row.iter_mut().enumerate() {
			let tmp_val = (
				(i as f64 - (A_HEIGHT / 2) as f64).powi(2) + 
				(j as f64 - (A_WIDTH / 2 ) as f64).powi(2) 
			).sqrt() / KERNEL_RAD as f64;
			*pix = {
				if tmp_val < 1.0 {bell(tmp_val, 0.5, 0.15)}
				else {0.}
			};
			
		}
	}

	let sum: f64 = kern_stp.sum();
	let mut comp_kern: Vec<Complex<f64>> = kern_stp.iter().map(|&x| Complex::new(x / sum, 0.0) ).collect();
	comp_kern = fftshift(A_WIDTH as usize, A_HEIGHT as usize, &comp_kern);
	fft_2d(A_WIDTH as usize, A_HEIGHT as usize, &mut comp_kern);

	let colorgrad = colorgrad::viridis();
	'running: loop {
		let start = Instant::now();
		let mut image: Vec<Complex<f64>> = pxl_vec.iter().map(|&x| Complex::new(x, 0.0)).collect();
		fft_2d(A_WIDTH as usize, A_HEIGHT as usize, &mut image);
		for (i, k) in image.iter_mut().zip(&comp_kern) {
			*i *= k;
		}
		ifft_2d(A_WIDTH as usize, A_HEIGHT as usize, &mut image);
		for (v, c) in pxl_vec.iter_mut().zip(&image) {
			*v = (*v + ((1. / UPDATE_FREQ) * growth((c * 1.0 / (A_WIDTH * A_HEIGHT) as f64).re))).clamp(0. , 1.);
		}
		texture.with_lock(
			None,
			|bytearray, _|{
				for i in 0..HEIGHT {
					for j in 0..WIDTH {
						let offset: usize = (i * WIDTH * 4 + j * 4) as usize;
						// WINDOWS: BGRA (endianess)
						let color = colorgrad.at(pxl_vec[
							((i / PIXEL_EDGE_SIZE) * A_WIDTH + j / PIXEL_EDGE_SIZE) as usize
						] as f64).to_rgba8();
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
		let time = Duration::new(0,1_000_000_000u32 / 60).saturating_sub(start.elapsed());
		if !time.is_zero() {
			::std::thread::sleep(time);
		}
	}
}

