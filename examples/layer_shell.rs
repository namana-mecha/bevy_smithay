#![allow(warnings)]
use bevy::{
    color::palettes::basic::*,
    prelude::*,
    window::{PrimaryWindow, WindowCreated},
    winit::WinitPlugin,
};
use bevy_simple_subsecond_system::prelude::*;
use bevy_smithay::{
    SmithayPlugin, SmithayWindowType,
    prelude::layer_shell::{Anchor, Layer, LayerShellSettings},
};

#[derive(Resource, Default)]
struct NewWindowInfo {
    entity: Option<Entity>,
    is_setup_pending: bool,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<WinitPlugin>(),
            SmithayPlugin {
                primary_window_type: SmithayWindowType::LayerShell {
                    settings: LayerShellSettings { ..default() },
                },
            },
        ))
        .init_resource::<NewWindowInfo>()
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, exit_on_esc, setup_new_window))
        .run();
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

#[hot]
fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut new_window_info: ResMut<NewWindowInfo>,
    mut text_query: Query<&mut Text>,
    mut primary_window_entity: Single<Entity, With<PrimaryWindow>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;
                if mouse_button.just_pressed(MouseButton::Left) {
                    if let Some(window_entity) = new_window_info.entity {
                        commands.entity(window_entity).despawn();
                        new_window_info.entity = None;
                        continue;
                    }
                    let new_window_entity = commands
                        .spawn((
                            Window {
                                title: "UI Only Window".to_string(),
                                resolution: (400., 250.).into(),
                                ..default()
                            },
                            SmithayWindowType::SubSurface {
                                parent: *primary_window_entity,
                                position: (250, 250),
                            },
                        ))
                        .id();
                    new_window_info.entity = Some(new_window_entity);
                    new_window_info.is_setup_pending = true;
                }
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup_new_window(
    mut commands: Commands,
    mut window_created_events: EventReader<WindowCreated>,
    mut new_window_info: ResMut<NewWindowInfo>,
    asset_server: Res<AssetServer>, // For fonts
) {
    for event in window_created_events.read() {
        if Some(event.window) == new_window_info.entity && new_window_info.is_setup_pending {
            info!(
                "New UI window created (ID: {:?}), setting up its camera and UI.",
                event.window
            );

            commands.spawn((
                Camera {
                    target: bevy::render::camera::RenderTarget::Window(
                        bevy::window::WindowRef::Entity(event.window),
                    ),
                    clear_color: ClearColorConfig::Custom(Color::WHITE),
                    ..default()
                },
                Camera2d,
            ));
            new_window_info.is_setup_pending = false; // Mark as setup complete
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Button"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}
