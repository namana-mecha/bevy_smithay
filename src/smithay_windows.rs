use bevy::{ecs::entity::EntityHashMap, prelude::*, window::WindowWrapper};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use smithay_client_toolkit::{
    compositor::CompositorState,
    reexports::client::{Connection, Proxy, QueueHandle, globals::GlobalList},
};
use std::collections::*;
use wayland_backend::sys::client::ObjectId;

use crate::{
    prelude::{LayerShellSettings, LayerShellWindow, create_window},
    state::SmithayRunnerState,
};

#[derive(Deref, DerefMut)]
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

#[derive(Default)]
pub struct SmithayWindows {
    pub windows: HashMap<ObjectId, WindowWrapper<SmithayWindow>>,
    pub entity_to_smithay: EntityHashMap<ObjectId>,
    pub smithay_to_entity: HashMap<ObjectId, Entity>,

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
    ) -> &WindowWrapper<SmithayWindow> {
        let compositor = CompositorState::bind(globals, qh).expect("faild to bind compositor");
        let surface = compositor.create_surface(qh);
        let window_id = surface.id();

        let smithay_window = SmithayWindow(create_window(globals, qh, surface, conn, settings));
        self.entity_to_smithay
            .entry(entity)
            .insert(window_id.clone());
        self.smithay_to_entity
            .entry(window_id.clone())
            .insert_entry(entity);
        self.windows
            .entry(window_id.clone())
            .insert_entry(WindowWrapper::new(smithay_window))
            .into_mut()
    }
}
