use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TypedSeed>();
        app.init_resource::<ViewDistance>();

        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, (
            update_seed_text, 
            handle_seed_typing, 
            handle_save_load_buttons,
            toggle_menu_visibility,
            handle_view_distance_buttons,
            update_view_distance_text,
            update_selected_block_text));
    }
}
#[derive(Resource)]
pub struct ViewDistance(pub i32);

impl Default for ViewDistance {
    fn default() -> Self {
        Self(1) 
    }
}

#[derive(Component)]
pub struct IncreaseViewDistButton;

#[derive(Component)]
pub struct DecreaseViewDistButton;

#[derive(Component)]
pub struct ViewDistTextMarker;
#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct SeedTextMarker;

#[derive(Component)]
pub struct TypingTextMarker;

#[derive(Component)]
pub struct SaveButton;

#[derive(Component)]
pub struct LoadButton;

#[derive(Resource, Default)]
pub struct TypedSeed(pub String);

#[derive(Component)]
pub struct SelectedBlockTextMarker;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(50.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            BackgroundColor(Color::srgb(0.09, 0.09, 0.09)), 
            BorderRadius::all(Val::Px(10.0)),
            Visibility::Hidden,
            HudRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            )).with_children(|enfant| {
                enfant.spawn((
                    Text::new("Menu (Echap pour fermer)"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
            //ViewDistance
            parent.spawn((
                Node {
                    width: Val::Percent(90.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                )).with_children(|row| {
                // Bouton -
                row.spawn((
                    Button,
                    Node { padding: UiRect::all(Val::Px(10.0)), ..default() },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                    BorderRadius::all(Val::Px(5.0)),
                    DecreaseViewDistButton,
                )).with_children(|btn| {
                    btn.spawn((Text::new("-"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE)));
                });

                // Texte Distance
                row.spawn((
                    Text::new("Distance de vue: 1"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::WHITE),
                    ViewDistTextMarker,
                ));
                // Bouton +
                row.spawn((
                    Button,
                    Node { padding: UiRect::all(Val::Px(10.0)), ..default() },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                    BorderRadius::all(Val::Px(5.0)),
                    IncreaseViewDistButton,
                )).with_children(|btn| {
                    btn.spawn((Text::new("+"), TextFont { font_size: 20.0, ..default() }, TextColor(Color::WHITE)));
                });
            });

            parent.spawn((
                Text::new("Seed: Chargement..."),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
                SeedTextMarker, 
            ));

            parent.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            )).with_children(|input_box| {
                input_box.spawn((
                    Text::new("Taper Seed puis la touche Entrer"),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 0.0)),
                    TypingTextMarker,
                ));
            });

            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
            )).with_children(|row| {
                row.spawn((
                    Button,
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.5, 0.1)),
                    BorderRadius::all(Val::Px(5.0)),
                    SaveButton,
                )).with_children(|btn| {
                    btn.spawn((Text::new("Sauvegarder"), TextFont { font_size: 18.0, ..default() }, TextColor(Color::WHITE)));
                });

                row.spawn((
                    Button,
                    Node {
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.1, 0.1)),
                    BorderRadius::all(Val::Px(5.0)),
                    LoadButton,
                )).with_children(|btn| {
                    btn.spawn((Text::new("Charger JSON"), TextFont { font_size: 18.0, ..default() }, TextColor(Color::WHITE)));
                });
            });
        });

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Percent(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        BorderRadius::all(Val::Px(8.0)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("[B] Bloc : Terre"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(1.0, 0.85, 0.3)),
            SelectedBlockTextMarker,
        ));
    });

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            left: Val::Percent(50.0),
            width: Val::Px(4.0),
            height: Val::Px(4.0),
            ..default()
        },
        BackgroundColor(Color::WHITE),
        BorderRadius::all(Val::Px(2.0)),
    ));
}

fn toggle_menu_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<HudRoot>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if let Ok(mut visibility) = query.single_mut() {
            *visibility = match *visibility {
                Visibility::Hidden => Visibility::Inherited,
                _ => Visibility::Hidden,
            };
        }
    }
}

fn handle_view_distance_buttons(
    mut interaction_query: Query<(&Interaction, Option<&IncreaseViewDistButton>, Option<&DecreaseViewDistButton>), (Changed<Interaction>, With<Button>)>,
    mut view_dist: ResMut<ViewDistance>,
) {
    for (interaction, inc, dec) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if inc.is_some() {
                view_dist.0 += 1;
                if view_dist.0 > 8 { view_dist.0 = 8; } 
            }
            if dec.is_some() {
                view_dist.0 -= 1;
                if view_dist.0 < 1 { view_dist.0 = 1; } 
            }
        }
    }
}

fn update_view_distance_text(
    mut query: Query<&mut Text, With<ViewDistTextMarker>>,
    view_dist: Res<ViewDistance>,
) {
    if view_dist.is_changed() || view_dist.is_added() {
        for mut text in &mut query {
            text.0 = format!("Distance de vue : {}", view_dist.0);
        }
    }
}

fn update_seed_text(
    mut query: Query<&mut Text, With<SeedTextMarker>>,
    seed_res: Res<crate::rendering::setup::CurrentSeed>,
) {
    if seed_res.is_changed() || seed_res.is_added() {
        for mut text in &mut query {
            text.0 = format!("Seed actuelle : {}", seed_res.0);
        }
    }
}

fn handle_seed_typing(
    mut key_events: MessageReader<KeyboardInput>,
    mut typed_seed: ResMut<TypedSeed>,
    mut text_query: Query<&mut Text, With<TypingTextMarker>>,
    mut ev_writer: MessageWriter<crate::rendering::setup::GenerateSeedEvent>,
    hud_query: Query<&Visibility, With<HudRoot>>,
) {
    if let Ok(vis) = hud_query.single() {
        if *vis == Visibility::Hidden { return; }
    }
    for event in key_events.read() {
        if event.state == ButtonState::Pressed {
            match &event.logical_key {
                Key::Character(c) => {
                    if c.chars().all(|ch| ch.is_ascii_digit()) {
                        typed_seed.0.push_str(c);
                    }
                }
                Key::Backspace => {
                    typed_seed.0.pop();
                }
                Key::Enter => {
                    if let Ok(seed_val) = typed_seed.0.parse::<u32>() {
                        ev_writer.write(crate::rendering::setup::GenerateSeedEvent(seed_val));
                    }
                    typed_seed.0.clear();
                }
                _ => {}
            }
        }
    }

    if typed_seed.is_changed() {
        for mut text in &mut text_query {
            if typed_seed.0.is_empty() {
                text.0 = "Taper Seed puis la touche Entrer".to_string();
            } else {
                text.0 = format!("> {} <", typed_seed.0);
            }
        }
    }
}

fn handle_save_load_buttons(
    mut interaction_query: Query<(&Interaction, Option<&SaveButton>, Option<&LoadButton>), (Changed<Interaction>, With<Button>)>,
    mut save_writer: MessageWriter<crate::rendering::setup::SaveMapEvent>,
    mut load_writer: MessageWriter<crate::rendering::setup::LoadMapEvent>,
) {
    for (interaction, save, load) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if save.is_some() {
                save_writer.write(crate::rendering::setup::SaveMapEvent);
            }
            if load.is_some() {
                load_writer.write(crate::rendering::setup::LoadMapEvent);
            }
        }
    }
}

fn update_selected_block_text(
    mut query: Query<&mut Text, With<SelectedBlockTextMarker>>,
    selected: Res<crate::rendering::setup::SelectedBlock>,
) {
    if selected.is_changed() || selected.is_added() {
        for mut text in &mut query {
            text.0 = format!("[B] Bloc : {}", selected.name());
        }
    }
}
