use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component, Clone)]
pub struct ContextMenu {
    pub options: ContextMenuOptions,
    pub spawn_time: f32,
}

#[derive(Component, Clone)]
pub struct ContextMenuOptions {
    pub value: Vec<ContextMenuOption>,
}

#[derive(Component, Clone, Debug)]
pub struct ContextMenuOption {
    pub text: String,
    pub on_click: fn(),
}

pub struct ContextMenuPlugin;

impl Plugin for ContextMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_left_click,
                hover_system,
                click_system,
                despawn_menu_system,
            ),
        );
    }
}

fn handle_left_click(
    commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    interactable_entities_query: Query<(
        &ContextMenuOptions,
        &Interaction,
    )>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        let window = windows.get_single().unwrap();
        let cursor_position = window.cursor_position();

        let mut options = vec![];

        for (context_menu_options, interaction) in
            interactable_entities_query.iter()
        {
            if *interaction == Interaction::Hovered {
                options.extend(
                    context_menu_options.value.clone(),
                )
            }
        }

        println!("{:?}", options);

        spawn_menu(
            commands,
            asset_server,
            cursor_position,
            time,
            options,
        );
    }
}

fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cursor_position: Option<Vec2>, // materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    contest_menu_options: Vec<ContextMenuOption>,
) {
    let roboto_font = asset_server
        .load("fonts\\Roboto\\Roboto-Black.ttf");

    let mut options = contest_menu_options.clone();
    // options.push("Cancel".to_string());
    options.push(ContextMenuOption {
        text: "Cancel".to_string(),
        on_click: || println!("Cancel"),
    });

    // Parent entity for the menu
    let context_menu = commands
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Px(
                        cursor_position.unwrap().y,
                    ),
                    left: Val::Px(
                        cursor_position.unwrap().x,
                    ),
                    width: Val::Auto,
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::srgb(
                    0.3, 0.3, 0.3,
                )
                .into(),
                ..Default::default()
            },
            ContextMenu {
                spawn_time: time.elapsed_seconds(),
                options: ContextMenuOptions {
                    value: options.clone(),
                },
            },
        ))
        .insert(Interaction::default())
        .id();

    // Menu options
    commands.entity(context_menu).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Select option",
                TextStyle {
                    font: roboto_font.clone(),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ),
            ..Default::default()
        });
        for option in options.iter() {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        option.text.clone(),
                        TextStyle {
                            font: roboto_font.clone(),
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                },
                Interaction::default(),
                option.clone(),
                PreviousInteraction(Interaction::None),
            ));
        }
    });
    // todo: menu. make sure the bounds are within the window bounds
}

fn hover_system(
    mut query: Query<
        (&Interaction, &mut Text),
        With<ContextMenuOption>,
    >,
) {
    for (&interaction, mut text) in query.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                text.sections[0].style.color =
                    Color::srgb(255., 0., 0.);
            }
            _ => {
                text.sections[0].style.color = Color::WHITE;
            }
        }
    }
}

#[derive(Component)]
struct PreviousInteraction(Interaction);

fn click_system(
    mut query: Query<(
        &Interaction,
        &mut ContextMenuOption,
        &mut PreviousInteraction,
    )>,
    context_menu_query: Query<Entity, With<ContextMenu>>,
    mut commands: Commands,
) {
    for (
        interaction,
        context_menu_option,
        mut previous_interaction,
    ) in query.iter_mut()
    {
        if *interaction == Interaction::Pressed
            && previous_interaction.0
                != Interaction::Pressed
        {
            (context_menu_option.on_click)();
            let context_menu =
                context_menu_query.get_single().unwrap();

            commands
                .entity(context_menu)
                .despawn_recursive();

            previous_interaction.0 = *interaction;
        }

        // Reset the previous interaction state when released
        previous_interaction.0 = *interaction;
    }
}

fn despawn_menu_system(
    mut commands: Commands,
    context_menu_query: Query<
        (Entity, &Interaction, &ContextMenu),
        With<ContextMenu>,
    >,
    time: Res<Time>,
) {
    for (entity, &interaction, context_menu) in
        context_menu_query.iter()
    {
        if interaction == Interaction::None
            && (time.elapsed_seconds()
                > context_menu.spawn_time + 0.05)
        {
            commands.entity(entity).despawn_recursive();
        }
    }
}
