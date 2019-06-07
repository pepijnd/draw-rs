use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    pub fn new(x: f64, y: f64) -> Coord {
        Coord { x, y }
    }

    fn distance(&self, other: &Self) -> f64 {
        (self.x - other.x).hypot(self.y - other.y)
    }

    fn delta(&self, other: &Self) -> Coord {
        Coord::new(other.x - self.x, other.y - self.y)
    }

    fn sum2(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    fn radius(&self, c1: &Self, c2: &Self) -> f64 {
        let c0 = self;
        let d = c1.delta(c1);
        let e = c1.delta(c2);

        let b = d.sum2();
        let c = e.sum2();
        let a = 0.5 / (d.x * e.y - d.y * e.x);

        let x = (e.y * b - d.y * c) * a;
        let y = (d.x * c - e.x * b) * a;

        x.powi(2) + y.powi(2)
    }

    fn center(&self, c1: &Self, c2: &Self) -> Coord {
        let c0 = self;
        let d = c1.delta(c1);
        let e = c1.delta(c2);

        let b = d.sum2();
        let c = e.sum2();
        let a = 0.5 / (d.x * e.y - d.y * e.x);

        let x = c0.x + (e.y * b - d.y * c) * a;
        let y = c0.y + (d.x * c - e.x * b) * a;

        Coord::new(x, y)
    }

    fn orient(&self, c1: &Self, c2: &Self) -> bool {
        let c0 = self;
        (c1.y - c0.y) * (c2.x - c1.x) - (c1.x - c0.x) * (c2.y-c1.y) < 0f64
    }
}


pub struct Delaun {
    coords: Vec<Coord>,
}

impl Delaun {
    pub fn calc(coords: Vec<Coord>) -> Option<DelaunResult> {
        let n = coords.len();

        let max_triangles = 2 * n - 5;
        let triangles: Vec<Triangle> = Vec::with_capacity(max_triangles);
        let halfedges: Vec<HalfEdge> = Vec::with_capacity(max_triangles);

        let hull_prev: HashMap<usize, usize> = HashMap::new();
        let hull_tri: HashMap<usize, usize> = HashMap::new();
        //let hull_Hash: HashMap<usize, usize> = HashMap::new();

        let minX = coords
            .iter()
            .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap())
            .unwrap()
            .x;
        let minY = coords
            .iter()
            .min_by(|a, b| a.y.partial_cmp(&b.y).unwrap())
            .unwrap()
            .y;
        let maxX = coords
            .iter()
            .max_by(|a, b| a.x.partial_cmp(&b.x).unwrap())
            .unwrap()
            .x;
        let maxY = coords
            .iter()
            .max_by(|a, b| a.y.partial_cmp(&b.y).unwrap())
            .unwrap()
            .y;

        let center = Coord::new((minX + maxX) / 2f64, (minY + maxY) / 2f64);

        let mut c0: Option<&Coord> = None;
        let mut min_dist: Option<f64> = None;
        for coord in coords.iter() {
            let d = center.distance(coord);
            if min_dist.is_none() || d < min_dist.unwrap() {
                min_dist = Some(d);
                c0 = Some(coord);
            }
        }
        let c0 = c0.unwrap();

        let mut c1: Option<&Coord> = None;
        let mut min_dist: Option<f64> = None;
        for coord in coords.iter() {
            if std::ptr::eq(coord, c0) {
                continue;
            };
            let d = c0.distance(coord);
            if min_dist.is_none() || d < min_dist.unwrap() {
                min_dist = Some(d);
                c1 = Some(coord);
            }
        }
        let c1 = c1.unwrap();

        let mut c2: Option<&Coord> = None;
        let mut min_radius: Option<f64> = None;
        for coord in coords.iter() {
            if std::ptr::eq(coord, c0) || std::ptr::eq(coord, c1) {
                continue;
            };
            let r = coord.radius(c0, c1);
            if min_radius.is_none() || r < min_radius.unwrap() {
                c2 = Some(coord);
                min_radius = Some(r);
            }
        }
        let c2 = c2.unwrap();
        
        let orient = c0.orient(c1, c2);
        let (c0, c1, c2) = match orient {
            true  => {(c0, c2, c1)},
            false => {(c0, c1, c2)}
        };

        let center = c0.center(c1, c2);

        struct CoordDistance<'a> {
            coord: &'a Coord,
            distance: f64
        }

        let mut distances: Vec<CoordDistance> = Vec::with_capacity(n);

        for coord in coords.iter() {
            distances.push(CoordDistance {coord, distance: center.distance(coord)});
        }
        distances.sort_by(|a, b| { a.distance.partial_cmp(&b.distance).unwrap() });

        let hullstart = c0;
        let hullsize = 3;

        None
    }
} 

pub struct Triangle {
    coords_ids: Vec<usize>,
}

pub struct HalfEdge {
    coord_ids: Vec<usize>,
}

pub struct DelaunResult {
    coords: Vec<Coord>,
    triangles: Vec<Triangle>,
    halfedges: Vec<HalfEdge>,
}
