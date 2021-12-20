use crate::vertex_wrap::VertexWrap;

pub struct VertexBuffer {
    verts: &[VertexWrap];
}

impl VertexBuffer {
    pub fn new(verts: &[VertexWrap]) -> Self {
        VertexBuffer {
            verts,
        }
    }
}
