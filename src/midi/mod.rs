extern crate device_query;

use device_query::{DeviceEvents, DeviceQuery, DeviceState, MouseState, Keycode};

fn query_keys() {
    let device_state = DeviceEvents
    let guard = device_state.on_key_down(|key| {

    });
    let guard = device_state.on_key_up(|key| {

    });
}
