use rand;
use std::io::Write;
use std::net::TcpStream;
use std::thread;

pub fn hue_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn main() {
    let mut handles = vec![];
    for i in 0..8 {
        let k = i.clone();
        let handle =
            thread::spawn(
                move || match TcpStream::connect("table.apokalypse.email:1337") {
                    Ok(mut stream) => {
                        let mut offset_x = 0;
                        let mut offset_y = 0;
                        match i % 4 {
                            0 => {
                                // Go from left to right
                                loop {
                                    let offset = format!("OFFSET {} {}\n", offset_x, 0);
                                    stream.write(offset.as_bytes()).unwrap();

                                    let color =
                                        hue_to_rgb((offset_x as f64 / 3840.0) * 360.0, 1.0, 0.5);
                                    let width = 5;
                                    for x in 0..1 {
                                        for y in 0..1080 {
                                            let msg = format!(
                                                "PX {} {} {:02x}{:02x}{:02x}\n",
                                                (k * width) + x,
                                                y,
                                                color.0,
                                                color.1,
                                                color.2
                                            );
                                            // println!("{}", msg);
                                            stream.write(msg.as_bytes()).unwrap();
                                        }
                                    }
                                    offset_x += 1;
                                    offset_x = offset_x % 3840;
                                }
                            }
                            1 => {
                                // Go from rigth to left
                                loop {
                                    let offset = format!("OFFSET {} {}\n", offset_x, 0);
                                    stream.write(offset.as_bytes()).unwrap();

                                    let color =
                                        hue_to_rgb((offset_x as f64 / 3840.0) * 360.0, 1.0, 0.5);
                                    let width = 5;
                                    for x in 0..1 {
                                        for y in 0..1080 {
                                            let msg = format!(
                                                "PX {} {} {:02x}{:02x}{:02x}\n",
                                                (k * width) + x,
                                                y,
                                                color.0,
                                                color.1,
                                                color.2
                                            );
                                            stream.write(msg.as_bytes()).unwrap();
                                        }
                                    }
                                    offset_x -= 1;
                                    offset_x = (offset_x + 3840) % 3840;
                                }
                            }
                            2 => {
                                // Go from top to bottom
                                loop {
                                    let offset = format!("OFFSET {} {}\n", 0, offset_y);
                                    stream.write(offset.as_bytes()).unwrap();

                                    let color =
                                        hue_to_rgb((offset_y as f64 / 1080.0) * 360.0, 1.0, 0.5);
                                    let height = 5;
                                    for x in 0..3840 {
                                        for y in 0..1 {
                                            let msg = format!(
                                                "PX {} {} {:02x}{:02x}{:02x}\n",
                                                x,
                                                (k * height) + y,
                                                color.0,
                                                color.1,
                                                color.2
                                            );
                                            stream.write(msg.as_bytes()).unwrap();
                                        }
                                    }
                                    offset_y += 1;
                                    offset_y = offset_y % 1080;
                                }
                            }
                            _ => {
                                // Go from bottom to top
                                loop {
                                    let offset = format!("OFFSET {} {}\n", 0, offset_y);
                                    stream.write(offset.as_bytes()).unwrap();

                                    let color =
                                        hue_to_rgb((offset_y as f64 / 1080.0) * 360.0, 1.0, 0.5);
                                    let height = 5;
                                    for x in 0..3840 {
                                        for y in 0..1 {
                                            let msg = format!(
                                                "PX {} {} {:02x}{:02x}{:02x}\n",
                                                x,
                                                (k * height) + y,
                                                color.0,
                                                color.1,
                                                color.2
                                            );
                                            stream.write(msg.as_bytes()).unwrap();
                                        }
                                    }
                                    offset_y -= 1;
                                    offset_y = (offset_y + 1080) % 1080;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to connect: {}", e);
                    }
                },
            );
        handles.push(handle);
    }
    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
