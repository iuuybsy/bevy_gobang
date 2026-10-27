use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::logic_utils::*;

use crate::consts::*;
use crate::resources::*;
use crate::utils::*;

pub fn mouse_click(
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_state: ResMut<GameState>,
    plot_info: Res<PlotInfo>,
) {
    if game_state.is_game_finish {
        return;
    }

    if mouse.just_released(MouseButton::Left) {
        let Ok(game_window) = window_query.single() else {
            return;
        };
        let Some(cursor) = game_window.cursor_position() else {
            return;
        };

        let Some((num_x, num_y)) = cursor_to_board(cursor, &plot_info) else {
            return;
        };

        let num_x = (num_x + plot_info.center) as u8;
        let num_y = (num_y + plot_info.center) as u8;
        game_state.take_a_move((num_x, num_y));
    } else if mouse.just_released(MouseButton::Right) {
        game_state.regret();
    }
}

pub fn auto_restart(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut restart_timer: ResMut<RestartTimer>,
) {
    if !game_state.is_game_finish {
        return;
    }

    restart_timer.timer.tick(time.delta());
    if restart_timer.timer.is_finished() {
        game_state.reset();
        restart_timer.timer.reset();
    }
}

pub fn check_game_status(mut game_state: ResMut<GameState>) {
    game_state.last_five_set.clear();
    if game_state.get_num() < 9 {
        return;
    }

    let Some(last_move) = game_state.get_last_move() else {
        return;
    };

    let (lx, ly) = last_move;
    let len = BOARD_LOGICAL_LEN as i32;

    let directions: [(i32, i32); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];

    for (dx, dy) in directions {
        let mut count: u8 = 1;

        let mut x = lx as i32 + dx;
        let mut y = ly as i32 + dy;
        while x >= 0
            && x < len
            && y >= 0
            && y < len
            && is_same_color(&game_state, lx, ly, x as u8, y as u8)
        {
            count += 1;
            x += dx;
            y += dy;
        }

        let mut x = lx as i32 - dx;
        let mut y = ly as i32 - dy;
        while x >= 0
            && x < len
            && y >= 0
            && y < len
            && is_same_color(&game_state, lx, ly, x as u8, y as u8)
        {
            count += 1;
            x -= dx;
            y -= dy;
        }

        if count >= 5 {
            game_state.is_game_finish = true;
            game_state.last_five_set.insert((lx, ly));
            let mut x = lx as i32 + dx;
            let mut y = ly as i32 + dy;
            while x >= 0
                && x < len
                && y >= 0
                && y < len
                && is_same_color(&game_state, lx, ly, x as u8, y as u8)
            {
                game_state.last_five_set.insert((x as u8, y as u8));
                x += dx;
                y += dy;
            }
            let mut x = lx as i32 - dx;
            let mut y = ly as i32 - dy;
            while x >= 0
                && x < len
                && y >= 0
                && y < len
                && is_same_color(&game_state, lx, ly, x as u8, y as u8)
            {
                game_state.last_five_set.insert((x as u8, y as u8));
                x -= dx;
                y -= dy;
            }
            break;
        }
    }
}
