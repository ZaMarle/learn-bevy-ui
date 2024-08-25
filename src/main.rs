mod camera;
mod context_menu;
mod inventory;
mod spawn_elements;

use bevy::{prelude::*, window::PrimaryWindow};
use camera::CameraPlugin;
use context_menu::ContextMenuPlugin;
use inventory::*;
use spawn_elements::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // DefaultPickingPlugins,
            CameraPlugin,
            InventoryPlugin,
            ContextMenuPlugin,
        ))
        .add_systems(Startup, spawn_iron_rock)
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
