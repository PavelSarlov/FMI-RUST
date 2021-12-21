pub struct IndexBuffer {
    indices: Option<&[u32]>,
}

impl IndexBuffer {
    pub fn new() -> Self {
        IndexBuffer {
            None,
        }
    }

    pub fn set(&mut self, indices: &[u32]) {
        self.indices = Some(indices.clone());
    }

    pub fn get(&mut self) -> Option<&[u32]> {
        self.indices
    }
}
