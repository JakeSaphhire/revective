use serialport as sp;
use image::{imageops::*, GenericImageView};
use std::time::{Duration, Instant};
use std::thread;
mod point;

fn main() -> Result<(), image::ImageError> {
    let ports = sp::available_ports().expect("Failed to list ports");
    let mut wport = sp::new(&ports[0].port_name, 115200).open().expect("Failed to open port");

    let mut npoints : u32 = 0;
    let display = image::io::Reader::open("images/image.png")?.with_guessed_format()?.decode()?.grayscale();
    let points : Vec<Option<point::Point>> = display.pixels().map(
            |pixel| {
                if pixel.2[0] < 127 {
                    npoints += 1;
                    Some(point::Point::new(0b00001000, ((pixel.0 as f32/display.width() as f32)*4096f32) as u16, 4096u16-((pixel.1 as f32/display.height() as f32)*4096f32) as u16))
                } else {
                    None
                }
            }
    ).collect();
    let r = display.save("images/image-sent.png");
    match r {
        Ok(_v) => (),
        Err(e) => println!("Failed to send image: {:?}", e),
    }
    println!("{} points to draw, {} points to traverse ({:2.2}%)", npoints, points.len(), (npoints as f32/points.len() as f32)*100f32);
    // Diagnostic, debug and control variables
    // Sent ratio variables
    let mut s : u32 = 0;
    let mut p : u32 = 0;
    // Loop index variable
    let mut i : u32 = 0;
    // Used to display time used per frame
    let mut times : Vec<u128> = Vec::new();
    // Used to control amount of points drawn
    let mut flip : u16 = 0;
    const RATIO : u16 = 4;
    // Drawing code
    loop {
        if i >= 10 {break;}
        else {i += 1;}
        let now = Instant::now();
        for point in points.iter() {
            match point {
                None => (),
                Some(pt) => {
                    if flip == RATIO {
                        p += 1;
                        if match wport.bytes_to_write() {
                            Ok(r) => r,
                            Err(_e) => return Ok(()),
                        } != 0 {
                            thread::sleep(std::time::Duration::from_nanos(1));
                        }
                        match pt.send(wport.as_mut()) {
                            Ok(_v) => {s += 1;}
                            Err(_e) => (), 
                        }
                        flip = 0;
                    } else {
                        flip += 1;
                    } 
                }
            }
        }
        times.push(now.elapsed().as_millis());
    }
    //Final diagnostic line
    let average_time : f64 = times.iter().sum::<u128>() as f64/times.len() as f64;
    println!("{:2.2}% of write requests sucessful, ({}/{}), in {}ms per frame", (s as f32/p as f32)* 100f32, s, p, average_time);
    Ok(())
}
