extern crate esp32_sys;

use esp32_sys::*;
use crate::wrapper;
use crate::output::Output;

pub fn begin(output: &Output, ssid: &str, passphrase: &str) {
    get_mode(output);
}

fn get_mode(output: &Output) {
    let mut mode: &mut wifi_mode_t = &mut wifi_mode_t_WIFI_MODE_NULL;
    let response: esp_err_t = wrapper::esp_wifi_get_mode(mode);
    print_error_code(output, response);
}

fn print_error_code(output: &Output, esp_err: esp_err_t) {
    let resp = match esp_err as u32 {
        ESP_ERR_WIFI_NOT_INIT => "ESP_ERR_WIFI_NOT_INIT",
        ESP_ERR_WIFI_NOT_STARTED => "ESP_ERR_WIFI_NOT_STARTED",
        ESP_ERR_WIFI_NOT_STOPPED => "ESP_ERR_WIFI_NOT_STOPPED",
        ESP_ERR_WIFI_IF => "ESP_ERR_WIFI_IF",
        ESP_ERR_WIFI_MODE => "ESP_ERR_WIFI_MODE",
        ESP_ERR_WIFI_STATE => "ESP_ERR_WIFI_STATE",
        ESP_ERR_WIFI_CONN => "ESP_ERR_WIFI_CONN",
        ESP_ERR_WIFI_NVS => "ESP_ERR_WIFI_NVS",
        ESP_ERR_WIFI_MAC => "ESP_ERR_WIFI_MAC",
        ESP_ERR_WIFI_SSID => "ESP_ERR_WIFI_SSID",
        ESP_ERR_WIFI_PASSWORD => "ESP_ERR_WIFI_PASSWORD",
        ESP_ERR_WIFI_TIMEOUT => "ESP_ERR_WIFI_TIMEOUT",
        ESP_ERR_WIFI_WAKE_FAIL => "ESP_ERR_WIFI_WAKE_FAIL",
        ESP_ERR_WIFI_WOULD_BLOCK => "ESP_ERR_WIFI_WOULD_BLOCK",
        ESP_ERR_WIFI_NOT_CONNECT => "ESP_ERR_WIFI_NOT_CONNECT",
        _ => "Unknown"
    };
    output.print(resp);
    output.print("\n")
}

