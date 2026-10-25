use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use super::plot_utils::*;

use crate::consts::*;
use crate::resources::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn update_plot_info(
    mut plot_info: ResMut<PlotInfo>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(game_window) = window_query.single() else {
        return;
    };

    plot_info.window_size = Vec2::new(
        game_window.resolution.width(),
        game_window.resolution.height(),
    );
    (plot_info.outter_radius, plot_info.center) = board_metrics(game_window);
}

pub fn draw_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_pieces: Query<Entity, With<StaticPieceStrct>>,
    plot_info: Res<PlotInfo>,
) {
    for entity in &existing_pieces {
        commands.entity(entity).try_despawn();
    }

    for i in 0..BOARD_LOGICAL_LEN {
        for j in 0..BOARD_LOGICAL_LEN {
            draw_single_piece(
                &mut commands,
                &mut meshes,
                &mut materials,
                plot_info.outter_radius,
                LIGHTER_GREY,
                false,
                LEN_RADIUS_RATIO * plot_info.outter_radius,
                DARK_GREY,
                (
                    (i as f32 - plot_info.center) * plot_info.outter_radius * 2.0,
                    (j as f32 - plot_info.center) * plot_info.outter_radius * 2.0,
                    0.0,
                ),
                StaticPieceStrct,
            );
        }
    }
}

pub fn draw_pieces(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_pieces: Query<Entity, With<SetPieceStrct>>,
    game_state: Res<GameState>,
    plot_info: Res<PlotInfo>,
) {
    for entity in &existing_pieces {
        commands.entity(entity).despawn();
    }

    game_state.red_set.iter().for_each(|cord| {
        draw_single_piece(
            &mut commands,
            &mut meshes,
            &mut materials,
            plot_info.outter_radius,
            CINNABAR_RED,
            false,
            LEN_RADIUS_RATIO * plot_info.outter_radius,
            DARK_GREY,
            (
                (cord.0 as f32 - plot_info.center) * plot_info.outter_radius * 2.0,
                (cord.1 as f32 - plot_info.center) * plot_info.outter_radius * 2.0,
                2.0,
            ),
            SetPieceStrct,
        );
    });

    game_state.yellow_set.iter().for_each(|cord| {
        draw_single_piece(
            &mut commands,
            &mut meshes,
            &mut materials,
            plot_info.outter_radius,
            IMPERIAL_YELLOW,
            false,
            LEN_RADIUS_RATIO * plot_info.outter_radius,
            DARK_GREY,
            (
                (cord.0 as f32 - plot_info.center) * plot_info.outter_radius * 2.0,
                (cord.1 as f32 - plot_info.center) * plot_info.outter_radius * 2.0,
                2.0,
            ),
            SetPieceStrct,
        );
    });
}

pub fn mouse_movement(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut mouse_motion: MessageReader<CursorMoved>,
    existing_pieces: Query<Entity, With<MouthFloatPieceStrct>>,
    game_state: Res<GameState>,
    plot_info: Res<PlotInfo>,
) {
    let Some(motion) = mouse_motion.read().last() else {
        return;
    };

    let new_cell = cursor_to_board(motion.position, &plot_info);

    for entity in &existing_pieces {
        commands.entity(entity).try_despawn();
    }

    let Some((num_x, num_y)) = new_cell else {
        return;
    };

    let cord = (
        (num_x + plot_info.center) as u32,
        (num_y + plot_info.center) as u32,
    );
    if game_state.red_set.contains(&cord) || game_state.yellow_set.contains(&cord) {
        return;
    }

    let coin_color = if game_state.is_red_turn() {
        CINNABAR_RED
    } else {
        IMPERIAL_YELLOW
    };

    draw_single_piece(
        &mut commands,
        &mut meshes,
        &mut materials,
        plot_info.outter_radius,
        coin_color,
        true,
        LEN_RADIUS_RATIO * plot_info.outter_radius,
        DARK_GREY,
        (
            num_x * plot_info.outter_radius * 2.0,
            num_y * plot_info.outter_radius * 2.0,
            1.0,
        ),
        MouthFloatPieceStrct,
    );
}

pub fn mouse_click(
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_state: ResMut<GameState>,
    plot_info: Res<PlotInfo>,
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

    let Some((num_x, num_y)) = cursor_to_board(cursor, &plot_info) else {
        return;
    };

    let num_x = (num_x + plot_info.center) as u32;
    let num_y = (num_y + plot_info.center) as u32;
    game_state.take_a_move((num_x, num_y));
}
