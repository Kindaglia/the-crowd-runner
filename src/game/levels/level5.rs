use super::super::{GateKind, MaterialAssets, MeshAssets};
use bevy::prelude::{Commands, Vec3};

pub fn spawn(commands: &mut Commands, meshes: &MeshAssets, materials: &MaterialAssets) {
    let finish_z = 94.0;
    let boss_z = 98.0;
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
        Vec3::new(2.0, 0.0, 12.0),
        GateKind::Add(-8),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 22.0),
        GateKind::Divide(2),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 30.0),
        9,
        Vec3::X,
        2.4,
        2.4,
        0.3,
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(-1.6, 0.0, 46.0),
        GateKind::Add(18),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(1.6, 0.0, 46.0),
        GateKind::Divide(3),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 54.0),
        10,
        Vec3::X,
        2.6,
        2.8,
        1.1,
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(-2.0, 0.0, 70.0),
        GateKind::Multiply(2),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_gate_with_label_offset(
        commands,
        meshes,
        materials,
        Vec3::new(2.0, 0.0, 70.0),
        GateKind::Add(-14),
        Vec3::new(0.0, 1.2, 0.0),
    );
    super::super::spawn_moving_obstacle(
        commands,
        meshes,
        materials,
        Vec3::new(0.0, 0.0, 78.0),
        11,
        Vec3::X,
        2.6,
        3.0,
        2.4,
    );
    super::super::spawn_finish_line(commands, meshes, materials, Vec3::new(0.0, 0.0, finish_z));
    super::super::spawn_boss(commands, meshes, materials, Vec3::new(0.0, 0.0, boss_z), 56);
}
