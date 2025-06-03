use bevy::{
    prelude::*,
    window::{PrimaryWindow, RawHandleWrapperHolder, WindowCreated},
};
use shells::layer_shell::LayerShellSettings;
use smithay_windows::SmithayWindows;

mod input;
mod shells;
mod smithay_windows;
mod state;
mod system;

pub mod prelude {
    pub use super::SmithayPlugin;
    pub use super::shells::*;
}

pub struct SmithayPlugin {
    pub primary_window_type: SmithayWindowType,
}

impl Default for SmithayPlugin {
    fn default() -> Self {
        Self {
            primary_window_type: SmithayWindowType::LayerShell {
                settings: LayerShellSettings { ..default() },
            },
        }
    }
}

impl Plugin for SmithayPlugin {
    fn name(&self) -> &str {
        "bevy_smithay::SmithayPlugin"
    }

    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<SmithayWindows>()
            .add_systems(Last, (system::changed_windows, system::despawn_windows));
        let query = app
            .world()
            .try_query_filtered::<Entity, (With<PrimaryWindow>, With<Window>)>();
        let mut query = query.expect("could not find primary window");
        let primary_window_entity = query
            .iter(app.world())
            .next()
            .expect("found multiple primary window");
        app.world_mut()
            .entity_mut(primary_window_entity)
            .insert(self.primary_window_type.clone());
        app.set_runner(state::smithay_runner);
    }
}

#[derive(Component, Clone)]
pub enum SmithayWindowType {
    LayerShell {
        settings: LayerShellSettings,
    },
    SubSurface {
        parent: Entity,
        position: (i32, i32),
    },
}

impl Default for SmithayWindowType {
    fn default() -> Self {
        Self::LayerShell {
            settings: default(),
        }
    }
}

trait AppSendEvent {
    fn send(&mut self, event: impl Into<bevy::window::WindowEvent>);
}

impl AppSendEvent for Vec<bevy::window::WindowEvent> {
    fn send(&mut self, event: impl Into<bevy::window::WindowEvent>) {
        self.push(Into::<bevy::window::WindowEvent>::into(event));
    }
}

pub type CreateWindowParams<'w, 's, F = ()> = (
    Commands<'w, 's>,
    Query<
        'w,
        's,
        (
            Entity,
            &'static mut Window,
            Option<&'static SmithayWindowType>,
            Option<&'static RawHandleWrapperHolder>,
        ),
        F,
    >,
    NonSendMut<'w, SmithayWindows>,
    EventWriter<'w, WindowCreated>,
);
