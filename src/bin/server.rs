use pixel_canvas::{input::MouseState, Canvas, Color};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

#[derive(Debug, Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    let mut mem: [Pixel; 512 * 512] = [Pixel { r: 0, g: 10, b: 50 }; 512 * 512];

    std::thread::spawn(|| {
        // accept connections and process them, spawning a new thread for each one
        println!("Server listening on port 3333");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        // connection succeeded
                        handle_client(stream)
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
        // close the socket server
        drop(listener);
    });

    let canvas = Canvas::new(512, 512)
        .title("Tile")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    // The canvas will render for you at up to 60fps.
    canvas.render(move |mouse, image| {
        // Modify the `image` based on your state.
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let pix = mem[x + y * 512];
                *pixel = Color {
                    r: pix.r,
                    g: pix.g,
                    b: pix.b,
                };
            }
        }
    });
}
