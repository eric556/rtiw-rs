use minifb::MENU_KEY_CTRL;
use minifb::{InputCallback, Key, Menu, Scale, Window, WindowOptions};
use std::time::Instant;
use cgmath::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn at(&self, t: f64) -> Vector3<f64> {
        return self.origin + t * self.direction;
    }
}

struct KeyCharCallback;

impl InputCallback for KeyCharCallback {
    fn add_char(&mut self, c: u32) {
    }
}

pub fn test_image(buffer: &mut Vec<u32>) {
    for y in 0..HEIGHT{
        for x in 0..WIDTH{
            let r: f32 = (x as f32) / ((WIDTH - 1) as f32);
            let g: f32 = (y as f32) / ((HEIGHT - 1) as f32);
            let b: f32 = 0.25;

            let index = WIDTH * y + x;
            let pixel: u32 = ((((r * 255.0) as u8) as u32) << 16) | ((((g * 255.0) as u8) as u32) << 8) | (((b * 255.0) as u8) as u32);

            buffer[index] = pixel;
        }
    }
}

pub fn ray_color(ray: &Ray) -> Vector3<f64> {
    let t = hit_sphere(&Vector3::new(0f64, 0f64, -1f64), 0.5f64, ray);
    if t > 0.0 {
        let N = (ray.at(t) - Vector3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * Vector3::new(N.x + 1.0, N.y + 1.0, N.z + 1.0);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let color1 = Vector3::new(1.0f64, 1.0f64, 1.0f64);
    let color2 = Vector3::new(0.5f64, 0.7f64, 1.0f64);

    return ((1.0 - t) * color1) + (t * color2);
}

pub fn hit_sphere(center: &Vector3<f64>, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.magnitude2();
    let half_b = Vector3::<f64>::dot(oc, ray.direction);
    let c = oc.magnitude2() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

pub fn convert_to_0rgb_f64(vec: &Vector3<f64>) -> u32 {
    return ((((vec.x * 255.0) as u8) as u32) << 16) | ((((vec.y * 255.0) as u8) as u32) << 8) | (((vec.z * 255.0) as u8) as u32);
}

pub fn convert_to_0rgb_u8(vec: &Vector3<u8>) -> u32 {
    return ((vec.x as u32) << 16) | ((vec.y as u32) << 8)  | (vec.x as u32);
}

pub fn write_color(x: usize, y: usize, color: Vector3<u8>, buffer: &mut Vec<u32>) {
    let index = WIDTH * y + x;
    let pixel: u32 = ((color.x as u32) << 16) | ((color.y as u32) << 8) | (color.z as u32);

    buffer[index] = pixel;
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Ray Tracing in One Weekend",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to Open Window");

    window.set_input_callback(Box::new(KeyCharCallback {}));

    // test_image(&mut buffer);

    let viewport_height = 2.0;
    let viewport_width = (16.0 / 9.0) * viewport_height;
    let focal_length = 1.0;

    let origin = Vector3::new(0f64, 0f64, 0f64);
    let horizontal = Vector3::new(viewport_width, 0f64, 0f64);
    let vertical = Vector3::new(0f64, viewport_height, 0f64);
    let lower_left_corner = origin - (horizontal / 2f64) - (vertical / 2f64) - Vector3::new(0f64, 0f64, focal_length);    

    while window.is_open() && !window.is_key_down(Key::Escape) {

        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    _ => (),
                }
            }
        });

        let now = Instant::now();

        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                let index = WIDTH * y + x;
                let u = (x as f64) / ((WIDTH - 1) as f64);
                let v = (y as f64) / ((HEIGHT - 1) as f64);
                let ray = Ray{
                    origin: origin,
                    direction: lower_left_corner + u * horizontal + v * vertical - origin
                };
    
                buffer[index] = convert_to_0rgb_f64(&ray_color(&ray));
            }
        }

        let fps = 1f64 / now.elapsed().as_secs_f64();
        window.set_title(&format!("Ray Tracing in One Weekend: {}", fps));

        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}