use super::super::{GateKind, MaterialAssets, MeshAssets};
use bevy::prelude::{Commands, Vec3};

pub fn spawn(commands: &mut Commands, meshes: &MeshAssets, materials: &MaterialAssets) {
    let finish_z = 164.0;
    let boss_z = 168.0;
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
        Vec3::new(0.0, 0.0, 26.0),
        GateKind::Add(18),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 36.0),
        14,
        Vec3::X,
        2.8,
        4.2,
        1.4,
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(-1.6, 0.0, 56.0),
        GateKind::Add(-22),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(1.6, 0.0, 58.0),
        GateKind::Multiply(3),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 66.0),
        15,
        Vec3::X,
        2.8,
        4.4,
        2.4,
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(-2.0, 0.0, 86.0),
        GateKind::Divide(3),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(2.0, 0.0, 88.0),
        GateKind::Add(26),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 96.0),
        16,
        Vec3::X,
        2.8,
        4.6,
        3.4,
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 116.0),
        16,
        Vec3::X,
        2.8,
        4.8,
        4.1,
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 138.0),
        17,
        Vec3::X,
        2.8,
        5.0,
        4.8,
    );
    super::super::spawn_finish_line(commands, meshes, materials, Vec3::new(0.0, 0.0, finish_z));
    super::super::spawn_boss(commands, meshes, materials, Vec3::new(0.0, 0.0, boss_z), 98);
}
