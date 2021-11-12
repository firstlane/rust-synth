use device_query::{DeviceEvents, DeviceState};

pub fn query_keys() {
    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(|key| {

    });
    let _guard = device_state.on_key_up(|key| {

    });
}

pub struct KeyboardEvent {
    pub key: device_query::Keycode,
    pub on: bool,
}
