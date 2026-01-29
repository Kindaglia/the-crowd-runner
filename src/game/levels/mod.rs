pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;

use super::{MaterialAssets, MeshAssets};
use bevy::prelude::Commands;

pub fn spawn_level(
    level_index: usize,
    commands: &mut Commands,
    meshes: &MeshAssets,
    materials: &MaterialAssets,
) {
    match level_index {
        0 => level1::spawn(commands, meshes, materials),
        1 => level2::spawn(commands, meshes, materials),
        2 => level3::spawn(commands, meshes, materials),
        3 => level4::spawn(commands, meshes, materials),
        _ => level1::spawn(commands, meshes, materials),
    }
}
