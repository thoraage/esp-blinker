extern crate esp32_sys;

use esp32_sys::{gpio_num_t, gpio_mode_t, esp_err_t, uart_port_t, uart_config_t, wifi_interface_t, wifi_config_t, wifi_mode_t};

macro_rules! re_export {
//    () => {};

    (
        pub fn $i:ident($($arg:ident: $argty:ty),*) -> $ret:ty;
//        $($tail:tt)*
    ) => {
        pub fn $i($($arg: $argty),*) -> $ret {
            unsafe { esp32_sys::$i($($arg),*) }
        }
    };

    (
        pub fn $i:ident($($arg:ident: $argty:ty),*);
//        $($tail:tt)*
    ) => {
        pub fn $i($($arg: $argty),*) {
            unsafe { esp32_sys::$i($($arg),*) }
        }
    };
}

re_export! {
//    pub fn esp_wifi_init(config: *const wifi_init_config_t) -> esp_err_t;
    pub fn esp_wifi_get_mode(mode: *mut wifi_mode_t) -> esp_err_t;
}

re_export! {
    pub fn esp_wifi_set_config(interface: wifi_interface_t, conf: *mut wifi_config_t) -> esp_err_t;
}

re_export! {
    pub fn gpio_pad_select_gpio(gpio_num: u8);
}

re_export! {
    pub fn gpio_set_direction(gpio_num: gpio_num_t, mode: gpio_mode_t) -> esp_err_t;
}

re_export! {
    pub fn uart_param_config(uart_num: uart_port_t, uart_config: *const uart_config_t) -> esp_err_t;
}

re_export! {
    pub fn uart_set_pin(
        uart_num: uart_port_t,
        tx_io_num: esp32_sys::std::os::raw::c_int,
        rx_io_num: esp32_sys::std::os::raw::c_int,
        rts_io_num: esp32_sys::std::os::raw::c_int,
        cts_io_num: esp32_sys::std::os::raw::c_int
    ) -> esp_err_t;
}

re_export! {
    pub fn uart_driver_install(
        uart_num: uart_port_t,
        rx_buffer_size: esp32_sys::std::os::raw::c_int,
        tx_buffer_size: esp32_sys::std::os::raw::c_int,
        queue_size: esp32_sys::std::os::raw::c_int,
        uart_queue: *mut esp32_sys::QueueHandle_t,
        intr_alloc_flags: esp32_sys::std::os::raw::c_int
    ) -> esp_err_t;
}

re_export! {
    pub fn gpio_set_level(gpio_num: gpio_num_t, level: u32) -> esp_err_t;
}

re_export! {
    pub fn ets_delay_us(us: u32);
}

re_export! {
    pub fn uart_write_bytes(
        uart_num: esp32_sys::uart_port_t,
        src: *const esp32_sys::std::os::raw::c_char,
        size: usize
    ) -> esp32_sys::std::os::raw::c_int;
}

