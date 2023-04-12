use gtest::{Log, Program, System};

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send_bytes(2, String::from("INIT MESSAGE"));
}