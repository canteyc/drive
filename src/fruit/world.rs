use bevy::prelude::*;
use bevy::color::palettes::basic::*;

pub const RIGHT: f32 = 300.;
pub const LEFT: f32 = RIGHT * -1.0;
pub const TOP: f32 = 300.;
pub const BOTTOM: f32 = -300.;
pub const THICKNESS: f32 = 2.;

#[derive(Component, Default)]
pub struct Wall;

pub fn load_container(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let layer = 100.;
    let color = Color::from(WHITE);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(THICKNESS, TOP - BOTTOM))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(RIGHT, 0., layer),
        Wall,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(THICKNESS, TOP - BOTTOM))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(LEFT, 0., layer),
        Wall,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(RIGHT - LEFT, THICKNESS))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., BOTTOM, layer),
        Wall,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(RIGHT - LEFT, THICKNESS))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., TOP, layer),
        Wall,
    ));
}
