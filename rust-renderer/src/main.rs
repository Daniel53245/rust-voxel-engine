use image::{Rgba, RgbaImage};
use std::mem::swap;

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
const RED: Rgba<u8> = Rgba([0, 0, 255, 255]); // matches your C++ values (note: looks blue in RGBA)
const BLUE: Rgba<u8> = Rgba([255, 128, 64, 255]);
const YELLOW: Rgba<u8> = Rgba([0, 200, 255, 255]);

fn set_pixel(img: &mut RgbaImage, x: i32, y: i32, c: Rgba<u8>) {
    if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
        img.put_pixel(x as u32, y as u32, c);
    }
}


fn line(ax: i32, ay: i32, bx: i32, by: i32, img: &mut RgbaImage, color: Rgba<u8>) {
    // make mutable local copies (function parameters are immutable)
    let (mut ax, mut ay) = (ax, ay);
    let (mut bx, mut by) = (bx, by);

    let steep = (ax - bx).abs() < (ay - by).abs();

    if steep {
        swap(&mut ax, &mut ay);
        swap(&mut bx, &mut by);
    }

    if ax > bx {
        swap(&mut ax, &mut bx);
        swap(&mut ay, &mut by);
    }

    if ax == bx {
        return;
    }

    for x in ax..=bx {
        let t = (x - ax) as f32 / (bx - ax) as f32;
        let y = (ay as f32 + (by - ay) as f32 * t).round() as i32;

        // bounds check + transpose handling
        if steep {
            if y >= 0 && x >= 0 && (y as u32) < img.width() && (x as u32) < img.height() {
                img.put_pixel(y as u32, x as u32, color);
            }
        } else {
            if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
                img.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const WIDTH: u32 = 64;
    const HEIGHT: u32 = 64;

    let mut framebuffer = RgbaImage::new(WIDTH, HEIGHT);

    let (ax, ay) = (7, 3);
    let (bx, by) = (12, 37);
    let (cx, cy) = (62, 53);

    line(ax, ay, bx, by, &mut framebuffer, BLUE);
    line(cx, cy, bx, by, &mut framebuffer, GREEN);
    line(cx, cy, ax, ay, &mut framebuffer, YELLOW);
    line(ax, ay, cx, cy, &mut framebuffer, RED);

    set_pixel(&mut framebuffer, ax, ay, WHITE);
    set_pixel(&mut framebuffer, bx, by, WHITE);
    set_pixel(&mut framebuffer, cx, cy, WHITE);

    framebuffer.save("framebuffer.png")?;
    Ok(())
}
