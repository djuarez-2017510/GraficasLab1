use image::{ImageBuffer, RgbImage, Rgb};

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn draw_line(img: &mut RgbImage, p1: Point, p2: Point, color: Rgb<u8>) {
    let dx = (p2.x - p1.x).abs();
    let sx = if p1.x < p2.x { 1 } else { -1 };
    let dy = -(p2.y - p1.y).abs();
    let sy = if p1.y < p2.y { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = p1.x;
    let mut y = p1.y;

    loop {
        if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
            img.put_pixel(x as u32, y as u32, color);
        }
        if x == p2.x && y == p2.y {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn fill_polygon(img: &mut RgbImage, points: &[Point], color: Rgb<u8>) {
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    for y in min_y..=max_y {
        let mut intersections = vec![];

        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];

            if (p1.y <= y && p2.y > y) || (p2.y <= y && p1.y > y) {
                let x = p1.x + (y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                intersections.push(x);
            }
        }

        intersections.sort();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                for x in intersections[i]..=intersections[i + 1] {
                    if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
                        img.put_pixel(x as u32, y as u32, color);
                    }
                }
            }
        }
    }
}

fn flip_y(points: &[Point], height: i32) -> Vec<Point> {
    points.iter().map(|p| Point { x: p.x, y: height - p.y }).collect()
}

fn main() {
    let height = 600;
    let mut img: RgbImage = ImageBuffer::from_pixel(800, height as u32, Rgb([255, 255, 255]));

    let yellow = Rgb([255, 255, 0]);  // polígono 1
    let blue   = Rgb([0, 0, 255]);    // polígono 2
    let red    = Rgb([255, 0, 0]);    // polígono 3
    let green  = Rgb([0, 255, 0]);    // polígono 4
    let white  = Rgb([255, 255, 255]);

    let poly1 = flip_y(&vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ].into_iter().map(|(x, y)| Point { x, y }).collect::<Vec<_>>(), height);

    let poly2 = flip_y(&vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ].into_iter().map(|(x, y)| Point { x, y }).collect::<Vec<_>>(), height);

    let poly3 = flip_y(&vec![
        (377, 249), (411, 197), (436, 249),
    ].into_iter().map(|(x, y)| Point { x, y }).collect::<Vec<_>>(), height);

    let poly4 = flip_y(&vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ].into_iter().map(|(x, y)| Point { x, y }).collect::<Vec<_>>(), height);

    let hole5 = flip_y(&vec![
        (682, 175), (708, 120), (735, 148), (739, 170),
    ].into_iter().map(|(x, y)| Point { x, y }).collect::<Vec<_>>(), height);

    fill_polygon(&mut img, &poly1, yellow);
    fill_polygon(&mut img, &poly2, blue);
    fill_polygon(&mut img, &poly3, red);
    fill_polygon(&mut img, &poly4, green);
    fill_polygon(&mut img, &hole5, white);

    for poly in [&poly1, &poly2, &poly3, &poly4, &hole5] {
        for i in 0..poly.len() {
            draw_line(&mut img, poly[i], poly[(i + 1) % poly.len()], white);
        }
    }

    img.save("out.bmp").unwrap();   
}
