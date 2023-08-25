use rand::prelude::Distribution;
use rand::distributions::Standard;
use rand::Rng;

#[derive(Debug, Clone, Copy, Default)]
struct Range<T> {
    start: T,
    end: T,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Body {
    mass: f32,
    location: Point,
}

#[derive(Default, Copy, Clone, Debug)]
struct Cuboid {
    // since range is an iterator and thus lazy, this does not use much memory
    x: Range<f64>,
    y: Range<f64>,
    z: Range<f64>,
}

impl<T> Range<T> {
    fn midpoint(&self) -> T
    where
        T: std::ops::Add<Output = T> + std::ops::Div<Output = T> + From<u8> + Copy,
    {
        (self.start + self.end) / T::from(2u8)
    }
}

impl Cuboid {
    fn split(&self) -> [Cuboid; 8] {
        let x_mid = self.x.midpoint();
        let y_mid = self.y.midpoint();
        let z_mid = self.z.midpoint();

        let mut octants = [Cuboid::default(); 8];

        for (i, (x_sign, y_sign, z_sign)) in [
            (0., 0., 0.),
            (1., 0., 0.),
            (0., 1., 0.),
            (1., 1., 0.),
            (0., 0., 1.),
            (1., 0., 1.),
            (0., 1., 1.),
            (1., 1., 1.),
        ]
        .iter()
        .enumerate()
        {
            octants[i] = Cuboid {
                x: Range {
                    start: self.x.start + x_sign * (x_mid - self.x.start),
                    end: self.x.start + (x_sign + 1.) as f64 * (x_mid - self.x.start),
                },
                y: Range {
                    start: self.y.start + y_sign * (y_mid - self.y.start),
                    end: self.y.start + (y_sign + 1.) as f64 * (y_mid - self.y.start),
                },
                z: Range {
                    start: self.z.start + z_sign * (z_mid - self.z.start),
                    end: self.z.start + (z_sign + 1.) as f64 * (z_mid - self.z.start),
                },
            };
        }

        octants
    }
   fn octant_contains_point(&self, point: &Point) -> Option<usize> {
        let x_mid = self.x.midpoint();
        let y_mid = self.y.midpoint();
        let z_mid = self.z.midpoint();

        let x_octant = if point.x < x_mid { 0 } else { 1 };
        let y_octant = if point.y < y_mid { 0 } else { 1 };
        let z_octant = if point.z < z_mid { 0 } else { 1 };

        match (x_octant, y_octant, z_octant) {
            (0, 0, 0) => Some(0),
            (1, 0, 0) => Some(1),
            (0, 1, 0) => Some(2),
            (1, 1, 0) => Some(3),
            (0, 0, 1) => Some(4),
            (1, 0, 1) => Some(5),
            (0, 1, 1) => Some(6),
            (1, 1, 1) => Some(7),
            _ => None,
        }
    }

}


#[derive(Debug)]
struct OctreeNode {
    children: [Box<Option<OctreeNode>>; 8],
    body: Option<Body>,
    bounding_box: Cuboid,
}


impl OctreeNode {
    fn new(space: Cuboid) -> Self {
        OctreeNode {
            children: std::array::from_fn(|_| Box::new(None)),
            body: None,
            bounding_box: space
        }
    }
    fn insert(&mut self, body: Body) {
        if !self.body.is_some() {
            self.body = Some(body);
            return;
        }
        let cuboids = self.bounding_box.split();
        // get which octant the point is in
        for cube in cuboids.iter() {
        	if let Some(cuboid_idx) = cube.octant_contains_point(&body.location) {
        	    let octant = cuboids[cuboid_idx];
        	    if !self.children[cuboid_idx].is_some() {
                    // if the octant does not exist, create it
                    self.children[cuboid_idx] = Box::new(Some(octant.into()));
                }
                self.children[cuboid_idx].as_mut().as_mut().unwrap().insert(body);
                return;
            }
        }
    }
}

impl From<Cuboid> for OctreeNode {
    fn from(value: Cuboid) -> Self {
        return OctreeNode {
            body: None,
            children: std::array::from_fn(|_| Box::new(None)),
            bounding_box: value
        }
    }
}
impl Distribution<Body> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Body {
        Body {
            mass: rng.gen(),
            location: rng.gen()
        }
    }
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        Point { x: rng.gen(), y: rng.gen(), z: rng.gen() }
    }
}

struct Simulation {
    tree: OctreeNode
}

impl Simulation {
    fn new(bodies: Vec<Body>, space: Cuboid) -> Self {
        let mut root = OctreeNode::new(space);
        for body in bodies {
            root.insert(body.into());
        }
        
        Simulation { tree: root }
    }
}

fn main() {
    for step in (0..256).step_by(8) {
        let mut bodies = vec![];
        for _ in 0..8 * step {
            bodies.push(rand::random::<Body>())
        }
        
        let space = Cuboid {
            x: Range { start: 0.0, end: 1024.0 },
            y: Range { start: 0.0, end: 1024.0 },
            z: Range { start: 0.0, end: 1024.0 }
        };
        
        let instant = std::time::Instant::now();
        let simulation = Simulation::new(bodies, space);
        let after = std::time::Instant::now();
        //println!("took {:?} constructing tree for {} bodies", after - instant, step * 8);
        println!("{:?},{:?}", after - instant, step * 8)
    }
    
}


