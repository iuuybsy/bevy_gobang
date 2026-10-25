mod consts;
mod game_plot;
mod resources;
mod utils;

use bevy::prelude::*;
use bevy::window::WindowResolution;

use consts::*;
use game_plot::systems::{
    draw_board, draw_piece, mouse_click, mouse_movement, setup, update_plot_info,
};
use resources::*;

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
        .insert_resource(ClearColor(utils::color_rgb(DARK_GREY, false)))
        .insert_resource(GameState::default())
        .insert_resource(PlotInfo::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_plot_info,
                draw_board,
                draw_piece,
                mouse_movement,
                mouse_click,
            )
                .chain(),
        )
        .run();
}
