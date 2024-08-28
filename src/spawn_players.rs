use bevy::prelude::*;

use crate::context_menu::{
    ContextMenuOption, ContextMenuOptions,
};

pub fn spawn_players(mut commands: Commands) {
    let _ = commands
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Px(100.),
                    left: Val::Px(500.),
                    width: Val::Px(50.),
                    height: Val::Px(50.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::srgb(
                    0.3, 0.8, 0.3,
                )
                .into(),
                ..Default::default()
            },
            ContextMenuOptions {
                value: vec![ContextMenuOption {
                    text: "Select".to_string(),
                    on_click: || println!("Select"),
                }],
            },
            Name::new("IronRock"), // Optional: for easier debugging
        ))
        .insert(Interaction::default())
        .id();
}
