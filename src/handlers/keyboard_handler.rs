use crate::state::SmithayRunnerState;
use bevy::{
    input::keyboard::{Key, KeyboardInput},
    log::info,
    prelude::KeyCode,
    window::WindowEvent,
};
use smithay_client_toolkit::{
    delegate_keyboard,
    seat::keyboard::{KeyboardHandler, Keysym},
};

fn convert_keyboard_input(
    key_event: smithay_client_toolkit::seat::keyboard::KeyEvent,
) -> bevy::window::WindowEvent {
    bevy::window::WindowEvent::KeyboardInput(KeyboardInput {
        key_code: KeyCode::KeyA,
        logical_key: Key::Character("a".into()),
        state: bevy::input::ButtonState::Pressed,
        repeat: false,
        window: todo!(),
    })
}

impl KeyboardHandler for SmithayRunnerState {
    fn enter(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _: u32,
        _: &[u32],
        _: &[smithay_client_toolkit::seat::keyboard::Keysym],
    ) {
        info!("Keyboard enter!");
    }

    fn leave(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _: u32,
    ) {
        info!("Keyboard leave!");
    }

    fn press_key(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _: u32,
        _: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
        todo!()
    }

    fn release_key(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _: u32,
        _: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
        todo!()
    }

    fn update_modifiers(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _: u32,
        _: smithay_client_toolkit::seat::keyboard::Modifiers,
        _: u32,
    ) {
        todo!()
    }
}

delegate_keyboard!(SmithayRunnerState);
