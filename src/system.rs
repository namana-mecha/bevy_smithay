use bevy::{
    ecs::{query::QueryFilter, system::SystemParamItem},
    prelude::*,
    window::{PrimaryWindow, RawHandleWrapper, WindowCreated},
};
use smithay_client_toolkit::{
    reexports::client::{Connection, QueueHandle, globals::GlobalList},
    shell::WaylandSurface,
};

use crate::{
    CreateWindowParams, shells::layer_shell::LayerShellSettings, smithay_windows::SmithayWindows,
    state::SmithayRunnerState,
};
pub(crate) fn create_windows<F: QueryFilter + 'static>(
    globals: &GlobalList,
    qh: &QueueHandle<SmithayRunnerState>,
    conn: Connection,
    (
        mut commands,
        mut created_windows,
        mut smithay_windows,
        mut window_settings,
        mut window_created_events,
    ): SystemParamItem<CreateWindowParams<F>>,
) {
    let mut smithay_window;
    for (entity, window, handle_holder) in &mut created_windows {
        let window_settings = if let Some(window_settings) = window_settings.as_mut() {
            window_settings.size = (window.physical_width(), window.physical_height());
            window_settings
        } else {
            &LayerShellSettings {
                size: (window.physical_size().x, window.physical_size().y),
                ..Default::default()
            }
        };

        commands.entity(entity).insert_if_new(PrimaryWindow);
        smithay_window =
            smithay_windows.create_window(entity, window_settings, globals, qh, conn.clone());

        let mut wrapper: Option<_> = None;
        if let Ok(handle_wrapper) = RawHandleWrapper::new(smithay_window) {
            wrapper = Some(handle_wrapper.clone());
            if let Some(handle_holder) = handle_holder {
                *handle_holder.0.lock().unwrap() = Some(handle_wrapper);
            }
        }
        commands
            .entity(entity)
            .insert(wrapper.unwrap())
            .insert(window_settings.clone());

        info!("Window created!");
        window_created_events.send(WindowCreated { window: entity });
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn changed_windows(
    mut smithay_windows: NonSendMut<SmithayWindows>,
    mut changed_windows: Query<
        (Entity, &mut Window, &mut LayerShellSettings),
        Or<(Changed<Window>, Changed<LayerShellSettings>)>,
    >,
) {
    for (entity, _, layer_shell_settings) in &mut changed_windows {
        let window_id = smithay_windows
            .entity_to_smithay
            .get(&entity)
            .unwrap()
            .clone();
        let window = smithay_windows.windows.get_mut(&window_id).unwrap();
        let surface = window.layer_surface();
        surface.set_exclusive_zone(layer_shell_settings.exclusive_zone);
        surface.set_size(layer_shell_settings.size.0, layer_shell_settings.size.1);
        surface.set_margin(
            layer_shell_settings.margin.0,
            layer_shell_settings.margin.1,
            layer_shell_settings.margin.2,
            layer_shell_settings.margin.3,
        );
        surface.commit();
    }
}
