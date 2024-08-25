use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, (add_item,));
    }
}

pub fn add_item() {
    println!("Add item")
}
