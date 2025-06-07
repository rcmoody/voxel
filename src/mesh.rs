use crate::chunk::Chunk;

#[repr(C)]
pub struct Vertex {
    position: [f32; 3],
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<usize>,
}

pub trait GenerateMesh {
    fn generate_mesh(&self) -> Mesh;
}
