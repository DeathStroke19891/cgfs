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
    lights: Vec<Light>,
}

struct PointL {
    position: Point,
    intensity: f64,
}

struct DirectionalL {
    intensity: f64,
    direction: Vec3,
}

enum Light {
    PointL(PointL),
    AmbientL(f64),
    DirectionalL(DirectionalL),
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
    } }

impl Vec3 {
    fn dot_product(&self, b : &Vec3) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    fn length(&self) -> f64 {
        self.dot_product(&self).sqrt()
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
            Some(s) => {
                let p: Point = Point {
                    x: self.camera.x + closest_t * d.x,
                    y: self.camera.y + closest_t * d.y,
                    z: self.camera.z + closest_t * d.z,
                };
                let n: Vec3 = p.subtract(&s.center);
                let n_nor: Vec3 = Vec3 {
                    x: n.x / n.length(),
                    y: n.y / n.length(),
                    z: n.z / n.length(),
                };
                s.color.multiply_by(self.compute_lighting(&p, &n_nor))
            },
            None => Rgb::from_ints(255, 255, 255),
        }
    }

    fn compute_lighting(&self, p: &Point, n: &Vec3) -> f64 {
        let mut intensity: f64 = 0.0;
        for light in &self.scene.lights {
            match light {
                Light::AmbientL(i) => {
                    intensity += i;
                },
                other => {
                    let l: &Vec3 = match other {
                        Light::PointL(pl) => &pl.position.subtract(&p),
                        Light::DirectionalL(dl) => &dl.direction,
                        _ => &Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                    };

                    let i = match other {
                        Light::PointL(pl) => pl.intensity,
                        Light::DirectionalL(dl) => dl.intensity,
                        _ => 0.0,
                    };
                    let n_dot_l: f64 = n.dot_product(l);
                    if n_dot_l > 0.0 {
                        intensity += (i * n_dot_l)/(n.length() * l.length());
                    }
                }
            }
        }

        intensity
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
            lights: Vec::new(),
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

    world.scene.objects.push(
        Sphere {
            center: Point {
                x: 0.0,
                y: -5001.0,
                z: 0.0,
            },
            radius: 5000.0,
            color: Rgb::from_ints(255, 255, 0),
        }
    );

    world.scene.lights.push(
        Light::AmbientL(0.2)
    );

    world.scene.lights.push(
        Light::PointL(PointL{
            intensity: 0.6,
            position: Point {
                x: 2.0,
                y: 1.0,
                z: 0.0,
            }
        })
    );

    world.scene.lights.push(
        Light::DirectionalL(DirectionalL{
            intensity: 0.2,
            direction: Vec3 {
                x: 1.0,
                y: 4.0,
                z: 4.0,
            }
        })
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
