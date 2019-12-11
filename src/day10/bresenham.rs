#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub type Line = Vec<Point>;

fn plot_line_low(x0: isize, y0: isize, x1: isize, y1: isize) -> Line {
    let mut line = Line::new();

    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi: isize = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = 2 * dy - dx;
    let mut y = y0;

    for x in x0..x1 {
        line.push(Point { x, y });
        if d > 0 {
            y = y + yi;
            d = d - 2 * dx;
        }
        d = d + 2 * dy;
    }

    line
}

fn plot_line_high(x0: isize, y0: isize, x1: isize, y1: isize) -> Line {
    let mut line = Line::new();

    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi: isize = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = 2 * dx - dy;
    let mut x = x0;

    for y in y0..y1 {
        line.push(Point { x, y });
        if d > 0 {
            x = x + xi;
            d = d - 2 * dy;
        }
        d = d + 2 * dx;
    }

    line
}

pub fn plot_line(x0: isize, y0: isize, x1: isize, y1: isize) -> Line {
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            plot_line_low(x1, y1, x0, y0)
        } else {
            plot_line_low(x0, y0, x1, y1)
        }
    } else {
        if y0 > y1 {
            plot_line_high(x1, y1, x0, y0)
        } else {
            plot_line_high(x0, y0, x1, y1)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plot_line() {
        println!("{:?}", plot_line(0, 1, 6, 4));
    }
}
