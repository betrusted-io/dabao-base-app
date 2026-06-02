mod cmds;
mod repl;
mod shell;
use cmds::*;
use usb_bao1x::UsbHid;

fn main() {
    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("my PID is {}", xous::process::id());
    let tt = ticktimer::Ticktimer::new().unwrap();

    let hal = bao1x_hal_service::Hal::new();
    // allow preemption in the dabao console environment
    hal.set_preemption(true);

    // spawn the shell thread
    shell::start_shell();

    tt.sleep_ms(500).ok(); // pause for the system to startup
    let usb = UsbHid::new();
    // connect the USB serial port to the local console
    usb.serial_console_input_injection();

    // This idiom creates a dummy server that blocks. This effectively parks the parent
    // process, allowing its child threads to run without taking any CPU resources.
    let dummy_sid = xous::create_server().unwrap();
    loop {
        // This call blocks forever since nobody has `dummy_sid` and thus it will never
        // receive a message.
        let _ = xous::receive_message(dummy_sid);
    }
}
