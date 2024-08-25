use bevy::prelude::*;

use crate::context_menu::{
    ContextMenuOption, ContextMenuOptions,
};

pub fn spawn_iron_rock(mut commands: Commands) {
    let _ = commands
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Px(10.),
                    left: Val::Px(200.),
                    width: Val::Px(50.),
                    height: Val::Px(50.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::srgb(
                    0.5, 0.3, 0.3,
                )
                .into(),
                ..Default::default()
            },
            ContextMenuOptions {
                value: vec![
                    ContextMenuOption {
                        text: "Mine".to_string(),
                        on_click: || println!("Click mine"),
                    },
                    ContextMenuOption {
                        text: "Examine".to_string(),
                        on_click: || {
                            println!("Click examine")
                        },
                    },
                ],
            },
            Name::new("IronRock"), // Optional: for easier debugging
        ))
        .insert(Interaction::default())
        .id();
}
