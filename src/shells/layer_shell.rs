use bevy::prelude::{Component, Resource};
use raw_window_handle::{
    DisplayHandle, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    WaylandDisplayHandle, WaylandWindowHandle, WindowHandle,
};
use smithay_client_toolkit::{
    delegate_layer,
    globals::GlobalData,
    reexports::{
        client::{
            Connection, Dispatch, Proxy, QueueHandle, globals::GlobalList,
            protocol::wl_surface::WlSurface,
        },
        protocols_wlr::layer_shell::v1::client::{zwlr_layer_shell_v1, zwlr_layer_surface_v1},
    },
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShell, LayerShellHandler, LayerSurface, LayerSurfaceData},
    },
};

pub use smithay_client_toolkit::shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer};

use crate::state::SmithayRunnerState;

#[derive(Debug, Resource, Component, Clone)]
pub struct LayerShellSettings {
    pub anchor: Anchor,
    pub size: (u32, u32),
    pub exclusive_zone: i32,
    pub margin: (i32, i32, i32, i32),
    pub keyboard_interactivity: KeyboardInteractivity,
    pub layer: Layer,
}

impl Default for LayerShellSettings {
    fn default() -> Self {
        Self {
            anchor: Anchor::empty(),
            exclusive_zone: Default::default(),
            margin: Default::default(),
            size: (256, 256),
            keyboard_interactivity: Default::default(),
            layer: Layer::Top,
        }
    }
}

pub struct LayerShellWindow {
    window: LayerSurface,
    conn: Connection,
}

impl LayerShellWindow {
    pub fn layer_surface(&self) -> &LayerSurface {
        &self.window
    }

    pub fn layer_surface_mut(&mut self) -> &mut LayerSurface {
        &mut self.window
    }
}

impl HasWindowHandle for LayerShellWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            core::ptr::NonNull::new(self.window.wl_surface().id().as_ptr() as *mut _).unwrap(),
        ));
        unsafe { Ok(WindowHandle::borrow_raw(raw_window_handle)) }
    }
}

impl HasDisplayHandle for LayerShellWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle, raw_window_handle::HandleError> {
        let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            core::ptr::NonNull::new(self.conn.backend().display_ptr() as *mut _).unwrap(),
        ));
        unsafe { Ok(DisplayHandle::borrow_raw(raw_display_handle)) }
    }
}

pub fn create_window<State>(
    globals: &GlobalList,
    qh: &QueueHandle<State>,
    surface: WlSurface,
    conn: Connection,
    settings: &LayerShellSettings,
) -> LayerShellWindow
where
    State: LayerShellHandler
        + Dispatch<zwlr_layer_shell_v1::ZwlrLayerShellV1, GlobalData>
        + Dispatch<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1, LayerSurfaceData>
        + 'static,
{
    let layer_shell = LayerShell::bind(globals, qh).expect("layer shell not available");
    let layer =
        layer_shell.create_layer_surface(qh, surface, settings.layer, Some("simple_layer"), None);

    layer.set_anchor(settings.anchor);
    layer.set_keyboard_interactivity(settings.keyboard_interactivity);
    layer.set_size(settings.size.0, settings.size.1);
    layer.set_margin(
        settings.margin.0,
        settings.margin.1,
        settings.margin.2,
        settings.margin.3,
    );
    layer.set_exclusive_zone(settings.exclusive_zone);
    layer.commit();

    LayerShellWindow {
        window: layer,
        conn,
    }
}

impl LayerShellHandler for SmithayRunnerState {
    fn closed(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &QueueHandle<Self>,
        _: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
    ) {
    }

    fn configure(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &QueueHandle<Self>,
        _: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
        _: smithay_client_toolkit::shell::wlr_layer::LayerSurfaceConfigure,
        _: u32,
    ) {
    }
}

delegate_layer!(SmithayRunnerState);
