use super::super::{GateKind, MaterialAssets, MeshAssets};
use bevy::prelude::{Commands, Vec3};

pub fn spawn(commands: &mut Commands, meshes: &MeshAssets, materials: &MaterialAssets) {
    let finish_z = 116.0;
    let boss_z = 120.0;
    let track_length = boss_z + 12.0;
    super::super::spawn_ground_with_length(commands, meshes, materials, track_length);
    let camera = super::super::spawn_camera(commands);
    super::super::spawn_player(commands, meshes, materials);
    super::super::spawn_control_hint(commands, camera, meshes, materials);

    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(-2.0, 0.0, 12.0),
        GateKind::Multiply(2),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(2.0, 0.0, 14.0),
        GateKind::Divide(2),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 24.0),
        GateKind::Add(16),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 32.0),
        11,
        Vec3::X,
        2.8,
        3.0,
        0.8,
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(-1.6, 0.0, 50.0),
        GateKind::Add(-18),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(1.6, 0.0, 52.0),
        GateKind::Multiply(3),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 58.0),
        12,
        Vec3::X,
        2.8,
        3.3,
        1.8,
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(-2.0, 0.0, 74.0),
        GateKind::Divide(3),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(2.0, 0.0, 76.0),
        GateKind::Add(22),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 82.0),
        13,
        Vec3::X,
        2.8,
        3.5,
        2.6,
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 98.0),
        13,
        Vec3::X,
        2.8,
        3.7,
        3.4,
    );
    super::super::spawn_finish_line(commands, meshes, materials, Vec3::new(0.0, 0.0, finish_z));
    super::super::spawn_boss(commands, meshes, materials, Vec3::new(0.0, 0.0, boss_z), 68);
}
