# bevy_gobang

Gobang (五子棋 / Gomoku) game built with [Bevy](https://bevyengine.org/).

## Features

- 15 × 15 board rendered as a grid of pieces.
- Two players take turns: red moves first, then yellow.
- Live hover preview shows the next piece under the cursor.
- Win detection for a run of five (or more) in a row; the winning line is highlighted.
- One second after a game ends, it restarts automatically.

During the one-second end-of-game pause:

- the mouse moves freely;
- no hover preview is drawn;
- clicks have no effect.

## Requirements

- Rust (edition 2024)
- [Bevy](https://bevyengine.org/) 0.19

## Run

```sh
cargo run
```

For a faster build, use a release profile:

```sh
cargo run --release
```

## Controls

- **Left mouse button** — place a piece on an empty intersection.
- **Right mouse button** — undo the last move.

## Project structure

- `src/main.rs` — app setup, window configuration, and system schedule.
- `src/consts.rs` — board size, colors, window size, and restart-timing constants.
- `src/resources.rs` — `GameState`, `PlotInfo`, and the `RestartTimer` resource.
- `src/game_logic/` — move handling, undo, win detection, and auto-restart.
- `src/game_plot/` — board and piece rendering, hover preview, and cursor-to-board mapping.
- `src/utils.rs` — color and coordinate helpers.
