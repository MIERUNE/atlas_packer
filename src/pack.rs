use std::path::Path;

use hashbrown::HashMap;
use rayon::prelude::*;

use crate::export::AtlasExporter;
use crate::place::{PlacedTextureInfo, TexturePlacer};
use crate::texture::{CroppedTexture, TextureCache};

pub type Atlas = Vec<PlacedTextureInfo>;

pub struct TexturePackerBuilder<P: TexturePlacer> {
    placer: P,
    // texture id -> texture
    textures: HashMap<String, CroppedTexture>,
}

impl<P: TexturePlacer> TexturePackerBuilder<P> {
    pub fn new(placer: P) -> Self {
        TexturePackerBuilder {
            placer,
            textures: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, texture_id: String, texture: CroppedTexture) {
        self.textures.insert(texture_id, texture);
    }

    pub fn build(mut self) -> TextureAtlasProvider {
        // See the definition of TextureAtlasProvider for more information about the below variables
        let mut current_atlas: Atlas = Vec::new();
        let mut atlases: HashMap<String, Atlas> = HashMap::new();
        let mut texture_info_map: HashMap<String, (String, usize)> = HashMap::new();

        for (texture_id, texture) in self.textures.iter() {
            let (atlas_id, atlas_index) = {
                let current_atlas_id = atlases.len();

                if self.placer.can_place(&texture) {
                    let texture_info = self.placer.place_texture(
                        &texture_id,
                        &texture,
                        current_atlas_id.to_string().as_ref(),
                    );
                    current_atlas.push(texture_info.clone());
                    (current_atlas_id.to_string(), current_atlas.len() - 1)
                } else {
                    atlases.insert(current_atlas_id.to_string(), current_atlas.clone());
                    current_atlas.clear();
                    self.placer.reset_param();

                    let current_atlas_id = atlases.len();

                    let texture_info = self.placer.place_texture(
                        &texture_id,
                        &texture,
                        current_atlas_id.to_string().as_ref(),
                    );
                    current_atlas.push(texture_info.clone());
                    (current_atlas_id.to_string(), current_atlas.len() - 1)
                }
            };
            texture_info_map.insert(texture_id.clone(), (atlas_id, atlas_index));
        }

        // treat the last atlas
        if !current_atlas.is_empty() {
            let current_atlas_id = atlases.len();

            atlases.insert(current_atlas_id.to_string(), current_atlas.clone());
            current_atlas.clear();
        }

        TextureAtlasProvider {
            textures: self.textures,
            atlases,
            texture_info_map,
        }
    }
}

pub struct TextureAtlasProvider {
    // texture id -> texture
    textures: HashMap<String, CroppedTexture>,
    // atlas id -> atlas
    atlases: HashMap<String, Atlas>,
    // texture id -> (atlas id, index in the atlas)
    texture_info_map: HashMap<String, (String, usize)>,
}

impl TextureAtlasProvider {
    pub fn export<E: AtlasExporter>(
        &self,
        exporter: E,
        output_dir: &Path,
        texture_cache: &TextureCache,
        width: u32,
        height: u32,
    ) {
        self.atlases.par_iter().for_each(|(id, atlas)| {
            let output_path = output_dir.join(id);
            exporter.export(
                atlas,
                &self.textures,
                &output_path,
                texture_cache,
                width,
                height,
            );
        });
    }

    pub fn get_texture_info(&self, id: &str) -> Option<&PlacedTextureInfo> {
        let (atlas_id, altas_index) = self.texture_info_map.get(id)?;
        let atlas = self.atlases.get(atlas_id)?;
        atlas.get(*altas_index)
    }
}
