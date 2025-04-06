use std::time::Duration;

use bevy::{app::PluginsState, ecs::system::SystemState, prelude::*};

use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_output, delegate_registry, delegate_seat,
    output::{OutputHandler, OutputState},
    reexports::{
        calloop::EventLoop,
        calloop_wayland_source::WaylandSource,
        client::{
            Connection,
            globals::registry_queue_init,
            protocol::{wl_keyboard, wl_pointer, wl_touch},
        },
    },
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::{Capability, SeatHandler, SeatState},
};

#[allow(unused_imports)]
use crate::handlers::prelude::*;
use crate::{CreateWindowParams, system::create_windows};

pub fn smithay_runner(mut app: App) -> AppExit {
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }

    let conn = Connection::connect_to_env().expect("failed to connect to wayland!");
    let mut event_loop =
        EventLoop::<SmithayRunnerState>::try_new().expect("failed to create event_loop");
    let (globals, event_queue) =
        registry_queue_init::<SmithayRunnerState>(&conn).expect("failed to init registry queue");

    let qh = event_queue.handle();
    let loop_handle = event_loop.handle();
    WaylandSource::new(conn.clone(), event_queue)
        .insert(loop_handle.clone())
        .expect("failed to insert wayland source to event loop");

    let compositor = CompositorState::bind(&globals, &qh).expect("faild to bind compositor");
    let surface = compositor.create_surface(&qh);

    let mut smithay_runner_state = SmithayRunnerState {
        registry_state: RegistryState::new(&globals),
        seat_state: SeatState::new(&globals, &qh),
        output_state: OutputState::new(&globals, &qh),

        keyboard: None,
        pointer: None,
        touch: None,

        app,
    };
    let mut create_window = SystemState::<CreateWindowParams<Added<Window>>>::from_world(
        smithay_runner_state.world_mut(),
    );
    create_windows(
        &globals,
        &qh,
        &surface,
        conn.clone(),
        create_window.get_mut(smithay_runner_state.world_mut()),
    );
    create_window.apply(smithay_runner_state.world_mut());

    loop {
        event_loop
            .dispatch(
                Duration::from_secs_f32(1.0 / 60.0),
                &mut smithay_runner_state,
            )
            .expect("an unexpected error occured");
        smithay_runner_state.run_app_update();
    }
}

pub struct SmithayRunnerState {
    // Wayland States
    registry_state: RegistryState,
    seat_state: SeatState,
    output_state: OutputState,

    // Inputs
    keyboard: Option<wl_keyboard::WlKeyboard>,
    pointer: Option<wl_pointer::WlPointer>,
    touch: Option<wl_touch::WlTouch>,

    // Bevy
    app: App,
}

impl SmithayRunnerState {
    pub fn world(&self) -> &World {
        self.app.world()
    }

    pub fn world_mut(&mut self) -> &mut World {
        self.app.world_mut()
    }

    pub fn run_app_update(&mut self) {
        if self.app.plugins_state() == PluginsState::Cleaned {
            self.app.update();
        }
    }
}

impl CompositorHandler for SmithayRunnerState {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _new_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _new_transform: smithay_client_toolkit::reexports::client::protocol::wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _time: u32,
    ) {
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _output: &smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _output: &smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }
}

impl OutputHandler for SmithayRunnerState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }
}

impl SeatHandler for SmithayRunnerState {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
    ) {
    }

    fn new_capability(
        &mut self,
        _: &Connection,
        qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        seat: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
        capability: smithay_client_toolkit::seat::Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_none() {
            let keyboard = self.seat_state.get_keyboard(qh, &seat, None).unwrap();
            self.keyboard = Some(keyboard);
        }
        if capability == Capability::Pointer && self.pointer.is_none() {
            // let pointer = self.seat_state.get_pointer(qh, &seat).unwrap();
            // self.pointer = Some(pointer);

            info!("Pointer Attached");
        }
        if capability == Capability::Touch && self.touch.is_none() {
            // let touch = self.seat_state.get_touch(qh, &seat).unwrap();
            // self.touch = Some(touch);

            info!("Touchscreen Attached");
        }
    }

    fn remove_capability(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
        capability: smithay_client_toolkit::seat::Capability,
    ) {
        if capability == Capability::Keyboard {
            if let Some(keyboard) = self.keyboard.take() {
                keyboard.release();
            }
        }
        if capability == Capability::Pointer {
            if let Some(pointer) = self.pointer.take() {
                pointer.release();
            }
        }
    }

    fn remove_seat(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
    ) {
    }
}

impl ProvidesRegistryState for SmithayRunnerState {
    fn registry(&mut self) -> &mut smithay_client_toolkit::registry::RegistryState {
        &mut self.registry_state
    }

    registry_handlers!(OutputState, SeatState);
}

delegate_compositor!(SmithayRunnerState);
delegate_output!(SmithayRunnerState);
delegate_seat!(SmithayRunnerState);
delegate_registry!(SmithayRunnerState);
