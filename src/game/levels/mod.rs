pub mod level1;
pub mod level10;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;
pub mod level7;
pub mod level8;
pub mod level9;

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
        4 => level5::spawn(commands, meshes, materials),
        5 => level6::spawn(commands, meshes, materials),
        6 => level7::spawn(commands, meshes, materials),
        7 => level8::spawn(commands, meshes, materials),
        8 => level9::spawn(commands, meshes, materials),
        9 => level10::spawn(commands, meshes, materials),
        _ => level1::spawn(commands, meshes, materials),
    }
}
