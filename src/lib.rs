#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
fn sleep_3_sec() {
    std::thread::sleep(std::time::Duration::from_secs(3));
}
