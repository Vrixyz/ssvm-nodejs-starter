// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

use rand::prelude::*;
use raytracer_core::materials::lambertian_diffuse::Lambertian;
use raytracer_core::materials::metal::Metal;
use raytracer_core::shapes::sphere::Sphere;
use raytracer_core::Vector3;
use raytracer_core::{PixelColor, PixelPosition, Raytracer, Scene};
// use rand_core::{RngCore, OsRng};

// Define the size of our camera
const WIDTH: usize = 1920 / 10;
const HEIGHT: usize = 1080 / 10;

const SAMPLES_PER_PIXEL: i64 = 64;

/*
 * 1. What is going on here?
 * Create a static mutable byte buffer.
 * We will use for putting the output of our graphics,
 * to pass the output to js.
 * NOTE: global `static mut` means we will have "unsafe" code
 * but for passing memory between js and wasm should be fine.
 *
 * 2. Why is the size HEIGHT * WIDTH * 4?
 * We want to have HEIGHT pixels by WIDTH pixels. And 4 colors per pixel (r,g,b,a)
 * Which the Canvas API Supports.
 */
const OUTPUT_BUFFER_SIZE: usize = HEIGHT * WIDTH * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = OUTPUT_BUFFER.as_ptr();
    }

    pointer
}

fn set_pixel(position: PixelPosition, c: PixelColor) {
    // Let's calculate our index, using our 2d -> 1d mapping.
    // And then multiple by 4, for each pixel property (r,g,b,a).
    let square_number: usize = (WIDTH * HEIGHT) - (position.y * WIDTH + position.x) - 1;
    let square_rgba_index: usize = square_number * 4;

    unsafe {
        OUTPUT_BUFFER[square_rgba_index] = c.r; // Red
        OUTPUT_BUFFER[square_rgba_index + 1] = c.g; // Green
        OUTPUT_BUFFER[square_rgba_index + 2] = c.b; // Blue
        OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha (Always Opaque)
    }
}

#[wasm_bindgen]
pub fn get_height() -> usize {
    HEIGHT
}

#[wasm_bindgen]
pub fn get_width() -> usize {
    WIDTH
}

pub struct RendererCommunicator {}

impl raytracer_core::PixelRenderer for RendererCommunicator {
    fn set_pixel(&mut self, pos: PixelPosition, color: PixelColor) {
        set_pixel(pos, color)
    }
    fn invalidate_pixels(&mut self) {
        unimplemented!()
    }
}
#[wasm_bindgen]
pub fn say(s: &str) -> String {
  println!("The Rust function say() received {}", s);
  let r = String::from("hello ");
  return r + s;
}
// Function to generate our checkerboard, pixel by pixel
#[wasm_bindgen]
pub fn render() -> Vec<u8> {
    println!("The Rust function render() begins.");
    let sphere = Sphere::new(
        Vector3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Vector3::new(0.1, 1.0, 0.1))),

    );
    let sphere2 = Sphere::new(
        Vector3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Vector3::new(1.0, 0.1, 0.1))),
    );
    let sphere3 = Sphere::new(
        Vector3::new(0.5, -0.4, -0.85),
        0.1,
        Box::new(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.0)),
    );
    let scene: Scene = vec![&sphere, &sphere2, &sphere3];
    let rng = rand::rngs::StdRng::seed_from_u64(0);
    let mut raytracer = Raytracer::new(WIDTH as f64, HEIGHT as f64, rng, RendererCommunicator {});
    println!("Render() generation is Starting.");
    raytracer.generate(scene.as_slice(), SAMPLES_PER_PIXEL);

    println!("Render() generation is done.");
    /*
    let mut image_ppm = format!("P3\n{} {} \n255\n", WIDTH, HEIGHT);
    
    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {

          let square_number: usize = (WIDTH * HEIGHT) - (y * WIDTH + x) - 1;
          let square_rgba_index: usize = square_number * 4;

          unsafe {
            let red = OUTPUT_BUFFER[square_rgba_index];
            let green = OUTPUT_BUFFER[square_rgba_index + 1];
            let blue = OUTPUT_BUFFER[square_rgba_index + 2];
            image_ppm = image_ppm + &format!("{} {} {}", red, green, blue);
          }
        }
    }
    println!("Setting pixels is done.");
    */
    let result;
    unsafe {
      result = OUTPUT_BUFFER.to_vec()
    }
    result
}
