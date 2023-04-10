#![no_std] 
use gstd::debug;
use::gstd::{msg, prelude::*};

#[no_mangle]
extern "C" fn handle() {
    msg::reply(String::from("Hello"), 0).expect("Error in sending a reply message");
}

#[no_mangle]
extern "C" fn init() {
    let init_message = String::from_utf8(msg::load_bytes().expect("Can't load init message")).expect("Can't decode to String");
    debug!("Program was initialized with message {:?}", init_message);
}

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send_bytes(2, String::from("INIT MESSAGE"));
}