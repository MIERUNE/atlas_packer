use std::{collections::HashMap, path::PathBuf, sync::Mutex, time::Instant};

use image::{open, GenericImageView, Rgba};

use atlas_packer::{
    export::PngAtlasExporter,
    pack::TexturePacker,
    place::{GuillotineTexturePlacer, TexturePlacer, TexturePlacerConfig},
    texture::{CroppedTexture, DownsampleFactor, TextureSizeCache},
};

use clap::{Arg, ArgAction, Command};
mod utils;

#[derive(Debug, Clone)]
struct Polygon {
    id: String,
    uv_coords: Vec<(f64, f64)>,
    texture_uri: PathBuf,
    downsample_factor: DownsampleFactor,
}

mod algorithms {
    pub struct GuillotineTexturePlacer;

    impl GuillotineTexturePlacer {
        pub fn new() -> Self {
            GuillotineTexturePlacer
        }

        pub fn execute(&self) {
            println!("Using GuillotineTexturePlacer algorithm.");
        }
    }

    // Add more algorithms here
}

fn main() {
    let faces = [
        (
            "yellow_dice",
            vec![
                (0.316406, 0.816406),
                (-0.000000, 0.628906),
                (0.316406, 0.628906),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.500000, 1.000000),
                (0.816406, 0.816406),
                (0.816406, 1.000000),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.500000, 0.816406),
                (0.816406, 0.628906),
                (0.816406, 0.816406),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.500000, 0.628906),
                (0.816406, 0.445312),
                (0.816406, 0.628906),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.816406, 0.816406),
                (1.000000, 0.628906),
                (0.816406, 0.628906),
            ],
        ),
        (
            "yellow_dice",
            vec![
                (0.500000, 0.816406),
                (0.316406, 0.628906),
                (0.500000, 0.628906),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.500000, 0.329309),
                (0.327473, 0.656008),
                (0.327473, 0.329309),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.500000, 0.656008),
                (0.327473, 0.984543),
                (0.327473, 0.656008),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.826699, 0.656008),
                (0.999226, 0.329309),
                (0.999226, 0.656008),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.500000, 0.000774),
                (0.327473, 0.329309),
                (0.327473, 0.000774),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.327473, 0.656008),
                (0.327473, 0.329309),
                (0.000774, 0.656008),
            ],
        ),
        (
            "blue_dice",
            vec![
                (0.500000, 0.656008),
                (0.826699, 0.329309),
                (0.826699, 0.656008),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.371094, 0.871094),
                (-0.000000, 0.742188),
                (0.371094, 0.742188),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.500000, 1.000000),
                (0.871094, 0.871094),
                (0.871094, 1.000000),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.500000, 0.871094),
                (0.871094, 0.742188),
                (0.871094, 0.871094),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.500000, 0.742188),
                (0.871094, 0.613281),
                (0.871094, 0.742188),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.871094, 0.871094),
                (1.000000, 0.742188),
                (0.871094, 0.742188),
            ],
        ),
        (
            "red_dice",
            vec![
                (0.500000, 0.871094),
                (0.371094, 0.742188),
                (0.500000, 0.742188),
            ],
        ),
    ];

    let material_to_texture = HashMap::from([
        ("blue_dice", "blue_dice.png"),
        ("red_dice", "red_dice.png"),
        ("yellow_dice", "yellow_dice.png"),
    ]);

    // 3D Tiles Sink passes the texture path and UV coordinates for each polygon
    let mut polygons: Vec<Polygon> = Vec::new();
    let downsample_factor = 1.0;

    for (idx, (material, uv_coords)) in faces.iter().enumerate() {
        let texture_file = material_to_texture.get(material).unwrap();
        let path_string = format!("./examples/assets/dice/{}", texture_file);
        let image_path = PathBuf::from(path_string);
        polygons.push(Polygon {
            id: format!("texture_{}_{}", material, idx),
            uv_coords: uv_coords.iter().map(|&(u, v)| (u, v)).collect(),
            texture_uri: image_path,
            downsample_factor: DownsampleFactor::new(&downsample_factor),
        });
    }

    let matches = Command::new("Image Processor")
        .about("Processes images to find unused pixels")
        .arg(
            Arg::new("ALGORITHM")
                .help("Specify the packing algorithm (guillotine or custom)")
                .short('a')
                .long("algorithm")
                .default_value("guillotine")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("INPUT")
                .help("Outputs unused pixels")
                .short('u')
                .long("unused_pixels")
                .value_name("FILE")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("TIME")
                .help("Measure execution time")
                .short('t')
                .long("time")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let algorithm = matches.get_one::<String>("ALGORITHM").unwrap();

    match algorithm.as_str() {
        "guillotine" => {
            let placer = algorithms::GuillotineTexturePlacer::new();
            placer.execute();
        }
        _ => {
            eprintln!("Unknown algorithm: {}", algorithm);
            std::process::exit(1);
        }
    }

    // initialize texture packer
    let config = TexturePlacerConfig::new(500, 500, 1);
    let placer: Box<dyn TexturePlacer> = Box::new(GuillotineTexturePlacer::new(config.clone()));
    let exporter = PngAtlasExporter::default();
    let packer = Mutex::new(TexturePacker::new(placer, exporter));

    // Texture cache
    let texture_cache = TextureSizeCache::new();

    // Measure execution time start for adding textures to the atlas
    let measure_time = matches.get_flag("TIME");

    // Add textures to the atlas,
    polygons.iter().for_each(|polygon| {
        let place_start = Instant::now();
        let texture_size = texture_cache.get_or_insert(&polygon.texture_uri);
        let cropped_texture = CroppedTexture::new(
            &polygon.texture_uri,
            texture_size,
            &polygon.uv_coords,
            polygon.downsample_factor.clone(),
        );

        let _ = packer
            .lock()
            .unwrap()
            .add_texture(polygon.id.clone(), cropped_texture);
        let place_duration = place_start.elapsed();

        if measure_time {
            println!("{}, texture place process {:?}", polygon.id, place_duration);
        }
    });

    let output_unused_pixels = matches.contains_id("INPUT");

    let input = match matches.get_one::<String>("INPUT") {
        Some(input_value) => input_value,
        None => {
            std::process::exit(0);
        }
    };

    let img = open(input).expect("Failed to open image");

    if output_unused_pixels {
        let unused_pixels = img
            .pixels()
            .filter(|&(_, _, pixel)| matches!(pixel, Rgba([0, 0, 0, 0])))
            .count();

        println!("unused pixels: {}", unused_pixels);
    }
}
