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
        GateKind::Multiply(2),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(2.0, 0.0, 12.0),
        GateKind::Add(-10),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 22.0),
        GateKind::Add(14),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 28.0),
        9,
        Vec3::X,
        2.6,
        2.2,
        0.0,
    );
    super::super::spawn_enemy_group(commands, meshes, materials, Vec3::new(-0.8, 0.0, 36.0), 22);
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(-1.6, 0.0, 44.0),
        GateKind::Multiply(2),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(1.6, 0.0, 44.0),
        GateKind::Add(-12),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 50.0),
        10,
        Vec3::X,
        2.8,
        2.6,
        1.2,
    );
    super::super::spawn_enemy_group(commands, meshes, materials, Vec3::new(0.8, 0.0, 56.0), 28);
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(-2.0, 0.0, 62.0),
        GateKind::Add(18),
    );
    super::super::spawn_gate(
        commands,
        meshes,
        materials,
        Vec3::new(2.0, 0.0, 62.0),
        GateKind::Multiply(3),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 68.0),
        12,
        Vec3::X,
        2.4,
        3.0,
        2.4,
    );
    super::super::spawn_enemy_group(commands, meshes, materials, Vec3::new(0.0, 0.0, 74.0), 34);
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 80.0),
        12,
        Vec3::X,
        2.6,
        3.2,
        3.1,
    );
    super::super::spawn_finish_line(commands, meshes, materials, Vec3::new(0.0, 0.0, 86.0));
    super::super::spawn_boss(commands, meshes, materials, Vec3::new(0.0, 0.0, 90.0), 52);
}
