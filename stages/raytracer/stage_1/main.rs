use cgfs::canvas::{Canvas, Rgb};

const INF: f64 = 100000.0;

//#[derive(Debug)]
struct World {
    scene: Scene,
    viewport: Viewport,
    camera: Point,
}

//trait Object  {
//
//}

//#[derive(Debug)]
struct Sphere {
    center: Point,
    radius: f64,
    color: Rgb,
}

//impl Object for Sphere {
//
//}

//#[derive(Debug)]
struct Viewport {
    v_x: f64,
    v_y : f64,
    d : f64,
}

//#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

//#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

//#[derive(Debug)]
struct Scene {
    //objects: Vec<Box<dyn Object>>,
    objects: Vec<Sphere>,
}


impl Viewport {
    fn point_from_canvas(&self, canvas: &Canvas, x: i32,y: i32) -> Point {
        Point {
            x: (x as f64 / canvas.width() as f64) * self.v_x,
            y: (y as f64 / canvas.height() as f64) * self.v_y,
            z: self.d,
        }
    }
}

impl Point {
    fn subtract(&self, p2: &Point) -> Vec3 {
        Vec3 {
            x: self.x - p2.x,
            y: self.y - p2.y,
            z: self.z - p2.z,
        }
    }
}

impl Vec3 {
    fn dot_product(&self, b : &Vec3) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
}

impl World {
    fn trace_ray(&self, d: &Vec3, min_t: f64, max_t: f64) -> Rgb {
        let mut closest_t = INF;
        let mut closest_sphere : Option<&Sphere> = None;
        for sphere in &self.scene.objects {
            let t: (f64, f64) = intersect_ray_sphere(&self.camera, d, sphere);
            if ((min_t <= t.0) && (t.0 <= max_t)) && (t.0 < closest_t) {
                closest_t = t.0;
                closest_sphere = Some(sphere);
            }
            if ((min_t <= t.1) && (t.1 <= max_t)) && (t.1 < closest_t) {
                closest_t = t.1;
                closest_sphere = Some(sphere);
            }
        }
        match closest_sphere {
            Some(s) => s.color,
            None => Rgb::from_ints(255, 255, 255),
        }
    }
}

fn intersect_ray_sphere(o: &Point, d: &Vec3, s: &Sphere) -> (f64, f64){
    let r = s.radius;
    let co = o.subtract(&s.center);

    let a = d.dot_product(&d);
    let b = 2.0*co.dot_product(&d);
    let c = co.dot_product(&co) - r*r;

    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return (INF, INF);
    }
    let t1 = ( -b + discriminant.sqrt()) / (2.0*a);
    let t2 = ( -b - discriminant.sqrt()) / (2.0*a);
    (t1,t2)
}

fn main() {
    let mut canvas = Canvas::new("Raytracer", 800, 800);

    canvas.clear_canvas(&Rgb::from_ints(255,255,255));

    let mut world: World = World {
        viewport: Viewport{
            v_x: 1.0,
            v_y: 1.0,
            d: 1.0
        },
        camera: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        scene: Scene {
            objects: Vec::new(),
        }
    };


    world.scene.objects.push(
        Sphere {
            center: Point {
                x: 0.0,
                y: -1.0,
                z: 3.0,
            },
            radius: 1.0,
            color: Rgb::from_ints(255, 0, 0),
        }
    );

    world.scene.objects.push(
        Sphere {
            center: Point {
                x: 2.0,
                y: 0.0,
                z: 4.0,
            },
            radius: 1.0,
            color: Rgb::from_ints(0, 0, 255),
        }
    );

    world.scene.objects.push(
        Sphere {
            center: Point {
                x: -2.0,
                y: 0.0,
                z: 4.0,
            },
            radius: 1.0,
            color: Rgb::from_ints(0, 255, 0),
        }
    );

    let mut x: i32 = -(canvas.width() as i32)/2;
    while x < (canvas.width() as i32)/2 {
        let mut y: i32 = -(canvas.height() as i32)/2;
        while y < (canvas.height() as i32)/2 {
            let v: Point = world.viewport.point_from_canvas(&canvas,x,y);
            let d: Vec3 = v.subtract(&world.camera);
            let color: Rgb = world.trace_ray(&d, 1.0, INF);
            canvas.put_pixel(x, y, &color);
            y += 1;
        }
        x += 1;
    }

   canvas.display_until_exit();
}
