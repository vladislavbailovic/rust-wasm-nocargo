mod canvas;

mod client;
pub mod interface;
pub use interface::*;

mod map_data;

#[derive(Debug)]
pub struct Node {
    lat: f32,
    lon: f32,
}

impl Node {
    pub fn rel_x(&self, bounds: &Area) -> f32 {
        let diff = bounds.diff();
        (self.lon - bounds.min.lon) / diff.lon
    }

    pub fn rel_y(&self, bounds: &Area) -> f32 {
        let diff = bounds.diff();
        (self.lat - bounds.min.lat) / diff.lat
    }

    pub fn relative_to(&self, bounds: &Area) -> RelativePoint {
        RelativePoint {
            x: self.rel_x(bounds),
            y: self.rel_y(bounds),
        }
    }
}

pub struct RelativePoint {
    x: f32,
    y: f32,
}

impl RelativePoint {
    pub fn project_onto(&self, width: usize, height: usize, size: usize) -> Option<AbsolutePoint> {
        let x = Self::project_dimension(self.x, width, size)?;
        let y = Self::project_dimension(self.y, height, size)?;
        Some(AbsolutePoint { x, y })
    }

    fn project_dimension(src: f32, dest: usize, size: usize) -> Option<usize> {
        let p0 = (src * dest as f32) as usize;
        let point = if p0 <= size / 2 {
            return None;
        } else {
            p0 - size / 2
        };
        if point + size > dest {
            return None;
        }
        Some(point)
    }
}

pub struct AbsolutePoint {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod points {
    use super::*;

    #[test]
    fn relative_to_absolute_no_size() {
        let p0 = RelativePoint { x: 0.25, y: 0.1 };
        if let Some(p1) = p0.project_onto(10, 10, 1) {
            assert_eq!(p1.x, 2);
            assert_eq!(p1.y, 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn relative_to_absolute_with_size() {
        let p0 = RelativePoint { x: 0.25, y: 0.25 };
        if let Some(p1) = p0.project_onto(10, 10, 3) {
            assert_eq!(p1.x, 1);
            assert_eq!(p1.y, 1);
        } else {
            assert!(false);
        }
    }
}

pub struct Area {
    max: Node,
    min: Node,
}

impl Area {
    pub fn new(nodes: &[Node]) -> Self {
        let mut min = Node {
            lat: f32::MAX,
            lon: f32::MAX,
        };
        let mut max = Node {
            lat: f32::MIN,
            lon: f32::MIN,
        };
        for node in nodes {
            min.lat = min.lat.min(node.lat);
            min.lon = min.lon.min(node.lon);

            max.lat = max.lat.max(node.lat);
            max.lon = max.lon.max(node.lon);
        }
        Self { min, max }
    }

    fn diff(&self) -> Node {
        Node {
            lat: self.max.lat - self.min.lat,
            lon: self.max.lon - self.min.lon,
        }
    }
}

#[cfg(test)]
mod node {
    use super::*;
    use map_data::MAP_NODES;

    #[test]
    fn rel_x() {
        let b = Area::new(&MAP_NODES);
        for node in &MAP_NODES {
            assert!(node.rel_x(&b) >= 0.0, "{}", node.rel_x(&b));
            assert!(node.rel_x(&b) <= 1.0, "{}", node.rel_x(&b));
        }
    }

    #[test]
    fn rel_y() {
        let b = Area::new(&MAP_NODES);
        for node in &MAP_NODES {
            assert!(node.rel_y(&b) >= 0.0, "{}", node.rel_y(&b));
            assert!(node.rel_y(&b) <= 1.0, "{}", node.rel_y(&b));
        }
    }
}

#[cfg(test)]
mod bounding_box {
    use super::*;
    use map_data::MAP_NODES;

    #[test]
    fn bounds_new() {
        let b = Area::new(&MAP_NODES);

        assert!(b.min.lat > f32::MIN, "lat: {:?} > {}", b.min, f32::MIN);
        assert!(b.min.lat < b.max.lat, "lat: {:?} < {:?}", b.min, b.max);

        assert!(b.min.lon > f32::MIN, "lon: {:?} > {}", b.min, f32::MIN);
        assert!(b.min.lon < b.max.lon, "lon: {:?} < {:?}", b.min, b.max);
    }

    #[test]
    fn bounds_diff() {
        let b = Area::new(&MAP_NODES);
        let diff = b.diff();

        assert!(diff.lat > 0.0);
        assert!(diff.lat < b.min.lat, "{} < {}", diff.lat, b.min.lat);
        assert!(diff.lat < b.max.lat, "{} < {}", diff.lat, b.min.lat);

        assert!(diff.lon > 0.0);
        assert!(diff.lon < b.min.lon, "{} < {}", diff.lon, b.min.lon);
        assert!(diff.lon < b.max.lon, "{} < {}", diff.lon, b.max.lon);
    }
}
