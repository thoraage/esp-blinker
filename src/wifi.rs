extern crate esp32_sys;

use esp32_sys::*;
use crate::wrapper;

pub fn begin(ssid: &str, passphrase: &str) -> esp_err_t {
    get_mode()
}

fn get_mode() -> esp_err_t {
    let mut mode: &mut wifi_mode_t = &mut wifi_mode_t_WIFI_MODE_NULL;
    let response: esp_err_t = wrapper::esp_wifi_get_mode(mode);
    response
}
