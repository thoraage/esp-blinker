extern crate esp32_sys;

use crate::wrapper;
use esp32_sys::*;
use core::ptr;

impl Output {
    pub fn new(uart_port: uart_port_t, echo_test_txd: i32, echo_test_rxd: i32,
           echo_test_rts: i32, echo_test_cts: i32, buf_size: i32) -> Self {
        /* Configure parameters of an UART driver,
         * communication pins and install the driver */
        let uart_config = uart_config_t {
            baud_rate: 115200,
            data_bits: uart_word_length_t_UART_DATA_8_BITS,
            parity: uart_parity_t_UART_PARITY_DISABLE,
            stop_bits: uart_stop_bits_t_UART_STOP_BITS_1,
            flow_ctrl: uart_hw_flowcontrol_t_UART_HW_FLOWCTRL_DISABLE,
            rx_flow_ctrl_thresh: 0,
            use_ref_tick: false,
        };
        wrapper::uart_param_config(uart_port, &uart_config);
        wrapper::uart_set_pin(uart_port, echo_test_txd, echo_test_rxd, echo_test_rts, echo_test_cts);
        wrapper::uart_driver_install(uart_port, buf_size * 2, 0, 0, ptr::null_mut(), 0);
        Output { uart_port: uart_port }
    }

    pub fn print(&self, test_str: &str) {
        wrapper::uart_write_bytes(self.uart_port, test_str.as_ptr() as *const _, test_str.len());
    }
}

pub struct Output {
    uart_port: uart_port_t
}
