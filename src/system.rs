use bevy::{
    ecs::{query::QueryFilter, system::SystemParamItem},
    prelude::*,
    window::{RawHandleWrapper, WindowCreated, WindowWrapper},
};
use smithay_client_toolkit::reexports::client::{
    Connection, QueueHandle, globals::GlobalList, protocol::wl_surface::WlSurface,
};

use crate::{
    CreateWindowParams, shells::layer_shell::LayerShellSettings, state::SmithayRunnerState,
};
pub(crate) fn create_windows<F: QueryFilter + 'static>(
    globals: &GlobalList,
    qh: &QueueHandle<SmithayRunnerState>,
    surface: &WlSurface,
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

        smithay_window = smithay_windows.create_window(
            entity,
            window_settings,
            globals,
            qh,
            conn.clone(),
            surface.clone(),
        );

        let mut wrapper: Option<_> = None;
        if let Ok(handle_wrapper) = RawHandleWrapper::new(smithay_window) {
            wrapper = Some(handle_wrapper.clone());
            if let Some(handle_holder) = handle_holder {
                *handle_holder.0.lock().unwrap() = Some(handle_wrapper);
            }
        }
        commands.entity(entity).insert(wrapper.unwrap());

        info!("Window created!");
        window_created_events.send(WindowCreated { window: entity });
    }
}

pub(crate) fn changed_windows(mut changed_windows: Query<(Entity, &mut Window), Changed<Window>>) {
    for (_entity, window) in &mut changed_windows {
        println!("Changed window!");
        println!("{:?}", window);
    }
}
