use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::consts::*;
use crate::resources::*;
use crate::utils::*;

fn draw_coin<T: Component + Clone>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    circle_radius: f32,
    circle_color: (u8, u8, u8),
    square_len: f32,
    square_color: (u8, u8, u8),
    pos: (f32, f32, f32),
    piece_struct: T,
) {
    let circle = meshes.add(Circle::new(circle_radius));
    let color = color_rgb(circle_color);
    commands.spawn((
        Mesh2d(circle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos.0, pos.1, pos.2),
        piece_struct.clone(),
    ));

    let square = meshes.add(Rectangle::new(square_len, square_len));
    let color = color_rgb(square_color);
    commands.spawn((
        Mesh2d(square),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos.0, pos.1, pos.2),
        piece_struct.clone(),
    ));
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    existing_pieces: Query<Entity, With<BackGroundPieceStrct>>,
    mut last_size: Local<Vec2>,
) {
    let Ok(game_window) = window_query.single() else {
        return;
    };

    let size = Vec2::new(
        game_window.resolution.width(),
        game_window.resolution.height(),
    );
    if *last_size == size {
        return;
    }
    *last_size = size;

    for entity in &existing_pieces {
        commands.entity(entity).despawn();
    }

    let min_length_pattern = size.x.min(size.y);
    let outter_radius =
        min_length_pattern * SCREEN_SIDE_LENGTH_RATIO / BOARD_LOGICAL_LEN as f32 / 2.0;
    let center = (BOARD_LOGICAL_LEN as f32 - 1.0) / 2.0;

    for i in 0..BOARD_LOGICAL_LEN {
        for j in 0..BOARD_LOGICAL_LEN {
            let coin_color = LIGHTER_GREY;
            draw_coin(
                &mut commands,
                &mut meshes,
                &mut materials,
                outter_radius,
                coin_color,
                LEN_RADIUS_RATIO * outter_radius,
                DARK_GREY,
                (
                    (i as f32 - center) * outter_radius * 2.0,
                    (j as f32 - center) * outter_radius * 2.0,
                    0.0,
                ),
                BackGroundPieceStrct,
            );
        }
    }
}

fn board_metrics(window: &Window) -> (f32, f32) {
    let min_side = window.resolution.width().min(window.resolution.height());
    let outter_radius = min_side * SCREEN_SIDE_LENGTH_RATIO / BOARD_LOGICAL_LEN as f32 / 2.0;
    let center = (BOARD_LOGICAL_LEN as f32 - 1.0) / 2.0;
    (outter_radius, center)
}

fn cursor_to_board(
    cursor: Vec2,
    window: &Window,
    outter_radius: f32,
    center: f32,
) -> Option<(f32, f32)> {
    let ori_x = 0.5 * window.resolution.width();
    let ori_y = 0.5 * window.resolution.height();

    let delta_x = cursor.x - ori_x;
    let delta_y = cursor.y - ori_y;

    let num_x = (delta_x / (outter_radius * 2.0) + center).round() - center;
    let num_y = (-delta_y / (outter_radius * 2.0) + center).round() - center;

    if num_x < -center || num_y < -center || num_x > center || num_y > center {
        None
    } else {
        Some((num_x, num_y))
    }
}

pub fn mouse_movement(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mouse_motion: MessageReader<CursorMoved>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    existing_pieces: Query<Entity, With<MouthFloatPieceStrct>>,
    click_num: Res<GameState>,
    mut hover: ResMut<HoverCell>,
) {
    let Ok(game_window) = window_query.single() else {
        return;
    };

    let Some(motion) = mouse_motion.read().last() else {
        return;
    };

    let (outter_radius, center) = board_metrics(game_window);
    let new_cell = cursor_to_board(motion.position, game_window, outter_radius, center);
    if hover.0 == new_cell {
        return;
    }

    hover.0 = new_cell;
    for entity in &existing_pieces {
        commands.entity(entity).try_despawn();
    }

    let Some((num_x, num_y)) = new_cell else {
        return;
    };

    let coin_color = if click_num.num % 2 > 0 {
        IMPERIAL_YELLOW
    } else {
        CINNABAR_RED
    };

    draw_coin(
        &mut commands,
        &mut meshes,
        &mut materials,
        outter_radius,
        coin_color,
        LEN_RADIUS_RATIO * outter_radius,
        DARK_GREY,
        (
            num_x * outter_radius * 2.0,
            num_y * outter_radius * 2.0,
            0.0,
        ),
        MouthFloatPieceStrct,
    );
}

pub fn mouse_click(
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut click_num: ResMut<GameState>,
) {
    if !mouse.just_released(MouseButton::Left) {
        return;
    }

    let Ok(game_window) = window_query.single() else {
        return;
    };
    let Some(cursor) = game_window.cursor_position() else {
        return;
    };

    let (outter_radius, center) = board_metrics(game_window);
    let Some((num_x, num_y)) = cursor_to_board(cursor, game_window, outter_radius, center) else {
        info!("Click ignored: outside board");
        return;
    };

    info!("Click at cell ({num_x}, {num_y})");
    click_num.num += 1;
}
