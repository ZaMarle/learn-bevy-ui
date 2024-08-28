mod camera;
mod context_menu;
mod inventory;
mod spawn_iron_rock;
mod spawn_players;

use bevy::prelude::*;
use camera::CameraPlugin;
use context_menu::ContextMenuPlugin;
use inventory::*;
use spawn_iron_rock::*;
use spawn_players::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // DefaultPickingPlugins,
            CameraPlugin,
            InventoryPlugin,
            ContextMenuPlugin,
        ))
        .add_systems(
            Startup,
            (spawn_iron_rock, spawn_players),
        )
        // .add_systems(
        //     Update,
        //     (
        //         handle_left_click,
        //         hover_system,
        //         despawn_menu_system,
        //     ),
        // )
        .run();
}
