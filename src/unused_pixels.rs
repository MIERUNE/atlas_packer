use image::{open, GenericImageView, Rgba};

use clap::{Arg, ArgAction, Command};

pub fn unused_pixels() {
    let matches = Command::new("Image Processor")
        .about("Processes images to find unused pixels")
        .arg(
            Arg::new("INPUT")
                .help("Outputs unused pixels")
                .short('u')
                .long("unused_pixels")
                .value_name("FILE")
                .action(ArgAction::Set),
        )
        .get_matches();

    let input = match matches.get_one::<String>("INPUT") {
        Some(input_value) => input_value,
        None => {
            std::process::exit(0);
        }
    };
    let output_unused_pixels = matches.contains_id("INPUT");

    let img = open(input).expect("Failed to open image");

    if output_unused_pixels {
        let unused_pixels = img
            .pixels()
            .filter(|&(_, _, pixel)| matches!(pixel, Rgba([0, 0, 0, 0])))
            .count();

        println!("unused pixels: {}", unused_pixels);
    }
}
