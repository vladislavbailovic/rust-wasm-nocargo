use canvas::{get_default_canvas, Color};
use map_data::*;

extern "C" {
    fn wasmLogInt(a: usize);
}

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

    let mut min_lat: f32 = f32::MAX;
    let mut max_lat: f32 = 0.0;
    let mut min_lon: f32 = f32::MAX;
    let mut max_lon: f32 = 0.0;
    for node in &MAP_NODES {
        if node.lat > max_lat {
            max_lat = node.lat;
        }
        if node.lat < min_lat {
            min_lat = node.lat;
        }
        if node.lon > max_lon {
            max_lon = node.lon;
        }
        if node.lon < min_lon {
            min_lon = node.lon;
        }
    }

    let lon_diff = max_lon - min_lon;
    let lat_diff = max_lat - min_lat;
    let node_width = 5;
    for node in &MAP_NODES {
        let rel_x = (node.lon - min_lon) / lon_diff;
        let rel_y = (node.lat - min_lat) / lat_diff;
        let x_px = (rel_x * c.get_width() as f32) as usize;
        let y_px = (rel_y * c.get_height() as f32) as usize;
        let mut x0 = if x_px <= node_width / 2 {
            0
        } else {
            (x_px - node_width / 2) - 1
        };
        if x0 + node_width > c.get_width() {
            x0 = c.get_width() - node_width;
        }

        let mut y0 = if y_px <= node_width / 2 {
            0
        } else {
            (y_px - node_width / 2) - 1
        };
        if y0 + node_width > c.get_height() {
            y0 = c.get_height() - node_width;
        }

        c.rect(x0, y0, node_width, node_width, &fg);
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
