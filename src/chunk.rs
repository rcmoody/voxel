use crate::mesh::{GenerateMesh, Mesh};

pub const CHUNK_VOLUME: usize = CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH;
pub const CHUNK_WIDTH: usize = 15;
pub const CHUNK_HEIGHT: usize = 255;
pub const CHUNK_DEPTH: usize = 15;

#[derive(Copy, Clone)]
pub enum Voxel {
    Air,
    Dirt,
    Grass,
    Stone,
}

pub struct Chunk {
    voxels: [Voxel; CHUNK_VOLUME],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            voxels: [Voxel::Air; CHUNK_VOLUME],
        }
    }
}

impl GenerateMesh for Chunk {
    fn generate_mesh(&self) -> Mesh {
        todo!();
    }
}
