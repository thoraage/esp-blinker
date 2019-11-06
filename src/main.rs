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

use core::panic::PanicInfo;
use core::ptr;
use esp32_sys::*;
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

fn start_wifi() {
    print("Initiating wifi: ");
    let wifi_init_config: wifi_init_config_t = wifi_init_config_t {
        event_handler: None,
        osi_funcs:
    };
    let esp_err = wrapper::esp_wifi_init(wifi_init_config);
    print_error_code(esp_err);
}

fn setup_wifi() {
    print("Setting up wifi: ");
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
    print_error_code(esp_err)
}

fn print_error_code(esp_err: esp_err_t) {
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
    print(resp);
    print("\n")
}

fn print(test_str: &str) {
    wrapper::uart_write_bytes(UART_NUM, test_str.as_ptr() as *const _, test_str.len());
}

fn rust_blink_and_write() {
    wrapper::gpio_pad_select_gpio(BLINK_GPIO as u8);
    /* Set the GPIO as a push/pull output */
    wrapper::gpio_set_direction(BLINK_GPIO, gpio_mode_t_GPIO_MODE_OUTPUT);

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

    wrapper::uart_param_config(UART_NUM, &uart_config);
    wrapper::uart_set_pin(UART_NUM, ECHO_TEST_TXD, ECHO_TEST_RXD, ECHO_TEST_RTS, ECHO_TEST_CTS);
    wrapper::uart_driver_install(UART_NUM, BUF_SIZE * 2, 0, 0, ptr::null_mut(), 0);

    setup_wifi();

    loop {
        /* Blink off (output low) */
        wrapper::gpio_set_level(BLINK_GPIO, 0);

        //vTaskDelay(1000 / portTICK_PERIOD_MS);
        wrapper::ets_delay_us(1_000_000);

        // Write data to UART.
        let test_str = "This is a test string.\n";
        print(test_str);

        /* Blink on (output high) */
        wrapper::gpio_set_level(BLINK_GPIO, 1);

        // vTaskDelay(1000 / portTICK_PERIOD_MS);
        wrapper::ets_delay_us(1_000_000);
    }
}
