extern crate esp32_sys;

use esp32_sys::{gpio_num_t, gpio_mode_t, esp_err_t, uart_port_t, uart_config_t};

//pub fn gpio_pad_select_gpio(gpio_num: u8) {
//    unsafe { esp32_sys::gpio_pad_select_gpio(gpio_num) }
//}

pub fn gpio_set_direction(gpio_num: gpio_num_t, mode: gpio_mode_t) -> esp_err_t {
    unsafe { esp32_sys::gpio_set_direction(gpio_num, mode) }
}

pub fn uart_param_config(uart_num: uart_port_t, uart_config: *const uart_config_t) -> esp_err_t {
    unsafe { esp32_sys::uart_param_config(uart_num, uart_config) }
}

pub fn uart_set_pin(
    uart_num: uart_port_t,
    tx_io_num: esp32_sys::std::os::raw::c_int,
    rx_io_num: esp32_sys::std::os::raw::c_int,
    rts_io_num: esp32_sys::std::os::raw::c_int,
    cts_io_num: esp32_sys::std::os::raw::c_int,
) -> esp_err_t {
    unsafe { esp32_sys::uart_set_pin(uart_num, tx_io_num, rx_io_num, rts_io_num, cts_io_num) }
}

pub fn uart_driver_install(
    uart_num: uart_port_t,
    rx_buffer_size: esp32_sys::std::os::raw::c_int,
    tx_buffer_size: esp32_sys::std::os::raw::c_int,
    queue_size: esp32_sys::std::os::raw::c_int,
    uart_queue: *mut esp32_sys::QueueHandle_t,
    intr_alloc_flags: esp32_sys::std::os::raw::c_int,
) -> esp_err_t {
    unsafe { esp32_sys::uart_driver_install(uart_num, rx_buffer_size, tx_buffer_size, queue_size, uart_queue, intr_alloc_flags) }
}

pub fn gpio_set_level(gpio_num: gpio_num_t, level: u32) -> esp_err_t {
    unsafe { esp32_sys::gpio_set_level(gpio_num, level) }
}

pub fn ets_delay_us(us: u32) {
    unsafe { esp32_sys::ets_delay_us(us) }
}

pub fn uart_write_bytes(
    uart_num: esp32_sys::uart_port_t,
    src: *const esp32_sys::std::os::raw::c_char,
    size: usize,
) -> esp32_sys::std::os::raw::c_int {
    unsafe { esp32_sys::uart_write_bytes(uart_num, src, size) }
}

macro_rules! re_export {
    () => {};

//    (
//        pub fn $i:ident($($arg:ident: $argty:ty)*) -> $ret:ty;
//        $($tail:tt)*
//    ) => {
//        extern {
//            pub fn $i($($arg: $argty),*) -> $ret;
//        }
//        re_export! { $($tail)* }
//    };

    (
        pub fn $i:ident($($arg:ident: $argty:ty)*);
        $($tail:tt)*
    ) => {
        pub fn $i($($arg: $argty),*) {
            unsafe { esp32_sys::$i($($arg),*) }
        }
    };
}

re_export! {
    pub fn gpio_pad_select_gpio(gpio_num: u8);
}
