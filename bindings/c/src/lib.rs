extern crate monto3;

#[no_mangle]
pub extern "C" fn monto_service(x: i32, y: i32) -> i32 {
    x + y
}
