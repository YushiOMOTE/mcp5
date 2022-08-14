use block_mesh::{MergeVoxel, VoxelVisibility};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Voxel(Option<u64>);

impl Voxel {
    pub const EMPTY: Voxel = Voxel(None);

    pub fn new(value: u64) -> Self {
        Self(Some(value))
    }

    fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl block_mesh::Voxel for Voxel {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.is_empty() {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for Voxel {
    type MergeValue = Option<u64>;

    fn merge_value(&self) -> Self::MergeValue {
        self.0
    }
}
