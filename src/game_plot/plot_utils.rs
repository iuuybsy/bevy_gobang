use bevy::prelude::*;

use crate::utils::*;

use crate::consts::*;

pub fn draw_single_piece<T: Component + Clone>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    circle_radius: f32,
    circle_color: (u8, u8, u8),
    use_dark: bool,
    square_len: f32,
    square_color: (u8, u8, u8),
    pos: (f32, f32, f32),
    piece_struct: T,
) {
    let circle = meshes.add(Circle::new(circle_radius));
    let color = color_rgb(circle_color, use_dark);
    commands.spawn((
        Mesh2d(circle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos.0, pos.1, pos.2),
        piece_struct.clone(),
    ));

    let square = meshes.add(Rectangle::new(square_len, square_len));
    let color = color_rgb(square_color, false);
    commands.spawn((
        Mesh2d(square),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos.0, pos.1, pos.2),
        piece_struct.clone(),
    ));
}

pub fn board_metrics(window: &Window) -> (f32, f32) {
    let min_side = window.resolution.width().min(window.resolution.height());
    let outter_radius = min_side * SCREEN_SIDE_LENGTH_RATIO / BOARD_LOGICAL_LEN as f32 / 2.0;
    let center = (BOARD_LOGICAL_LEN as f32 - 1.0) / 2.0;
    (outter_radius, center)
}
