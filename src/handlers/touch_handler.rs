use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::log;
use bevy::math::Vec2;
use bevy::window::{Window, WindowEvent};
use smithay_client_toolkit::reexports::client::Proxy;
use smithay_client_toolkit::{
    delegate_touch,
    reexports::client::{
        Connection, QueueHandle,
        protocol::{wl_surface::WlSurface, wl_touch::WlTouch},
    },
    seat::touch::TouchHandler,
};

use crate::AppSendEvent;
use crate::smithay_windows::SmithayWindows;
use crate::state::SmithayRunnerState; // Needed for tracking active touches

impl TouchHandler for SmithayRunnerState {
    fn down(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _touch: &WlTouch,
        _serial: u32,
        _time: u32,
        surface: WlSurface,
        id: i32,
        position: (f64, f64),
    ) {
        let entity = {
            let smithay_windows = self.world().non_send_resource::<SmithayWindows>();
            let window_id = surface.id();

            // Find the Bevy entity associated with the Smithay surface
            if let Some(entity) = smithay_windows.smithay_to_entity.get(&window_id).copied() {
                entity
            } else {
                log::warn!("Touch down event on unknown surface: {:?}", window_id);
                return;
            }
        };

        let scale_factor = {
            // Get the window component to access scale factor
            if let Some(window) = self.world().get::<Window>(entity) {
                window.scale_factor()
            } else {
                log::warn!(
                    "Touch down event for entity {:?} without a Window component",
                    entity
                );
                return;
            }
        };

        let logical_position = Vec2::new(position.0 as f32, position.1 as f32) / scale_factor;

        // Store the active touch point's entity and logical position
        self.active_touches.insert(id, (entity, logical_position));

        // Create and send the Bevy touch event
        let bevy_event = TouchInput {
            phase: TouchPhase::Started,
            position: logical_position,
            // Force is not directly available in basic Wayland touch events
            force: None,
            id: id as u64, // Bevy uses u64 for touch IDs
            window: entity,
        };

        // Send the event (adapt this line based on how you send events)
        self.bevy_window_events
            .send(WindowEvent::TouchInput(bevy_event));
        // Or if you have a specific TouchInput writer:
        // self.touch_events.send(bevy_event);
    }

    fn up(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _touch: &WlTouch,
        _serial: u32,
        _time: u32,
        id: i32,
    ) {
        // Retrieve the entity and last known position for the touch ID, then remove it
        let touch_data = self.active_touches.remove(&id);

        if let Some((entity, last_position)) = touch_data {
            // Create and send the Bevy touch event
            let bevy_event = TouchInput {
                phase: TouchPhase::Ended,
                position: last_position, // Use the stored last position
                force: None,
                id: id as u64,
                window: entity,
            };
            // Send the event
            self.bevy_window_events
                .send(WindowEvent::TouchInput(bevy_event));
        } else {
            // This might happen if the 'down' event was missed or occurred on a different surface
            log::warn!(
                "Touch up event for unknown or already removed touch ID: {}",
                id
            );
        }
    }

    fn motion(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _touch: &WlTouch,
        _time: u32,
        id: i32,
        position: (f64, f64),
    ) {
        // Get the entity associated with this ongoing touch ID
        let entity = if let Some((entity, _)) = self.active_touches.get(&id).copied() {
            entity
        } else {
            log::warn!("Touch motion event for unknown touch ID: {}", id);
            return;
        };

        let scale_factor = {
            // Get the window component to access scale factor
            if let Some(window) = self.world().get::<Window>(entity) {
                window.scale_factor()
            } else {
                log::warn!(
                    "Touch motion event for entity {:?} without a Window component",
                    entity
                );
                // We could remove the touch id here, but it might be valid if the window is recreated fast
                return;
            }
        };

        let logical_position = Vec2::new(position.0 as f32, position.1 as f32) / scale_factor;

        // Update the stored position for the touch ID
        if let Some(touch_data) = self.active_touches.get_mut(&id) {
            touch_data.1 = logical_position;
        } else {
            // Should technically not happen if the first check passed, but belts and braces
            return;
        }

        // Create and send the Bevy touch event
        let bevy_event = TouchInput {
            phase: TouchPhase::Moved,
            position: logical_position,
            force: None,
            id: id as u64,
            window: entity,
        };

        // Send the event
        self.bevy_window_events
            .send(WindowEvent::TouchInput(bevy_event));
    }

    fn cancel(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _touch: &WlTouch) {
        // // Retrieve the entity and last known position for the touch ID, then remove it
        // let touch_data = self.active_touches.remove(&id);
        //
        // if let Some((entity, last_position)) = touch_data {
        //     // Create and send the Bevy touch event
        //     let bevy_event = TouchInput {
        //         phase: TouchPhase::Canceled,
        //         position: last_position, // Use the stored last position
        //         force: None,
        //         id: id as u64,
        //         window: entity,
        //     };
        //     // Send the event
        //     self.bevy_window_events
        //         .send(WindowEvent::TouchInput(bevy_event));
        // } else {
        //     // eprintln!("Touch cancel event for unknown or already removed touch ID: {}", id);
        //     return;
        // }
    }

    // Optional methods for Wayland protocols >= 6 (wl_touch)
    // Implement these if you need shape or orientation data and have the corresponding
    // Bevy event types or state to update.

    fn shape(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _touch: &WlTouch,
        _id: i32,
        _major: f64,
        _minor: f64,
    ) {
        // Handle touch shape change if needed
    }

    fn orientation(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _touch: &WlTouch,
        _id: i32,
        _orientation: f64,
    ) {
        // Handle touch orientation change if needed
    }
}

// Delegate the touch handling implementation
delegate_touch!(SmithayRunnerState);
