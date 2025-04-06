use bevy::{
    prelude::*,
    window::{RawHandleWrapperHolder, WindowCreated},
};
use shells::layer_shell::LayerShellSettings;
use smithay_windows::SmithayWindows;

mod handlers;
mod shells;
mod smithay_windows;
mod state;
mod system;

pub mod prelude {
    pub use super::SmithayPlugin;
    pub use super::shells::layer_shell::*;
}

pub struct SmithayPlugin;
impl Plugin for SmithayPlugin {
    fn name(&self) -> &str {
        "bevy_smithay::SmithayPlugin"
    }

    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<SmithayWindows>()
            .add_systems(Last, system::changed_windows);
        app.set_runner(state::smithay_runner);
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
            Option<&'static RawHandleWrapperHolder>,
        ),
        F,
    >,
    NonSendMut<'w, SmithayWindows>,
    Option<ResMut<'w, LayerShellSettings>>,
    EventWriter<'w, WindowCreated>,
);
