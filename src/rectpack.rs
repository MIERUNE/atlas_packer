#[derive(Clone, Copy)]
pub struct Rectangle {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl Rectangle {
    pub fn width(&self) -> u32 {
        self.right - self.left
    }

    pub fn height(&self) -> u32 {
        self.bottom - self.top
    }

    pub fn fits(&self, img: &Image) -> bool {
        self.width() >= img.width && self.height() >= img.height
    }

    pub fn fits_perfectly(&self, img: &Image) -> bool {
        self.width() == img.width && self.height() == img.height
    }
}

pub struct Image {
    pub id: String,
    pub width: u32,
    pub height: u32,
}

pub struct Node {
    child: [Option<Box<Node>>; 2],
    rect: Rectangle,
    image_id: Option<i32>,
}

impl Node {
    pub fn new(rect: Rectangle) -> Self {
        Node {
            child: [None, None],
            rect,
            image_id: None,
        }
    }

    pub fn insert(&mut self, img: &Image) -> Option<&mut Node> {
        if let Some(ref mut child0) = self.child[0] {
            if let Some(new_node) = child0.insert(img) {
                return Some(new_node);
            }
        } else {
            if self.image_id.is_some() {
                return None;
            }

            if !self.rect.fits(img) {
                return None;
            }

            if self.rect.fits_perfectly(img) {
                // self.image_id = Some(img.id);
                return Some(self);
            }

            let child0_rect;
            let child1_rect;
            let dw = self.rect.width() - img.width;
            let dh = self.rect.height() - img.height;

            if dw > dh {
                child0_rect = Rectangle {
                    left: self.rect.left,
                    top: self.rect.top,
                    right: self.rect.left + img.width - 1,
                    bottom: self.rect.bottom,
                };
                child1_rect = Rectangle {
                    left: self.rect.left + img.width,
                    top: self.rect.top,
                    right: self.rect.right,
                    bottom: self.rect.bottom,
                };
            } else {
                child0_rect = Rectangle {
                    left: self.rect.left,
                    top: self.rect.top,
                    right: self.rect.right,
                    bottom: self.rect.top + img.height - 1,
                };
                child1_rect = Rectangle {
                    left: self.rect.left,
                    top: self.rect.top + img.height,
                    right: self.rect.right,
                    bottom: self.rect.bottom,
                };
            }

            self.child[0] = Some(Box::new(Node::new(child0_rect)));
            self.child[1] = Some(Box::new(Node::new(child1_rect)));

            if let Some(ref mut child0) = self.child[0] {
                return child0.insert(img);
            }
        }
        None
    }
}
