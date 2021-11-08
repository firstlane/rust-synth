use device_query::{DeviceEvents, DeviceState};

pub fn query_keys() {
    loop {
        let device_state = DeviceState::new();
        let _guard = device_state.on_key_down(|key| {

        });
        let _guard = device_state.on_key_up(|key| {

        });
    }
}

pub struct KeyboardEvent {
    key: device_query::Keycode,
    on: bool,
}
