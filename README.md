# atlas-packer

## Project Overview

atlas-packer is a Rust-based utility for creating texture atlases from multiple source textures. It takes as input a set of image textures (by file path or URL) and corresponding UV coordinate polygons (regions of those images) and packs them into one or more larger atlas images​.

This helps combine many small texture segments into a single texture atlas, which can improve rendering performance by reducing texture swaps. The library is designed with high performance in mind, making it suitable for large-scale conversions (for example, consolidating thousands of texture files for 3D tiles or game assets) while preserving the mapping of each original texture region within the atlas.

This crate is actually used in the [PLATEAU GIS Converter](https://github.com/MIERUNE/PLATEAU-GIS-Converter).

## Installation Instructions

To use atlas-packer in your project, you will need Rust installed on your system (via rustup). Include atlas-packer in your Rust project.

```toml
[dependencies]
atlas-packer = { git = "<https://github.com/MIERUNE/atlas-packer.git>", branch = "main" }
```

Run cargo build in your project directory to fetch and compile atlas-packer along with its dependencies. This will make the atlas-packer library available for use in your code.

## Usage Examples

Once installed, atlas-packer can be used in your Rust application to generate texture atlases. Below is a basic example demonstrating how to use the library.

```rust
use std::path::PathBuf;
use atlas_packer::{
    pack::AtlasPacker,
    place::{GuillotineTexturePlacer, TexturePlacerConfig},
    texture::{PolygonMappedTexture, DownsampleFactor, cache::TextureCache},
    export::WebpAtlasExporter
};

fn main() {
    // 1. Prepare a list of textures with their UV polygon coordinates.
    let image_path = PathBuf::from("path/to/image.png");
    // Define UV coordinates for the region of the image to use (here the full image corners):
    let uv_coords = vec![
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0)
    ];
    // (DownsampleFactor of 1.0 means no downscaling; use <1.0 to reduce resolution)
    let downsample = DownsampleFactor::new(&1.0);

    // Each texture region needs a unique ID.
    let texture_id = "example_texture_1";
    let cropped_texture = PolygonMappedTexture::new(&image_path, None, &uv_coords, downsample);

    // 2. Initialize the atlas packer and add the texture region.
    let mut packer = AtlasPacker::default();
    packer.add_texture(texture_id.to_string(), cropped_texture);

    // 3. Configure the atlas size and packing algorithm, then pack the textures.
    let config = TexturePlacerConfig { width: 1024, height: 1024, padding: 0 };
    let placer = GuillotineTexturePlacer::new(config);
    let atlas = packer.pack(placer);  // Pack all added textures into atlas layout

    // 4. Export the resulting atlas image(s) to files.
    let output_dir = std::path::Path::new("./output");
    let cache = TextureCache::new(100_000_000);  // cache to reuse image data (100MB max)
    atlas.export(WebpAtlasExporter::default(), output_dir, &cache, config.width(), config.height()).expect("Export failed");
}
```

In this example, we add one texture (the entire image.png) to the atlas.
In practice, you would repeat the add_texture step for each texture region you want to include. Each region is identified by an id so that after packing you can retrieve its position in the atlas​. We then call pack() with a chosen packing algorithm (here we use the default guillotine algorithm with a 1024×1024 atlas size) to compute the atlas layout.
Finally, we export the atlas to image files on disk using a WebP exporter. The library supports exporting to multiple image formats (WebP in this case, see Features below) and will produce one or more atlas image files depending on how many are needed to fit all input textures.

## Running the Example

The atlas-packer repository includes an example program. After cloning the repo, you can run `cargo run --example test_pack` to see atlas-packer in action. This example will read a set of sample textures and UV coordinates, pack them into an atlas, and save the output images (along with logging the process duration). Check the `examples/` directory for the input assets and the output results.

## Features Overview

**Combine Multiple Textures**: Pack many small texture segments into one or more large textures (atlas). You provide the path/URL to each source image and the UV polygon coordinates of the region to use from that image, and atlas-packer will composite them together​​. This is especially useful for 3D models or sprites that use many tiny texture files.

**Maintains UV Mapping**: Each input texture region is tracked by a unique ID so you can map the atlas output back to your original data. After packing, you can obtain the atlas coordinates of each region via its ID, allowing you to update model UVs or sprite coordinates to use the atlas​.

**Configurable Atlas Size & Padding**: You can specify the atlas dimensions (e.g. 4096×4096 or any size) and padding between textures through a configuration object​. This lets you control how large each atlas image can be and add spacing to avoid bleeding between packed textures if needed.

**Pluggable Packing Algorithms**: The packing algorithm is abstracted behind an interface, so different algorithms can be used. By default, atlas-packer uses a Guillotine algorithm for bin packing​

– this method recursively subdivides free space by cutting rectangles (like a guillotine) as textures are placed. The design allows adding other algorithms in the future without changing your code.

**Duplicate Region Detection**: For efficiency, atlas-packer can detect if the same texture region is requested multiple times. It uses an R-Tree spatial index to quickly find and avoid duplicating identical texture areas​. This texture clustering ensures that shared imagery is only stored once in the atlas, saving space.

**Multi-Threaded Preparation**: The library is thread-safe and can integrate with parallel processing to speed up atlas creation. For example, you can use Rayon to process and crop multiple textures concurrently and add them to the AtlasPacker in parallel​. This is useful when dealing with a very large number of textures.

**Downsampling Support**: Each texture region can optionally be downscaled before packing. You can specify a downsample factor (ranging from 1.0 for full resolution down to 0.0 for maximum reduction) for each texture region​. This allows you to reduce the resolution of certain textures (for example, distant objects) to save space in the atlas.

**Multiple Export Formats**: Atlas images can be exported in various formats. Built-in exporters are provided for WebP, JPEG, and PNG outputs​. You can choose the format that best suits your needs (WebP for higher compression, PNG for lossless, etc.). The exporter system is extensible, so additional formats (e.g., Basis Universal) could be added in the future​.

**Caching for Performance**: The library provides a caching mechanism to optimize performance when reading and writing images. A TextureSizeCache can store image dimensions to avoid recomputing them, and a TextureCache can hold recently used image data in memory to speed up the export process​. This is particularly beneficial when the same source textures are used repeatedly or when writing out very large atlases.

Each of these features makes atlas-packer a flexible and powerful tool for texture atlas generation. Whether you are working on converting 3D city models, packing sprite sheets for a game, or simply consolidating many images, atlas-packer provides the building blocks to automate the atlas creation process efficiently. Enjoy faster rendering and easier texture management with your new atlases!

## Authors

- Satoru Nishio ([@nokonoko1203](https://github.com/nokonoko1203))
- Teruki Tada ([@TadaTeruki](https://github.com/TadaTeruki))
- Yu Osaka ([@yud0uhu](https://github.com/yud0uhu))
