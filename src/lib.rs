const FACTOR: usize = 5;
const WIDTH: usize = 16 * FACTOR;
const HEIGHT: usize = 9 * FACTOR;


static mut CANVAS: [u8; WIDTH*HEIGHT*4] = [0; WIDTH*HEIGHT*4];

#[no_mangle]
pub extern fn add(a: usize, b: usize) -> usize {
    return a+b;
}

#[no_mangle]
pub unsafe extern fn get_data() -> u32 {
    return CANVAS.as_ptr() as u32;
}

#[no_mangle]
pub extern fn get_len() -> usize {
    return WIDTH*HEIGHT*4;
}

#[no_mangle]
pub unsafe extern fn init() {
    for y in 0..HEIGHT {
        for x in (0..WIDTH).step_by(4) {
            CANVAS[y * WIDTH + x + 0] = 0xFF;
            CANVAS[y * WIDTH + x + 1] = 0x11;
            CANVAS[y * WIDTH + x + 2] = 0x22;
            CANVAS[y * WIDTH + x + 3] = 0x33;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn wat() {
        assert!(5 == add(2,3), "If this fails, we are in trouble");
    }
}
