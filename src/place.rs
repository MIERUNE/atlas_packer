use std::collections::HashMap;

use crate::{
    texture::{ChildUVPolygon, ClusterBoundingTexture},
    AtlasID, ClusterID, PolygonID,
};

#[derive(Debug, Clone)]
pub struct TexturePlacerConfig {
    pub width: u32,
    pub height: u32,
    pub padding: u32,
    // and more option
    // Allow rotation, allow multiple pages, adjust resolution, specify resampling method, etc...
}

impl Default for TexturePlacerConfig {
    fn default() -> Self {
        TexturePlacerConfig {
            width: 1024,
            height: 1024,
            padding: 0,
        }
    }
}

impl TexturePlacerConfig {
    // Ensure that the width and height are powers of two
    pub fn new(width: u32, height: u32, padding: u32) -> Self {
        TexturePlacerConfig {
            width: width.checked_next_power_of_two().unwrap(),
            height: height.checked_next_power_of_two().unwrap(),
            padding,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn padding(&self) -> u32 {
        self.padding
    }
}

#[derive(Debug, Clone)]
pub struct PlacedTextureGeometry {
    pub cluster_id: ClusterID,
    pub atlas_id: AtlasID,
    // Pixel coordinates on atlas
    pub origin: (u32, u32),
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct PlacedUVPolygon {
    pub polygon_id: PolygonID,
    pub cluster_id: ClusterID,
    pub atlas_id: AtlasID,
    // UV coordinates on atlas
    pub placed_uv_coords: Vec<(f64, f64)>,
}

pub trait TexturePlacer: Send + Sync {
    fn config(&self) -> &TexturePlacerConfig;

    fn place_texture(
        &mut self,
        bounding_texture: ClusterBoundingTexture,
        children: Vec<(PolygonID, ChildUVPolygon)>,
        cluster_id: ClusterID,
        parent_atlas_id: AtlasID,
    ) -> (PlacedTextureGeometry, Vec<Option<PlacedUVPolygon>>);

    fn can_place(&self, texture: &ClusterBoundingTexture) -> bool;

    fn reset_param(&mut self);

    fn scale_dimensions(&self, width: u32, height: u32, downsample_factor: f32) -> (u32, u32) {
        let scaled_width = (width as f32 * downsample_factor).max(1.0) as u32;
        let scaled_height = (height as f32 * downsample_factor).max(1.0) as u32;
        (scaled_width, scaled_height)
    }
}

pub struct GuillotineTexturePlacer {
    config: TexturePlacerConfig,
    free_rects: Vec<Rect>,
    used_rects: HashMap<String, PlacedTextureGeometry>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl GuillotineTexturePlacer {
    pub fn new(config: TexturePlacerConfig) -> Self {
        let initial_rect = Rect {
            x: 0,
            y: 0,
            width: config.width,
            height: config.height,
        };
        GuillotineTexturePlacer {
            config,
            free_rects: vec![initial_rect],
            used_rects: HashMap::new(),
        }
    }

    fn find_best_rect(&self, width: u32, height: u32) -> Option<Rect> {
        self.free_rects
            .iter()
            .filter(|&rect| rect.width >= width && rect.height >= height)
            .min_by_key(|&rect| rect.width * rect.height)
            .cloned()
    }

    fn split_rect(&mut self, rect: Rect, placed: &PlacedTextureGeometry) {
        let padding = self.config.padding;
        let (right_rect, bottom_rect) = if rect.width <= rect.height {
            (
                Rect {
                    x: rect.x + placed.width + padding,
                    y: rect.y,
                    width: rect.width - placed.width - padding,
                    height: placed.height,
                },
                Rect {
                    x: rect.x,
                    y: rect.y + placed.height + padding,
                    width: rect.width,
                    height: rect.height - placed.height - padding,
                },
            )
        } else {
            (
                Rect {
                    x: rect.x + placed.width + padding,
                    y: rect.y,
                    width: rect.width - placed.width - padding,
                    height: rect.height,
                },
                Rect {
                    x: rect.x,
                    y: rect.y + placed.height + padding,
                    width: placed.width,
                    height: rect.height - placed.height - padding,
                },
            )
        };

        if right_rect.width > 0 && right_rect.height > 0 {
            self.free_rects.push(right_rect);
        }
        if bottom_rect.width > 0 && bottom_rect.height > 0 {
            self.free_rects.push(bottom_rect);
        }
    }

    fn merge_free_rects(&mut self) {
        let mut i = 0;
        while i < self.free_rects.len() {
            let mut merged = false;
            let rect1 = self.free_rects[i];
            let mut j = i + 1;
            while j < self.free_rects.len() {
                let rect2 = self.free_rects[j];
                if let Some(merged_rect) = Self::try_merge_rects(rect1, rect2) {
                    self.free_rects[i] = merged_rect;
                    self.free_rects.swap_remove(j);
                    merged = true;
                    break;
                }
                j += 1;
            }
            if !merged {
                i += 1;
            }
        }
    }

    fn try_merge_rects(rect1: Rect, rect2: Rect) -> Option<Rect> {
        if rect1.x == rect2.x && rect1.width == rect2.width && rect1.y + rect1.height == rect2.y {
            Some(Rect {
                x: rect1.x,
                y: rect1.y,
                width: rect1.width,
                height: rect1.height + rect2.height,
            })
        } else if rect1.y == rect2.y
            && rect1.height == rect2.height
            && rect1.x + rect1.width == rect2.x
        {
            Some(Rect {
                x: rect1.x,
                y: rect1.y,
                width: rect1.width + rect2.width,
                height: rect1.height,
            })
        } else {
            None
        }
    }

    fn cropped_uv_to_placed_uv(
        &self,
        rect: Rect,
        uv: (f64, f64),
        width: u32,
        height: u32,
    ) -> (f64, f64) {
        let (x, y) = self.uv_to_pixel(uv, width, height);
        (
            (rect.x as f64 + self.config.padding as f64 + x as f64) / self.config.width as f64,
            1.0 - ((rect.y as f64 + self.config.padding as f64 + y as f64)
                / self.config.height as f64),
        )
    }

    fn uv_to_pixel(&self, uv: (f64, f64), width: u32, height: u32) -> (u32, u32) {
        let x = (uv.0 * width as f64) as u32;
        let y = ((1.0 - uv.1) * height as f64) as u32;
        (x, y)
    }
}

impl TexturePlacer for GuillotineTexturePlacer {
    fn config(&self) -> &TexturePlacerConfig {
        &self.config
    }

    fn place_texture(
        &mut self,
        bounding_texture: ClusterBoundingTexture,
        children: Vec<(PolygonID, ChildUVPolygon)>,
        cluster_id: ClusterID,
        parent_atlas_id: AtlasID,
    ) -> (PlacedTextureGeometry, Vec<Option<PlacedUVPolygon>>) {
        let (_, _, buffered_width, buffered_height) = bounding_texture.get_buffered_geometry();
        let (scaled_width, scaled_height) = self.scale_dimensions(
            buffered_width,
            buffered_height,
            bounding_texture.downsample_factor.value(),
        );

        if let Some(rect) = self.find_best_rect(
            scaled_width + self.config.padding,
            scaled_height + self.config.padding,
        ) {
            let bounding_placed = PlacedTextureGeometry {
                cluster_id: cluster_id.clone(),
                atlas_id: parent_atlas_id,
                origin: (rect.x + self.config.padding, rect.y + self.config.padding),
                width: scaled_width,
                height: scaled_height,
            };

            let children_placed = children
                .iter()
                .map(|(polygon_id, uv_polygon)| {
                    let placed_uv_coords = uv_polygon
                        .cropped_uv_coords
                        .iter()
                        .map(|&(u, v)| {
                            self.cropped_uv_to_placed_uv(rect, (u, v), scaled_width, scaled_height)
                        })
                        .collect::<Vec<(f64, f64)>>();
                    Some(PlacedUVPolygon {
                        polygon_id: polygon_id.clone(),
                        cluster_id: cluster_id.clone(),
                        atlas_id: parent_atlas_id,
                        placed_uv_coords,
                    })
                })
                .collect::<Vec<Option<PlacedUVPolygon>>>();

            self.used_rects.insert(cluster_id, bounding_placed.clone());
            self.free_rects.retain(|r| r != &rect);
            self.split_rect(rect, &bounding_placed);
            self.merge_free_rects();
            (bounding_placed, children_placed)
        } else {
            // todo: Consideration of processing when the texture is larger than the atlas size
            panic!("Texture could not be placed: {}", cluster_id);
        }
    }

    fn can_place(&self, texture: &ClusterBoundingTexture) -> bool {
        let (_, _, buffered_width, buffered_height) = texture.get_buffered_geometry();
        let (scaled_width, scaled_height) = self.scale_dimensions(
            buffered_width,
            buffered_height,
            texture.downsample_factor.value(),
        );
        let width = scaled_width + self.config.padding;
        let height = scaled_height + self.config.padding;
        self.free_rects
            .iter()
            .any(|r| r.width >= width && r.height >= height)
    }

    fn reset_param(&mut self) {
        let initial_rect = Rect {
            x: 0,
            y: 0,
            width: self.config.width,
            height: self.config.height,
        };
        self.free_rects = vec![initial_rect];
        self.used_rects.clear();
    }
}
