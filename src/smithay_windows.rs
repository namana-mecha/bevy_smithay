use bevy::{ecs::entity::EntityHashMap, prelude::*, window::WindowWrapper};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use smithay_client_toolkit::reexports::client::{
    Connection, QueueHandle, globals::GlobalList, protocol::wl_surface::WlSurface,
};
use std::collections::*;

use crate::{
    prelude::{LayerShellSettings, LayerShellWindow, create_window},
    state::SmithayRunnerState,
};

pub struct SmithayWindow(LayerShellWindow);
impl HasDisplayHandle for SmithayWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.0.display_handle()
    }
}

impl HasWindowHandle for SmithayWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.0.window_handle()
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(usize);
impl WindowId {
    fn next(&mut self) -> Self {
        self.0 += 1;
        *self
    }
}

#[derive(Default)]
pub struct SmithayWindows {
    pub windows: HashMap<WindowId, WindowWrapper<SmithayWindow>>,
    pub entity_to_smithay: EntityHashMap<WindowId>,
    pub smithay_to_entity: HashMap<WindowId, Entity>,

    last_window_id: WindowId,
    _not_send_sync: core::marker::PhantomData<*const ()>,
}

impl SmithayWindows {
    pub fn create_window(
        &mut self,
        entity: Entity,
        settings: &LayerShellSettings,
        globals: &GlobalList,
        qh: &QueueHandle<SmithayRunnerState>,
        conn: Connection,
        surface: WlSurface,
    ) -> &WindowWrapper<SmithayWindow> {
        let smithay_window = SmithayWindow(create_window(globals, qh, surface, conn, settings));
        let window_id = self.last_window_id.next();
        self.entity_to_smithay.entry(entity).insert(window_id);
        self.smithay_to_entity.entry(window_id).insert_entry(entity);
        self.windows
            .entry(window_id)
            .insert_entry(WindowWrapper::new(smithay_window))
            .into_mut()
    }
}
