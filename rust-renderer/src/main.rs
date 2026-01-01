use image::{Rgba, RgbaImage};
use std::mem::swap;
use rand::random;



// const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
// const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
// const RED: Rgba<u8> = Rgba([0, 0, 255, 255]);
// const BLUE: Rgba<u8> = Rgba([255, 128, 64, 255]);
// const YELLOW: Rgba<u8> = Rgba([0, 200, 255, 255]);

fn line(ax: i32, ay: i32, bx: i32, by: i32, img: &mut RgbaImage, color: Rgba<u8>) {
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
    let mut ierror = 2*(by-ay).abs();
    let mut y = ay;
    for x in ax..=bx {

        if steep {
            if y >= 0 && x >= 0 && (y as u32) < img.width() && (x as u32) < img.height() {
                img.put_pixel(y as u32, x as u32, color);
            }
        } else {
            if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
                img.put_pixel(x as u32, y as u32, color);
            }
        }
        ierror += 2*by-ay.abs();
        if ierror > by-ay {
            if by > ay {
                y += 1;
            }else{
                y -= 1;
            }
            ierror -= 2* (bx-ax)
        }

    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const WIDTH: u32 = 64;
    const HEIGHT: u32 = 64;
    let mut framebuffer = RgbaImage::new(WIDTH,HEIGHT);

    let _start = std::time::Instant::now();
    let mut i = 1;
    while i <  10{
        let color = Rgba([random::<u8>(),random::<u8>(),random::<u8>(),random::<u8>()]);
        let ax = rand::random_range(..WIDTH) as i32;
        let bx = rand::random_range(..WIDTH) as i32;
        let ay = rand::random_range(..HEIGHT) as i32;
        let by = rand::random_range(..WIDTH) as i32;
        line(ax, ay, bx, by, &mut framebuffer, color);
        i+= 1;
    }
    let _elapse = _start.elapsed();
    print!("Duration is {:?}",_elapse);

    framebuffer.save("framebuffer.png")?;
    Ok(())
}
