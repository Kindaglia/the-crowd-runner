use super::super::{GateKind, MaterialAssets, MeshAssets};
use bevy::prelude::{Commands, Vec3};

pub fn spawn(commands: &mut Commands, meshes: &MeshAssets, materials: &MaterialAssets) {
    let finish_z = 146.0;
    let boss_z = 150.0;
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
        GateKind::Add(-12),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 26.0),
        GateKind::Divide(2),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 36.0),
        13,
        Vec3::X,
        2.8,
        3.8,
        1.2,
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(-1.6, 0.0, 54.0),
        GateKind::Add(24),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(1.6, 0.0, 56.0),
        GateKind::Divide(3),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 62.0),
        14,
        Vec3::X,
        2.8,
        4.0,
        2.2,
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(-2.0, 0.0, 80.0),
        GateKind::Multiply(3),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(2.0, 0.0, 82.0),
        GateKind::Divide(4),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 88.0),
        15,
        Vec3::X,
        2.8,
        4.2,
        3.2,
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 106.0),
        15,
        Vec3::X,
        2.8,
        4.4,
        3.8,
    );
    super::super::spawn_finish_line(commands, meshes, materials, Vec3::new(0.0, 0.0, finish_z));
    super::super::spawn_boss(commands, meshes, materials, Vec3::new(0.0, 0.0, boss_z), 86);
}
