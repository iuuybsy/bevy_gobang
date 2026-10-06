use bevy::{prelude::*, window::WindowResolution};

const CINNABAR_RED: (u8, u8, u8) = (140, 46, 37);
const IMPERIAL_YELLOW: (u8, u8, u8) = (225, 182, 55);
const DARK_GREY: (u8, u8, u8) = (10, 10, 10);

fn color_rgb(u8_rgb: (u8, u8, u8)) -> Color {
    let r_f32 = (u8_rgb.0 as f32) / 255.0;
    let g_f32 = (u8_rgb.1 as f32) / 255.0;
    let b_f32 = (u8_rgb.2 as f32) / 255.0;
    Color::srgb(r_f32, g_f32, b_f32)
}

fn draw_coin(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    circle_radius: f32,
    circle_color: (u8, u8, u8),
    square_len: f32,
    square_color: (u8, u8, u8),
    pos: (f32, f32, f32),
) {
    let circle = meshes.add(Circle::new(circle_radius));
    let color = color_rgb(circle_color);
    commands.spawn((
        Mesh2d(circle),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos.0, pos.1, pos.2),
    ));

    let square = meshes.add(Rectangle::new(square_len, square_len));
    let color = color_rgb(square_color);
    commands.spawn((
        Mesh2d(square),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos.0, pos.1, pos.2),
    ));
}

fn draw(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    draw_coin(
        &mut commands,
        &mut meshes,
        &mut materials,
        50.0,
        IMPERIAL_YELLOW,
        30.0,
        DARK_GREY,
        (-80.0, 0.0, 0.0),
    );
    draw_coin(
        &mut commands,
        &mut meshes,
        &mut materials,
        50.0,
        CINNABAR_RED,
        30.0,
        DARK_GREY,
        (80.0, 0.0, 0.0),
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_gobang".to_string(),
                resolution: WindowResolution::new(500, 500),
                resize_constraints: WindowResizeConstraints {
                    min_width: 300.0,
                    min_height: 300.0,
                    max_width: 3000.0,
                    max_height: 3000.0,
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(color_rgb(DARK_GREY)))
        .add_systems(Startup, draw)
        .run();
}
