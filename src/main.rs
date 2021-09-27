use pancurses::Window;
use pancurses::{endwin, initscr, noecho, Input};

fn main() {
    let window = initscr();
    window.refresh();
    window.keypad(true);
    noecho();
    render_line((0, 0), (40, 10), &window);
    loop {
        if let Some(Input::Character('q')) = window.getch() {
            break;
        }
    }
    endwin();
}

trait RenderOutput {
    fn set_pixel(&self, x: i32, y: i32);
    fn get_size(&self) -> (i32, i32);
}

impl RenderOutput for Window {
    fn set_pixel(&self, x: i32, y: i32) {
        self.mvaddch(y, x, '#');
    }

    fn get_size(&self) -> (i32, i32) {
        (self.get_max_x(), self.get_max_y())
    }
}

fn render_line<T>(p0: (i32, i32), p1: (i32, i32), out: &T)
where
    T: RenderOutput,
{
    let (x0, y0) = p0;
    let (x1, y1) = p1;
    match line_orientation(p0, p1) {
        LineOrientation::HorizontalIsh => {
            let ((x0, y0), (x1, y1)) = if x0 < x1 { (p0, p1) } else { (p1, p0) };
            let a = ((y1 - y0) as f64) / ((x1 - x0) as f64);
            let mut y = y0 as f64;
            for x in x0..=x1 {
                out.set_pixel(x, y as i32);
                y += a;
            }
        }
        LineOrientation::VerticalIsh => {
            let ((x0, y0), (x1, y1)) = if y0 < y1 { (p0, p1) } else { (p1, p0) };
            let a = ((x1 - x0) as f64) / ((y1 - y0) as f64);
            let mut x = x0 as f64;
            for y in y0..=y1 {
                out.set_pixel(x as i32, y);
                x += a;
            }
        }
    }
}

enum LineOrientation {
    HorizontalIsh,
    VerticalIsh,
}

fn line_orientation((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> LineOrientation {
    if (x1 - x0).abs() > (y1 - y0).abs() {
        LineOrientation::HorizontalIsh
    } else {
        LineOrientation::VerticalIsh
    }
}
