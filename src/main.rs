#![no_std]
#![no_main]

//#[macro_use] extern crate slice_as_array;

#[cfg(feature="use_std")]
#[macro_export]
#[doc(hidden)]
macro_rules! slice_as_array_transmute {
    ($slice:expr) => { ::std::mem::transmute($slice) }
}

#[cfg(not(feature="use_std"))]
#[macro_export]
macro_rules! slice_as_array_transmute {
    ($slice:expr) => { ::core::mem::transmute($slice) }
}
macro_rules! slice_as_array {
    ($slice:expr, [$t:ty ; $len:expr] ) => {{
        unsafe fn this_transmute(xs: &[$t]) -> &[$t; $len] {
            slice_as_array_transmute!(xs.as_ptr())
        }

        let s: &[$t] = $slice;
        if s.len() == $len {
            Some( unsafe { this_transmute(s) } )
        } else {
            None
        }
    }}
}
extern crate esp32_sys;

mod wrapper;
mod wifi;
mod output;

use core::panic::PanicInfo;
use esp32_sys::*;
use output::Output;
//use core::str::Chars;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const BLINK_GPIO: gpio_num_t = gpio_num_t_GPIO_NUM_5;
const UART_NUM: uart_port_t = uart_port_t_UART_NUM_1;
//const ECHO_TEST_TXD: i32 = gpio_num_t_GPIO_NUM_17 as i32;
//const ECHO_TEST_RXD: i32 = gpio_num_t_GPIO_NUM_16 as i32;
const ECHO_TEST_TXD: i32 = gpio_num_t_GPIO_NUM_1 as i32;
const ECHO_TEST_RXD: i32 = gpio_num_t_GPIO_NUM_3 as i32;
const ECHO_TEST_RTS: i32 = UART_PIN_NO_CHANGE;
const ECHO_TEST_CTS: i32 = UART_PIN_NO_CHANGE;

const BUF_SIZE: i32 = 1024;

#[no_mangle]
pub fn app_main() {
    rust_blink_and_write();
}

//fn start_wifi() {
//    print("Initiating wifi: ");
//    let wifi_init_config: wifi_init_config_t = wifi_init_config_t {
//        event_handler: None,
//        osi_funcs:
//    };
//    let esp_err = wrapper::esp_wifi_init(wifi_init_config);
//    print_error_code(esp_err);
//}

fn setup_wifi() {
//    output::print("Setting up wifi: ");
    let mut wifi_config: wifi_config_t = wifi_config_t {
        sta: wifi_sta_config_t {
            ssid: *slice_as_array!("Luftslottet".as_bytes(), [u8; 32]).expect("bad hash length"),
            password: *slice_as_array!("Uvelkommen(0".as_bytes(), [u8; 64]).expect("bad hash length"),
            scan_method: wifi_scan_method_t_WIFI_FAST_SCAN,
            bssid_set: false,
            bssid: [0, 0, 0, 0, 0, 0],
            channel: 0,
            listen_interval: 0,
            sort_method: wifi_sort_method_t_WIFI_CONNECT_AP_BY_SIGNAL,
            threshold: wifi_fast_scan_threshold_t {
                rssi: 0,
                authmode: wifi_auth_mode_t_WIFI_AUTH_OPEN
            }
        }
    };
    let esp_err = wrapper::esp_wifi_set_config(esp_interface_t_ESP_IF_WIFI_STA, &mut wifi_config);
//    let err_bytes = esp_err.to_be_bytes();
//    let error_chars = Chars { iter:  err_bytes.iter() };
//    print(error_chars.as_str());
//    fn aaaa(n: &u8) -> char { (n + b'0') as char };
//    let a: &str = std::str::from_utf8(err_bytes.iter().collect::<u8>()).expect("Doh!");
//    let chars: &dyn Iterator<Item=char> = &iter.map(aaaa).collect();
//    let a: Chars = chars;
//    let a: &str = &chars.as_str();
//    print_error_code(esp_err)
}

fn rust_blink_and_write() {
    wrapper::gpio_pad_select_gpio(BLINK_GPIO as u8);
    /* Set the GPIO as a push/pull output */
    wrapper::gpio_set_direction(BLINK_GPIO, gpio_mode_t_GPIO_MODE_OUTPUT);

    let output = Output::new(UART_NUM, ECHO_TEST_TXD, ECHO_TEST_RXD,
                             ECHO_TEST_RTS, ECHO_TEST_CTS, BUF_SIZE);

    wifi::begin(&output, "Luftslottet", "Uvelkommen(0");

    loop {
        /* Blink off (output low) */
        wrapper::gpio_set_level(BLINK_GPIO, 0);

        //vTaskDelay(1000 / portTICK_PERIOD_MS);
        wrapper::ets_delay_us(1_000_000);

        // Write data to UART.
        let test_str = "This is a test string.\n";
        output.print(test_str);

        /* Blink on (output high) */
        wrapper::gpio_set_level(BLINK_GPIO, 1);

        // vTaskDelay(1000 / portTICK_PERIOD_MS);
        wrapper::ets_delay_us(1_000_000);
    }
}
