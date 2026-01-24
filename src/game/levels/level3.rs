use super::super::{GateKind, MaterialAssets, MeshAssets};
use bevy::prelude::{Commands, Vec3};

pub fn spawn(commands: &mut Commands, meshes: &MeshAssets, materials: &MaterialAssets) {
    super::super::spawn_ground(commands, meshes, materials);
    let camera = super::super::spawn_camera(commands);
    super::super::spawn_player(commands, meshes, materials);
    super::super::spawn_control_hint(commands, camera, meshes, materials);

    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(-1.6, 0.0, 13.0),
        GateKind::Multiply(2),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(1.6, 0.0, 13.0),
        GateKind::Add(-5),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 23.0),
        GateKind::Add(10),
    );
    super::super::spawn_obstacle(commands, meshes, materials, Vec3::new(0.0, 0.0, 37.0), 7);
    super::super::spawn_enemy_group(commands, meshes, materials, Vec3::new(-1.0, 0.0, 49.0), 17);
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(-1.8, 0.0, 60.0),
        GateKind::Add(12),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(1.8, 0.0, 60.0),
        GateKind::Multiply(2),
    );
    super::super::spawn_obstacle(commands, meshes, materials, Vec3::new(-1.4, 0.0, 71.5), 6);
    super::super::spawn_enemy_group(commands, meshes, materials, Vec3::new(0.0, 0.0, 77.5), 27);
    super::super::spawn_finish_line(commands, meshes, materials, Vec3::new(0.0, 0.0, 82.5));
    super::super::spawn_boss(commands, meshes, materials, Vec3::new(0.0, 0.0, 86.5), 39);
}
