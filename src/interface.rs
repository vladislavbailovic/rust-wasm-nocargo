use super::{client, Area, Node};
use canvas::{get_default_canvas, Color};
use map_data::{get_map_ways, MAP_NODES};

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
    let fg1 = Color {
        r: 13,
        g: 0xFF,
        b: 12,
        a: 0xFF,
    };
    let fg2 = Color {
        r: 0xFF,
        g: 13,
        b: 12,
        a: 0xFF,
    };
    let mut c = get_default_canvas();
    client::log(&format!("Background: [{:?}], foreground: [{:?}]", bg, fg1));
    c.fill(&bg);

    let area = Area::new(&MAP_NODES);
    let node_size = 5;

    for way in get_map_ways() {
        let nodes: Vec<&Node> = way.iter().map(|&nid| &MAP_NODES[nid]).collect();
        for i in 0..nodes.len() - 1 {
            let p1 = if let Some(point) =
                nodes[i]
                    .relative_to(&area)
                    .project_onto(c.get_width(), c.get_height(), node_size)
            {
                point
            } else {
                continue;
            };
            // c.rect(p1.x, p1.y, node_size, node_size, &fg1);
            let p2 = if let Some(point) = nodes[i + 1].relative_to(&area).project_onto(
                c.get_width(),
                c.get_height(),
                node_size,
            ) {
                point
            } else {
                continue;
            };
            // c.rect(p2.x, p2.y, node_size, node_size, &fg2);

            c.line(p1.x, p1.y, p2.x, p2.y, &fg1);
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
