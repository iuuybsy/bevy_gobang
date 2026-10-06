use bevy::prelude::*;

const CINNABAR_RED: (u8, u8, u8) = (227, 66, 52);
const IMPERIAL_YELLOW: (u8, u8, u8) = (245, 199, 26);
const DARK_BROWN: (u8, u8, u8) = (25, 13, 8);

// fn draw_citcle_impl(x_center: f32, y_center: f32, radius: f32) {

// }

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
        DARK_BROWN,
        (-80.0, 0.0, 0.0),
    );
    draw_coin(
        &mut commands,
        &mut meshes,
        &mut materials,
        50.0,
        CINNABAR_RED,
        30.0,
        DARK_BROWN,
        (80.0, 0.0, 0.0),
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, draw)
        .run();
}
