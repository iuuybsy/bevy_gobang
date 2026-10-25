use bevy::prelude::*;

use crate::utils::*;

use crate::consts::*;
use crate::resources::*;

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

pub fn cursor_to_board(cursor: Vec2, plot_info: &Res<PlotInfo>) -> Option<(f32, f32)> {
    let ori_x = 0.5 * plot_info.window_size.x;
    let ori_y = 0.5 * plot_info.window_size.y;

    let delta_x = cursor.x - ori_x;
    let delta_y = cursor.y - ori_y;

    let num_x =
        (delta_x / (plot_info.outter_radius * 2.0) + plot_info.center).round() - plot_info.center;
    let num_y =
        (-delta_y / (plot_info.outter_radius * 2.0) + plot_info.center).round() - plot_info.center;

    if num_x < -plot_info.center
        || num_y < -plot_info.center
        || num_x > plot_info.center
        || num_y > plot_info.center
    {
        None
    } else {
        Some((num_x, num_y))
    }
}
