use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

const CINNABAR_RED: (u8, u8, u8) = (140, 46, 37);
const IMPERIAL_YELLOW: (u8, u8, u8) = (225, 182, 55);
const DARK_GREY: (u8, u8, u8) = (10, 10, 10);
const LIGHTER_GREY: (u8, u8, u8) = (20, 20, 20);
const RGB_NORMALIZE: f32 = 255.0;

const SCREEN_SIDE_LENGTH_RATIO: f32 = 0.95;
const LEN_RADIUS_RATIO: f32 = 0.6;

const BOARD_LOGICAL_LEN: u8 = 15;

const INIT_WIDTH: u32 = 500;
const INIT_HEIGHT: u32 = 500;

const MIN_WIDTH: f32 = 300.0;
const MAX_WIDTH: f32 = 3000.0;
const MIN_HEIGHT: f32 = 300.0;
const MAX_HEIGHT: f32 = 3000.0;

#[derive(Component, Clone)]
struct BackGroundPieceStrct;

#[derive(Component, Clone)]
struct MouthFloatPieceStrct;

fn color_rgb(u8_rgb: (u8, u8, u8)) -> Color {
    let r_f32 = (u8_rgb.0 as f32) / RGB_NORMALIZE;
    let g_f32 = (u8_rgb.1 as f32) / RGB_NORMALIZE;
    let b_f32 = (u8_rgb.2 as f32) / RGB_NORMALIZE;
    Color::srgb(r_f32, g_f32, b_f32)
}

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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn draw(
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

fn mouse_movement(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mouse_motion: MessageReader<CursorMoved>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    existing_pieces: Query<Entity, With<MouthFloatPieceStrct>>,
) {
    let Ok(game_window) = window_query.single() else {
        return;
    };

    let screen_width = game_window.resolution.width();
    let screen_height = game_window.resolution.height();

    let ori_x = 0.5 * screen_width;
    let ori_y = 0.5 * screen_height;

    let min_length_pattern = screen_width.min(screen_height);
    let outter_radius =
        min_length_pattern * SCREEN_SIDE_LENGTH_RATIO / BOARD_LOGICAL_LEN as f32 / 2.0;
    let center = (BOARD_LOGICAL_LEN as f32 - 1.0) / 2.0;

    for motion in mouse_motion.read() {
        if !existing_pieces.is_empty() {
            for entity in &existing_pieces {
                commands.entity(entity).try_despawn();
            }
        }

        let x = motion.position.x;
        let y = motion.position.y;
        let delta_x = x - ori_x;
        let delta_y = y - ori_y;
        let num_x = (delta_x / (outter_radius * 2.0) + center).round() - center;
        let num_y = (-delta_y / (outter_radius * 2.0) + center).round() - center;
        if num_x < -center || num_y < -center || num_x > center || num_y > center {
            continue;
        }

        let mut coin_color = CINNABAR_RED;
        if ((num_x as i32).abs() + (num_y as i32).abs()) % 2 > 0 {
            coin_color = IMPERIAL_YELLOW;
        }

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
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_gobang".to_string(),
                resolution: WindowResolution::new(INIT_WIDTH, INIT_HEIGHT),
                resize_constraints: WindowResizeConstraints {
                    min_width: MIN_WIDTH,
                    min_height: MIN_HEIGHT,
                    max_width: MAX_WIDTH,
                    max_height: MAX_HEIGHT,
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(color_rgb(DARK_GREY)))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw, mouse_movement).chain())
        .run();
}
