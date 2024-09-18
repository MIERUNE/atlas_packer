/*
use hashbrown::HashMap;

use crate::{
    disjoint_set::DisjointSet,
    texture::CroppedTexture,
};

pub struct CroppedTextureCluster {
    pub toplevel: CroppedTexture,
    pub children: Vec<CroppedTexture>,
}

pub struct ClusterBuilder {
    pub cropped_clusters: Vec<CroppedTexture>,
}

impl ClusterBuilder {
    pub fn new(cropped_clusters: Vec<CroppedTexture>) -> Self {
        ClusterBuilder { cropped_clusters }
    }

    // cropped textures -> cluster
    // This function doesn't check whether the cropped textures are overlapped or not.
    fn gather_cropped_textures(
        &self,
        cropped_textures: &[CroppedTexture],
    ) -> CroppedTextureCluster {
        //todo!();

    }

    pub fn build(&self) -> Vec<CroppedTextureCluster> {
        let mut clusters_map: HashMap<String, Vec<&CroppedTexture>> = HashMap::new();

        // Group cropped textures by their image path
        for cropped_texture in self.cropped_clusters.iter() {
            let uri = cropped_texture.image_path.to_string_lossy().to_string();
            let clusters = clusters_map.entry(uri).or_insert_with(Vec::new);
            clusters.push(cropped_texture);
        }

        let mut clusters: Vec<CroppedTextureCluster> = Vec::new();
        for (_, cropped_textures) in clusters_map.iter() {
            let mut disjoint_set = DisjointSet::new(self.cropped_clusters.len());
            for i in 0..cropped_textures.len() {
                for j in i + 1..cropped_textures.len() {
                    let a = cropped_textures[i];
                    let b = cropped_textures[j];
                    if a.overlaps(b) {
                        disjoint_set.unite(i, j);
                    }
                }
            }

            let mut raw_clusters: HashMap<usize, Vec<CroppedTexture>> = HashMap::new();
            for (i, &cropped_texture) in cropped_textures.iter().enumerate() {
                let root = disjoint_set.root(i);
                let cluster: &mut Vec<CroppedTexture> = raw_clusters.entry(root).or_insert_with(Vec::new);
                cluster.push(cropped_texture.clone());
            }

            // Build clusters
            for (_, cropped_textures) in raw_clusters.iter() {
                let cluster = self.gather_cropped_textures(cropped_textures);
                clusters.push(cluster);
            }
        }
        clusters
    }
}

*/
