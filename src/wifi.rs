extern crate esp32_sys;

use esp32_sys::*;
use crate::wrapper;
use crate::output::Output;

pub fn begin(output: &Output, _ssid: &str, _passphrase: &str) {
    let mut wifi_context = WifiContext::new(output);
    wifi_context.enable_sta(true);
}

struct WifiContext<'a> {
    output: &'a Output,
    tcpip_initialized: bool,
    network_event_group: Option<EventGroupHandle_t>,
    network_event_queue: Option<QueueHandle_t>,
    network_event_task_handle: Option<TaskHandle_t>
}

impl<'a> WifiContext<'a> {
    pub fn new(output: &'a Output) -> WifiContext<'a> {
        WifiContext { output: &output, tcpip_initialized: false, network_event_group: None,
            network_event_queue: None, network_event_task_handle: None }
    }

    fn enable_sta(&mut self, enable: bool) -> bool {
        let current_mode = self.get_mode();
        let enabled = current_mode & wifi_mode_t_WIFI_MODE_STA != 0;
        if enabled != enable {
            let new_mode;
            if enable {
                new_mode = current_mode | wifi_mode_t_WIFI_MODE_STA;
            } else {
                new_mode = current_mode & !wifi_mode_t_WIFI_MODE_STA;
            }
            self.set_mode(new_mode);
        }
        true
    }

    fn get_mode(&self) -> wifi_mode_t {
        let mode: &mut wifi_mode_t = &mut wifi_mode_t_WIFI_MODE_NULL;
        if wrapper::esp_wifi_get_mode(mode) as u32 == ESP_ERR_WIFI_NOT_INIT {
            self.output.print("WiFi not started");
            wifi_mode_t_WIFI_MODE_NULL
        } else {
            *mode
        }
    }

    fn set_mode(&mut self, mode: wifi_mode_t) -> bool {
        let current_mode = self.get_mode();
        if current_mode == mode {
            return true;
        }
        if current_mode == 0 {
            if !self.start_wifi() {
                return false;
            }
        } else {
            self.output.print("set_mode(): UNIMPLEMENTED FEATURE");
            return false;
        }
        let response = wrapper::esp_wifi_set_mode(mode);
        if response as u32 != ESP_OK {
            print_error(self.output, "Could not set mode! ", response);
            return false;
        }
        true
    }

    fn start_wifi(&mut self) -> bool {
        if !self.init_wifi_low_level() {
            return false;
        }
        let response = wrapper::esp_wifi_start();
        if response as u32 != ESP_OK {
            print_error(self.output, "Could not set mode! ", response);
            return false;
        }
        true
    }

    fn init_wifi_low_level(&mut self) -> bool {
        // TODO global variable as barrier
        self.tcpip_init();
        // TODO ...
        false
    }

    fn tcpip_init(&mut self) {
        if !self.tcpip_initialized && self.start_network_event_task().is_ok() {
            self.tcpip_initialized = true;
            wrapper::tcpip_adapter_init();
        }
    }

    fn start_network_event_task(&mut self) -> Result<bool, &str> {
        if self.network_event_group.is_none() {
            let event_group = wrapper::xEventGroupCreate();
            if event_group.is_null() {
                return Result::Err("Network event group create failed")
            }
            wrapper::xEventGroupSetBits(event_group, BIT13 /*WIFI_DNS_IDLE_BIT*/);
            self.network_event_group = Option::Some(event_group);
        }
        if self.network_event_queue.is_none() {
            let event_queue = wrapper::xQueueGenericCreate(
                32, std::mem::size_of::<system_event_t>() as u32, 0 /*queueQUEUE_TYPE_BASE*/);
            if event_queue.is_null() {
                return Result::Err("Network event queue create failed")
            }
            self.network_event_queue = Option::Some(event_queue);
        }
        if self.network_event_task_handle.is_none() {
            let event_task_handle: TaskHandle_t = 0 as TaskHandle_t;
            wrapper::xTaskCreatePinnedToCore(Option::Some(network_event_task), "network_event", 4096, 0 as esp32_sys::std::os::raw::c_void, 2, &event_task_handle, 1 /*ARDUINO_RUNNING_CORE*/);
            if event_task_handle.is_null() {
                return Result::Err("Network Event Task Start Failed!")
            }
            self.network_event_task_handle = Option::Some(event_task_handle)
        }
        // TODO
        Result::Err("start_network_event_task: NOT IMPLEMENTED")
    }
}

#[no_mangle]
pub unsafe extern "C" fn network_event_task(c: *mut esp32_sys::std::os::raw::c_void) {

}

fn print_error(output: &Output, message: &str, esp_err: esp_err_t) {
    output.print(message);
    print_error_code(output, esp_err);
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

