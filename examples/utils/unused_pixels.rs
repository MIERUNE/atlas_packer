use clap::{Arg, Command};
use image::{open, GenericImageView, Rgba};

pub fn unused_pixels() -> (usize, usize) {
    let matches = Command::new("Image Processor")
        .about("Processes images to find unused pixels")
        .arg(
            Arg::new("INPUT")
                .help("Path to the image file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches
        .get_one::<String>("INPUT")
        .expect("INPUT is required");
    let img = open(input).expect("Failed to open image");

    let total_pixels = img.width() as usize * img.height() as usize;

    let unused_pixels = img
        .pixels()
        .filter(|&(_, _, pixel)| matches!(pixel, Rgba([0, 0, 0, 0])))
        .count();

    (total_pixels, unused_pixels)
}
