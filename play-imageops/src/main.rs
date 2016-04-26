

extern crate image;
extern crate rand;
extern crate nalgebra;

use image::{FilterType, GenericImage, Pixel};
use nalgebra::DMat;
use rand::distributions::{Normal, IndependentSample};

use std::fs::File;


// Examples from https://github.com/zsiciarz/24daysofrust
fn twentyfour_days_of_rust() {

    let img = image::open("assets/lenna.png").ok().expect("Opening image failed");

    // flip
    // https://github.com/zsiciarz/24daysofrust/blob/master/src/day12.rs
    let filtered = img.fliph();
    let mut out = File::create("out-flip.png").unwrap();
    let _ = filtered.save(&mut out, image::PNG).ok().expect("Saving image failed");


    // edge detection
    // https://github.com/zsiciarz/24daysofrust/blob/master/src/day12.rs
    // http://www.roborealm.com/help/Convolution.php
    let kernel = [-1.0f32, -1.0, -1.0,
                  -1.0,     8.0, -1.0,
                  -1.0,    -1.0, -1.0];
    let filtered = img.filter3x3(&kernel);
    let mut out = File::create("out-edgy.png").unwrap();
    let _ = filtered.save(&mut out, image::PNG).ok().expect("Saving image failed");

    // making noise
    // https://github.com/zsiciarz/24daysofrust/blob/master/src/day12.rs
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();
    let normal = Normal::new(15.0, 15.0);
    let mut noisy = img.brighten(-25);
    for x in 0..(width) {
        for y in 0..(height) {
            let offset = normal.ind_sample(&mut rng) as u8;
            let px = img.get_pixel(x, y).map(|v| if v <= 255 - offset { v + offset } else { 255 });
            noisy.put_pixel(x, y, px);
        }
    }
    let mut out = File::create("out-noisy.png").unwrap();
    let _ = noisy.save(&mut out, image::PNG).ok().expect("Saving image failed");

    // resize with Lanczos3 filter
    // https://github.com/zsiciarz/24daysofrust/blob/master/src/day12.rs
    // http://docs.gimp.org/en/gimp-tools-transform.html
    let thumbnail = img.resize(120, 120, FilterType::Lanczos3);
    let mut out = File::create("out-thumb.png").unwrap();
    let _ = thumbnail.save(&mut out, image::PNG).ok().expect("Saving image failed");


    // pattern
    // https://github.com/zsiciarz/24daysofrust/blob/master/src/day17.rs
    let v = (0..10).map(|x| x * 3).collect::<Vec<_>>();
    println!("{:?}", v);
    let v = (0..10).map(|_| rand::random::<u32>()).collect::<Vec<_>>();
    println!("{:?}", v);
    let mat: DMat<u32> = DMat::from_fn(7, 7, |i, j| if j <= i { 1 } else { 0 });
    println!("{:?}", mat);
    let buffer = image::ImageBuffer::from_fn(512u32, 512u32, |x: u32, y: u32| {
        Pixel::from_channels((x * y % 256) as u8, (y % 256) as u8, (x % 256) as u8, 255)
    });
    let img = image::DynamicImage::ImageRgba8(buffer);
    let mut out = File::create("out-pattern.png").unwrap();
    let _ = img.save(&mut out, image::PNG);
}


fn main() {
	twentyfour_days_of_rust();
}