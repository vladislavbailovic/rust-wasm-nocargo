mod canvas;

pub mod interface;
pub use interface::*;

mod map_data;

#[derive(Debug)]
pub struct Node {
    lat: f32,
    lon: f32,
}

#[cfg(test)]
mod test {
    use map_data::MAP_NODES;

    #[test]
    fn node_bounding_box() {
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
        assert!(min_lat > 0.0);
        assert!(min_lat < max_lat);
        assert!(min_lon > 0.0);
        assert!(min_lon < max_lon);
    }
}
