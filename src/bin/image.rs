use rand;
use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Instant;

const THREADS: usize = 10;

fn main() {
    let contents = fs::read_to_string("out.txt").expect("Should have been able to read the file");
    let instructions = contents
        .split("\n")
        .map(|s| s.to_string() + "\n")
        .collect::<Vec<String>>()
        .clone();

    let per_thread = instructions.len() / THREADS;
    let mut splitted_instr = vec![];
    for i in 0..THREADS {
        let a = instructions
            .iter()
            .skip(i * per_thread)
            .take(per_thread)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        splitted_instr.push(a);
    }

    let start = Instant::now();

    let mut handles = vec![];

    for i in 0..THREADS {
        let splitted_instr = splitted_instr[i].clone();
        let start = start.clone();
        let handle = thread::spawn(move || {
            let mut stream = match TcpStream::connect("table.c3pixelflut.de:1337") {
                Ok(stream) => stream,
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            };

            let instructions = splitted_instr.join("\n");
            loop {
                let x = 1800.0 + 1800.0 * (start.elapsed().as_millis() as f32 / 10000.0).sin();
                let y = 200.0 + 200.0 * (start.elapsed().as_millis() as f32 / 1000.0).cos();

                stream
                    .write(format!("OFFSET {} {}\n", x as usize, y as usize).as_bytes())
                    .unwrap();

                stream.write(instructions.as_bytes()).unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
