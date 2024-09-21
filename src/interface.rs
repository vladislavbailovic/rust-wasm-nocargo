use super::{client, Area};
use canvas::{get_default_canvas, Color};
use map_data::MAP_NODES;

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
        r: 0x13,
        g: 0x32,
        b: 161,
        a: 0xFF,
    };
    let fg = Color {
        r: 13,
        g: 0xFF,
        b: 13,
        a: 0xFF,
    };
    let mut c = get_default_canvas();
    client::log(&format!("Background: [{:?}], foreground: [{:?}]", bg, fg));
    c.fill(&bg);

    let area = Area::new(&MAP_NODES);
    let node_size = 5;
    for node in &MAP_NODES {
        let rel = node.relative_to(&area);
        if let Some(point) = rel.project_onto(c.get_width(), c.get_height(), node_size) {
            c.rect(point.x, point.y, node_size, node_size, &fg);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn wat() {
        assert!(5 == add(2, 3), "If this fails, we are in trouble");
    }
}
