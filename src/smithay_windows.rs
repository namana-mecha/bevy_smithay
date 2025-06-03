use bevy::{ecs::entity::EntityHashMap, prelude::*, window::WindowWrapper};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use smithay_client_toolkit::{
    compositor::CompositorState,
    reexports::client::{Connection, Proxy, QueueHandle, globals::GlobalList},
    shell::WaylandSurface,
    subcompositor::SubcompositorState,
};
use std::collections::*;
use wayland_backend::sys::client::ObjectId;

use crate::{
    SmithayWindowType,
    prelude::layer_shell::{self, LayerShellWindow},
    shells::subsurface::{self, SubsurfaceWindow},
    state::SmithayRunnerState,
};

pub enum SmithayWindow {
    LayerShellWindow(LayerShellWindow),
    SubSurface(SubsurfaceWindow),
}

impl SmithayWindow {
    pub fn get(&self) -> &SmithayWindow {
        self
    }
}

impl HasDisplayHandle for SmithayWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        match self {
            SmithayWindow::LayerShellWindow(layer_shell_window) => {
                layer_shell_window.display_handle()
            }
            SmithayWindow::SubSurface(subsurface_window) => subsurface_window.display_handle(),
        }
    }
}

impl HasWindowHandle for SmithayWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        match self {
            SmithayWindow::LayerShellWindow(layer_shell_window) => {
                layer_shell_window.window_handle()
            }
            SmithayWindow::SubSurface(subsurface_window) => subsurface_window.window_handle(),
        }
    }
}

#[derive(Default)]
pub struct SmithayWindows {
    pub windows: HashMap<ObjectId, WindowWrapper<SmithayWindow>>,
    pub entity_to_smithay: EntityHashMap<ObjectId>,
    pub smithay_to_entity: HashMap<ObjectId, Entity>,

    pub compositor: Option<CompositorState>,
    pub subcompositor: Option<SubcompositorState>,

    _not_send_sync: core::marker::PhantomData<*const ()>,
}

impl SmithayWindows {
    pub fn create_window(
        &mut self,
        entity: Entity,
        window_type: &SmithayWindowType,
        globals: &GlobalList,
        qh: &QueueHandle<SmithayRunnerState>,
        conn: Connection,
    ) -> &WindowWrapper<SmithayWindow> {
        if self.compositor.is_none() {
            let compositor = CompositorState::bind(globals, qh).expect("faild to bind compositor");
            let subcompositor =
                SubcompositorState::bind(compositor.wl_compositor().clone(), globals, qh)
                    .expect("failed to bind subcompositor");
            self.compositor = Some(compositor);
            self.subcompositor = Some(subcompositor);
        }

        match window_type {
            SmithayWindowType::LayerShell { settings } => {
                let surface = self
                    .compositor
                    .as_mut()
                    .expect("compositor not found")
                    .create_surface(qh);
                let window_id = surface.id();
                let smithay_window = SmithayWindow::LayerShellWindow(layer_shell::create_window(
                    globals, qh, surface, conn, settings,
                ));

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
            SmithayWindowType::SubSurface { parent, position } => {
                let parent_window_id = self
                    .entity_to_smithay
                    .get(parent)
                    .expect("invalid parent entity while creating subsurface");
                let parent_window = self
                    .windows
                    .get(parent_window_id)
                    .expect("no smithay window with specified parent")
                    .get();

                let parent_wl_surface = match parent_window {
                    SmithayWindow::LayerShellWindow(layer_shell_window) => {
                        layer_shell_window.layer_surface().wl_surface()
                    }
                    SmithayWindow::SubSurface(..) => {
                        panic!("you cannot create a subsurface of a subsurface")
                    }
                };

                let (wl_subsurface, wl_surface) = self
                    .subcompositor
                    .as_ref()
                    .expect("subcompositor wasn't initilized")
                    .create_subsurface(parent_wl_surface.clone(), qh);
                wl_subsurface.set_position(position.0, position.1);
                wl_subsurface.place_above(&wl_surface.clone());

                wl_surface.commit();
                parent_wl_surface.commit();

                let window_id = wl_surface.id();
                let smithay_window = SmithayWindow::SubSurface(subsurface::create_window(
                    globals,
                    qh,
                    wl_surface,
                    wl_subsurface,
                    conn,
                ));

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
    }
}
