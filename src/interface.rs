use canvas::{get_default_canvas, Color};

#[no_mangle]
pub extern "C" fn add(a: usize, b: usize) -> usize {
    return a + b;
}

#[no_mangle]
pub unsafe extern "C" fn get_data() -> u32 {
    return get_default_canvas().get_bytes().as_ptr() as u32;
}

#[no_mangle]
pub unsafe extern "C" fn get_data_len() -> usize {
    return get_default_canvas().get_bytes().len();
}

#[no_mangle]
pub unsafe extern "C" fn get_width() -> usize {
    get_default_canvas().get_width()
}

#[no_mangle]
pub unsafe extern "C" fn get_height() -> usize {
    get_default_canvas().get_height()
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let bg = Color {
        r: 0x33,
        g: 0x33,
        b: 0x33,
        a: 0xFF,
    };
    let fg = Color {
        r: 0,
        g: 0xFF,
        b: 0,
        a: 0xFF,
    };
    let mut c = get_default_canvas();
    c.fill(&bg);
    c.rect(10, 10, c.get_width() - 20, c.get_height() - 20, &fg);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn wat() {
        assert!(5 == add(2, 3), "If this fails, we are in trouble");
    }
}
