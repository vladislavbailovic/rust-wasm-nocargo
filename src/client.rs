#[cfg(target_arch = "wasm32")]
extern "C" {
    fn wasm_log(ptr: u32, len: usize);
}
#[cfg(not(target_arch = "wasm32"))]
fn wasm_log(ptr: u32, len: usize) {}

pub fn log(what: &str) {
    unsafe {
        wasm_log(what.as_ptr() as u32, what.len());
    }
}
