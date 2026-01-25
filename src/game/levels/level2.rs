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
        Vec3::new(-2.0, 0.0, 12.0),
        GateKind::Add(8),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(2.0, 0.0, 12.0),
        GateKind::Add(-6),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 22.0),
        GateKind::Multiply(2),
    );
    super::super::spawn_obstacle(commands, meshes, materials, Vec3::new(-1.4, 0.0, 36.0), 7);
    super::super::spawn_enemy_group(commands, meshes, materials, Vec3::new(1.0, 0.0, 50.0), 18);
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(-1.8, 0.0, 62.0),
        GateKind::Add(14),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(1.8, 0.0, 62.0),
        GateKind::Add(-8),
    );
    super::super::spawn_obstacle(commands, meshes, materials, Vec3::new(1.6, 0.0, 71.0), 6);
    super::super::spawn_enemy_group(commands, meshes, materials, Vec3::new(0.0, 0.0, 78.0), 26);
    super::super::spawn_finish_line(commands, meshes, materials, Vec3::new(0.0, 0.0, 83.0));
    super::super::spawn_boss(commands, meshes, materials, Vec3::new(0.0, 0.0, 87.0), 38);
}
