use bevy::prelude::*;

use raw_window_handle::{
    DisplayHandle, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    WaylandDisplayHandle, WaylandWindowHandle, WindowHandle,
};
use smithay_client_toolkit::{
    delegate_subcompositor,
    reexports::client::{
        Connection, Proxy, QueueHandle,
        globals::GlobalList,
        protocol::{wl_subsurface::WlSubsurface, wl_surface::WlSurface},
    },
};

pub use smithay_client_toolkit::shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer};

use crate::state::SmithayRunnerState;

pub struct SubsurfaceWindow {
    surface: Option<WlSurface>,
    subsurface: Option<WlSubsurface>,
    conn: Connection,
}

impl HasWindowHandle for SubsurfaceWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            core::ptr::NonNull::new(
                self.surface
                    .as_ref()
                    .expect("window handles doesn't exist because surface was destroyed")
                    .id()
                    .as_ptr() as *mut _,
            )
            .unwrap(),
        ));
        unsafe { Ok(WindowHandle::borrow_raw(raw_window_handle)) }
    }
}

impl HasDisplayHandle for SubsurfaceWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle, raw_window_handle::HandleError> {
        let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            core::ptr::NonNull::new(self.conn.backend().display_ptr() as *mut _).unwrap(),
        ));
        unsafe { Ok(DisplayHandle::borrow_raw(raw_display_handle)) }
    }
}

impl Drop for SubsurfaceWindow {
    fn drop(&mut self) {
        if let Some(subsurface) = self.subsurface.take() {
            subsurface.destroy();
        }
        if let Some(surface) = self.surface.take() {
            surface.destroy();
        }
    }
}

pub fn create_window<State>(
    _globals: &GlobalList,
    _qh: &QueueHandle<State>,
    surface: WlSurface,
    subsurface: WlSubsurface,
    conn: Connection,
) -> SubsurfaceWindow {
    SubsurfaceWindow {
        conn,
        surface: Some(surface),
        subsurface: Some(subsurface),
    }
}
delegate_subcompositor!(SmithayRunnerState);
