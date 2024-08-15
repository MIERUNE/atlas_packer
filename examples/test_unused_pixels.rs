use std::path::Path;

use atlas_packer::{
    export::PngAtlasExporter,
    pack::TexturePacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::TextureCache,
};

mod utils;
use utils::unused_pixels;

fn main() {
    let texture_cache = TextureCache::new(100_000_000);

    let config = TexturePlacerConfig::new(500, 500, 1);
    let placer = GuillotineTexturePlacer::new(config.clone());
    let exporter = PngAtlasExporter::default();
    let packer = TexturePacker::new(placer, exporter);

    let output_dir = Path::new("examples/output/");
    packer.export(output_dir, &texture_cache, config.width(), config.height());

    let (total_pixels, unused_pixels) = unused_pixels::unused_pixels();

    println!(
        "Total Pixels: {}, Unused Pixels: {}",
        total_pixels, unused_pixels
    );
}
