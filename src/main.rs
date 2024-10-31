use cgfs::canvas::{Canvas, Rgb};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
    let mut my_canvas = Canvas::new("A tiny red 'H'", WIDTH, HEIGHT);

    let red = Rgb::from_ints(255, 0, 0);

    my_canvas.put_pixel(-1, 1, &red);
    my_canvas.put_pixel(-1, 0, &red);
    my_canvas.put_pixel(-1, -1, &red);
    my_canvas.put_pixel(0, 0, &red);
    my_canvas.put_pixel(1, 1, &red);
    my_canvas.put_pixel(1, 0, &red);
    my_canvas.put_pixel(1, -1, &red);

    my_canvas.display_until_exit();
}
