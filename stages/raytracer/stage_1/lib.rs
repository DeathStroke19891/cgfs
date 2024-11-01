pub mod canvas {
    use minifb::{Key, ScaleMode, Window, WindowOptions};

    #[derive(Debug)]
    pub struct Canvas {
        window: Window,
        buffer: Vec<u32>,
        width: usize,
        height: usize,
    }

    impl Canvas {
        pub fn new(name: &str, width: usize, height: usize) -> Self {
            let mut window = Window::new(
                name,
                width,
                height,
                WindowOptions {
                    resize: false,
                    scale_mode: ScaleMode::UpperLeft,
                    ..WindowOptions::default()
                },
            )
            .expect("Window creation failed");

            window.limit_update_rate(Some(std::time::Duration::from_micros(24_000)));

            let mut buffer: Vec<u32> = Vec::with_capacity(width * height);
            buffer.resize(width * height, 0);
            Canvas {
                window,
                buffer,
                width,
                height,
            }
        }

        pub fn clear_canvas(&mut self, color: &Rgb) {
            let col: u32 =
                (color.red as u32) * 65536 + (color.green as u32) * 256 + (color.blue as u32);

            for i in 0..self.buffer.len() {
                self.buffer[i] = col;
            }
        }

        pub fn put_pixel(&mut self, x: i32, y: i32, color: &Rgb) {
            let (width, height) = (self.width as i32, self.height as i32);

            let screen_x = width / 2 + x;
            let screen_y = height / 2 - y - 1;

            if (screen_x < 0) | (screen_x >= width) | (screen_y < 0) | (screen_y >= height) {
                return;
            }

            let pixel_pos_in_buffer = (screen_x + width * screen_y) as usize;

            self.buffer[pixel_pos_in_buffer] =
                (color.red as u32) * 65536 + (color.green as u32) * 256 + (color.blue as u32);
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.height
        }
        pub fn display_until_exit(&mut self) {
            // The unwrap causes the code to exit if the update fails
            while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
                self.window
                    .update_with_buffer(&self.buffer, self.width, self.height)
                    .unwrap();
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Rgb {
        pub red: f64,
        pub green: f64,
        pub blue: f64,
    }

    impl Rgb {
        pub fn multiply_by(&self, m: f64) -> Rgb {
            Self {
                red: self.red * m,
                green: self.green * m,
                blue: self.blue * m,
            }
        }

        pub fn add(&self, a: &Rgb) -> Rgb {
            Self {
                red: self.red + a.red,
                green: self.green + a.green,
                blue: self.blue + a.blue,
            }
        }

        #[rustfmt::skip]
        pub fn clamp(&self) -> Rgb {
            Rgb {
                red:   f64::min(255.0, f64::max(0.0, self.red)),
                green: f64::min(255.0, f64::max(0.0, self.green)),
                blue:  f64::min(255.0, f64::max(0.0, self.blue)),
            }
        }

        pub fn from_ints(red: i16, green: i16, blue: i16) -> Rgb {
            Rgb {
                red: red as f64,
                green: green as f64,
                blue: blue as f64,
            }
        }
    }
}
