#![no_std]

#[no_mangle]
pub extern fn avr_rust_test(x: i16) -> i16
{
    return x + 1;
}
