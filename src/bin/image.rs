use rand;
use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::thread;

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

    let mut handles = vec![];

    for i in 0..THREADS {
        let splitted_instr = splitted_instr[i].clone();
        let handle =
            thread::spawn(
                move || match TcpStream::connect("table.apokalypse.email:1337") {
                    Ok(mut stream) => {
                        stream.write("OFFSET 1000 200\n".as_bytes()).unwrap();
                        loop {
                            for instruction in &splitted_instr {
                                stream.write(instruction.as_bytes()).unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                },
            );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
