use gtest::{Log, Program, System};

#[test]
fn hello_test() {
    let sys = System::new();
    let program = Program::from_file(&sys, "./target/wasm32-unknown-unknown/release/hello_world.wasm");
    let program = Program::current(&sys);
}