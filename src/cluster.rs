/// Very simple disjoint set used to cluster the cropped textures
/// - fixed size
/// - cannot divide the union
struct DisjointSet {
    parent: Vec<usize>,
}

// TODO (optional): compress the path
impl DisjointSet {
    pub fn new(num_elements: usize) -> Self {
        DisjointSet {
            parent: (0..num_elements).collect(),
        }
    }

    pub fn root(&self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let root = self.root(self.parent[x]);
            root
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        self.parent[root_x] = root_y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_disjoint_set_unite() {
        let mut ds = DisjointSet::new(5);
        ds.unite(0, 1);
        ds.unite(3, 4);
        ds.unite(1, 2);
        assert_eq!(ds.root(0), ds.root(2));
        assert_ne!(ds.root(0), ds.root(3));

        ds.unite(0, 3);
        assert_eq!(ds.root(0), ds.root(4));
    }
}
