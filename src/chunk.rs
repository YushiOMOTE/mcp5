use bevy::{
    math::UVec3,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
};
use block_mesh::{
    greedy_quads,
    ndshape::{ConstShape, ConstShape3u32},
    GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG,
};

use crate::voxel::Voxel;

// 32 x 64 x 32 voxels in a chunk
const CHUNK_VOXELS: UVec3 = UVec3::new(32, 64, 32);

// block mesh parameters; +2 of chunk size as block-mesh requires 1-voxel boundary padding for each side
const CHUNK_SHAPE_SIZE_X: u32 = CHUNK_VOXELS.x + 2;
const CHUNK_SHAPE_SIZE_Y: u32 = CHUNK_VOXELS.y + 2;
const CHUNK_SHAPE_SIZE_Z: u32 = CHUNK_VOXELS.z + 2;
const CHUNK_SHAPE_MIN_BUF: [u32; 3] = [0; 3];
const CHUNK_SHAPE_MAX_BUF: [u32; 3] = [
    CHUNK_SHAPE_SIZE_X - 1,
    CHUNK_SHAPE_SIZE_Y - 1,
    CHUNK_SHAPE_SIZE_Z - 1,
];

pub type ChunkShape = ConstShape3u32<CHUNK_SHAPE_SIZE_X, CHUNK_SHAPE_SIZE_Y, CHUNK_SHAPE_SIZE_Z>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct Chunk {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Chunk {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn from_world_coord(coord: Vec3) -> Self {
        let chunk_coord = coord / Self::size();

        Self {
            x: chunk_coord.x as i64,
            y: chunk_coord.y as i64,
            z: chunk_coord.z as i64,
        }
    }

    /// Size in the world coordiate
    pub fn size() -> Vec3 {
        Vec3::new(
            CHUNK_VOXELS.x as f32,
            CHUNK_VOXELS.y as f32,
            CHUNK_VOXELS.z as f32,
        ) * Self::voxel_size()
    }

    /// Position in the world coordinate
    pub fn position(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32) * Self::size()
    }

    /// Length of voxel edge
    pub fn voxel_size() -> f32 {
        1.0
    }

    pub fn generate_mesh(&self) -> Mesh {
        let voxels: Vec<_> = (0..ChunkShape::SIZE)
            .map(|i| {
                let [x, y, z] = ChunkShape::delinearize(i);
                let (base_x, base_y, base_z) = self.voxel_coord();
                let (x, y, z) = (base_x + x as i64, base_y + y as i64, base_z + z as i64);
                generate_voxels(x, y, z)
            })
            .collect();

        let mut buffer = GreedyQuadsBuffer::new(voxels.len());

        greedy_quads(
            &voxels,
            &ChunkShape {},
            CHUNK_SHAPE_MIN_BUF,
            CHUNK_SHAPE_MAX_BUF,
            &RIGHT_HANDED_Y_UP_CONFIG.faces,
            &mut buffer,
        );

        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
        let num_indices = buffer.quads.num_quads() * 6;
        let num_vertices = buffer.quads.num_quads() * 4;
        let mut indices = Vec::with_capacity(num_indices);
        let mut positions = Vec::with_capacity(num_vertices);
        let mut normals = Vec::with_capacity(num_vertices);

        for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
            for quad in group.into_iter() {
                // construct vectors for mesh
                let i = face.quad_mesh_indices(positions.len() as u32);
                indices.extend_from_slice(&i);
                let p = face.quad_mesh_positions(&quad, Self::voxel_size());
                positions.extend_from_slice(&p);
                normals.extend_from_slice(&face.quad_mesh_normals());
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(positions),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(normals),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices]),
        );
        mesh.set_indices(Some(Indices::U32(indices.clone())));

        mesh
    }

    fn voxel_coord(&self) -> (i64, i64, i64) {
        (
            self.x * CHUNK_VOXELS.x as i64,
            self.y * CHUNK_VOXELS.y as i64,
            self.z * CHUNK_VOXELS.z as i64,
        )
    }
}

fn generate_voxels(x: i64, y: i64, z: i64) -> Voxel {
    let g = crate::map::ProcGen::new(crate::map::map_cfg());
    let v = g.gen(x, z);
    if (y as f32) <= v * 10.0 + 1.0 {
        Voxel::new((v * 10.0) as u64)
    } else {
        Voxel::EMPTY
    }
}
