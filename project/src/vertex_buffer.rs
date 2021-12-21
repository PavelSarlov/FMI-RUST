use crate::vertex_wrap::VertexWrap;

pub struct VertexBuffer {
    verts: Option<&[VertexWrap]>,
}

impl VertexBuffer {
    pub fn new() -> Self {
        VertexBuffer {
            None,
        }
    }

    pub fn set(&mut self, verts: &[VertexWrap]) {
        self.verts = Some(verts.clone());
    }

    pub fn get(&mut self) -> Option<&[VertexWrap]> {
        self.verts
    }
}
