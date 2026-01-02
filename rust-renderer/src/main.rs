use clap::Parser;
use image::{Rgba, RgbaImage};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

// const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
// const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
// const RED: Rgba<u8> = Rgba([0, 0, 255, 255]);
// const BLUE: Rgba<u8> = Rgba([255, 128, 64, 255]);
// const YELLOW: Rgba<u8> = Rgba([0, 200, 255, 255]);

// fn line(ax: i32, ay: i32, bx: i32, by: i32, img: &mut RgbaImage, color: Rgba<u8>) {
//     let (mut ax, mut ay) = (ax, ay);
//     let (mut bx, mut by) = (bx, by);

//     let steep = (ax - bx).abs() < (ay - by).abs();

//     if steep {
//         swap(&mut ax, &mut ay);
//         swap(&mut bx, &mut by);
//     }

//     if ax > bx {
//         swap(&mut ax, &mut bx);
//         swap(&mut ay, &mut by);
//     }

//     if ax == bx {
//         return;
//     }
//     let mut ierror = 2*(by-ay).abs();
//     let mut y = ay;
//     for x in ax..=bx {

//         if steep {
//             if y >= 0 && x >= 0 && (y as u32) < img.width() && (x as u32) < img.height() {
//                 img.put_pixel(y as u32, x as u32, color);
//             }
//         } else {
//             if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
//                 img.put_pixel(x as u32, y as u32, color);
//             }
//         }
//         ierror += 2*by-ay.abs();
//         if ierror > by-ay {
//             if by > ay {
//                 y += 1;
//             }else{
//                 y -= 1;
//             }
//             ierror -= 2* (bx-ax)
//         }

//     }
// }

fn line(ax: i32, ay: i32, bx: i32, by: i32, img: &mut RgbaImage, color: Rgba<u8>) {
    let (mut x0, mut y0, mut x1, mut y1) = (ax, ay, bx, by);

    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = (y1 - y0).abs();
    let mut err = dx / 2;
    let ystep = if y0 < y1 { 1 } else { -1 };
    let mut y = y0;

    for x in x0..=x1 {
        let (px, py) = if steep { (y, x) } else { (x, y) };
        if px >= 0 && py >= 0 && (px as u32) < img.width() && (py as u32) < img.height() {
            img.put_pixel(px as u32, py as u32, color);
        }

        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }
}

fn draw_face(
    vertices: &Vec<Vec<f64>>,
    target_face: &Vec<usize>,
    img: &mut RgbaImage,
) -> Result<(), String> {
    // the target face have 3 points where it is the index of point in vertices
    // for each of the 2 points draw a line
    let &[a, b, c] = target_face.as_slice() else {
        panic!("target_face must have exactly 3 indices");
    };
    let get_xy = |idx: usize| -> (i32, i32) {
        let v = &vertices[idx];
        (v[0] as i32, v[1] as i32)
    };

    let (ax, ay) = get_xy(a);
    let (bx, by) = get_xy(b);
    let (cx, cy) = get_xy(c);

    let color = Rgba([255, 0, 0, 255]);

    line(ax, ay, bx, by, img, color);
    line(ax, ay, cx, cy, img, color);
    line(bx, by, cx, cy, img, color);

    Ok(())
}

// fn process_wf3d_vertice(line: &str, frame_size: &u32) -> Result<Vec<f64>, String> {
//     let mut vertice = Vec::new();
//     let mut parts = line.split_whitespace();
//     parts.next();
//     for part in parts {
//         let mut cord: f64 = part.parse().map_err(|_| format!("invalid formt:{part}"))?;
//         cord = cord + 1.0;
//         cord = cord * (frame_size / 2) as f64;

//         vertice.push(cord);
//     }
//     Ok(vertice)
// }

fn process_wf3d_vertice(line: &str, frame_size: &u32) -> Result<Vec<f64>, String> {
    let mut vertice = Vec::new();
    let mut parts = line.split_whitespace();
    parts.next(); // skip "v"

    let half = (*frame_size as f64) / 2.0;

    for (i, part) in parts.enumerate() {
        let mut coord: f64 = part
            .parse()
            .map_err(|_| format!("invalid format: {part}"))?;

        // map from [-1,1] → [0,size]
        coord = (coord + 1.0) * half;

        // 🔥 flip Y axis (index 1 = y)
        if i == 1 {
            coord = (*frame_size as f64 - 1.0) - coord;
        }

        vertice.push(coord);
    }

    Ok(vertice)
}

fn process_wf3d_face(line: &str) -> Result<Vec<usize>, String> {
    let mut vertices = Vec::new();
    let mut parts = line.split_whitespace();
    parts.next(); // skip "f"

    for part in parts {
        let v_str = part.split('/').next().ok_or("empty face element")?;

        let index: usize = v_str
            .parse()
            .map_err(|_| format!("invalid index: {v_str}"))?;

        vertices.push(index - 1);
    }

    Ok(vertices)
}

fn read_wf3d_object(
    file_path: &str,
    frame_size: &u32,
) -> Result<(Vec<Vec<f64>>, Vec<Vec<usize>>), String> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    for (line_no, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| format!("I/O error on line {}: {e}", line_no + 1))?;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with("v ") {
            let v = process_wf3d_vertice(&line, frame_size)
                .map_err(|e| format!("line {}: {e}", line_no + 1))?;
            vertices.push(v);
        } else if line.starts_with("f ") {
            let f = process_wf3d_face(&line).map_err(|e| format!("line {}: {e}", line_no + 1))?;
            faces.push(f);
        } else {
            // ignore: vt, vn, o, g, usemtl, s, etc.
            continue;
        }
    }
    Ok((vertices, faces))
}
#[derive(Parser)]
struct Args {
    file_path: String,
    #[arg(default_value_t = 800)]
    size: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (v, f) = read_wf3d_object(&args.file_path, &args.size)?;
    let mut img = RgbaImage::new(args.size, args.size);

    for target_face in f {
        draw_face(&v, &target_face, &mut img)?;
    }

    img.save("rendered_image.png")?;
    Ok(())
}
