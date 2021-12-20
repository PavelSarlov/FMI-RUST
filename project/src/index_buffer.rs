pub struct IndexBuffer {
    indices: &[u32],   
}

impl IndexBuffer {
    pub fn new(indices: &[u32]) -> Self {
        IndexBuffer {
            indices,
        }
    }
}
