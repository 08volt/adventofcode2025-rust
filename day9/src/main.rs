

fn read_txt_file( path : &str ) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open( path )
        .expect( &format!( "Could not open file: {}", path ) );
    let mut contents = String::new();
    file.read_to_string( &mut contents ).unwrap();
    contents
}

struct Segment {
    is_vertical: bool,
    first: (f64, f64),
    last: (f64, f64)
}

impl Segment {
    fn new(a: (f64, f64), b: (f64, f64)) -> Segment {
        let is_vertical = a.0 == b.0;

        let (first, last) = if a.0 < b.0 || (is_vertical && a.1 < b.1) {
            (a, b)
        } else {
            (b, a)
        };

        Segment {
            is_vertical,
            first,
            last,
        }
    }
}


fn main() {
    let content = read_txt_file( "input.txt" );
    let points: Vec<(f64, f64)>  = content
        .lines()
        .filter_map( |line| {
            let mut sl = line.split(",");
            let x = sl.next()?.parse().ok()?;
            let y = sl.next()?.parse().ok()?;
            Some((x, y))
        } )
        .collect();

    let mut max_area = 0;

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i+1) {
            let width = (p1.0 - p2.0).abs() + 1.0;
            let height = (p1.1 - p2.1).abs() + 1.0;
            max_area = max_area.max((width * height) as usize);
        }
    }

    println!("{}", max_area);

    let segments: Vec<Segment> = (0..points.len())
        .map(|i| Segment::new(points[i], points[(i + 1) % points.len()]))
        .collect();

    let mut max_area2 = 0;

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i+1) {
            let width = (p1.0 - p2.0).abs() + 1.0;
            let height = (p1.1 - p2.1).abs() + 1.0;
            let area = (width * height) as usize;

            if area > max_area2 && is_inside_shape(p1.0, p1.1, p2.0, p2.1, &segments) {
                max_area2 = area;
            }
        }
    }

    println!("{}", max_area2);
}


fn is_inside_shape(
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    segments: &[Segment],
) -> bool {
    let (min_x, max_x) = (x1.min(x2), x1.max(x2));
    let (min_y, max_y) = (y1.min(y2), y1.max(y2));

    for segment in segments {
        if  segment.is_vertical { 
            let X = segment.first.0;
            if X > min_x && X < max_x { 
                
                if min_y < segment.last.1 && segment.first.1 < max_y { // segment is not fully over or full below of the rectangle
                    return false;
                }
            }

        } else { 
            let Y = segment.first.1;
            if Y > min_y && Y < max_y {
                
                if min_x < segment.last.0 && segment.first.0 < max_x { // segment is not fully righ or full left of the rectangle
                    return false;
                }
            }
        }
    }

    // -> | -> |
    // center is inside the shape

    let center_x = (min_x + max_x) as f64 / 2.0;
    let center_y = (min_y + max_y) as f64 / 2.0;

    let mut intersections = 0;
    for segment in segments {
        if segment.is_vertical {
            let edge_x = segment.first.0 as f64;
            if edge_x > center_x {
                if center_y > segment.first.1 && center_y < segment.last.1 {
                    intersections += 1;
                }
            }
        }
    }

    intersections % 2 == 1
}

