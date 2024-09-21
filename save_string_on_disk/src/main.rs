pub mod pager;
pub mod disk_string_list;

use std::fs::OpenOptions;
use std::io;
use std::io::Write;

use pager::Pager;
use disk_string_list::DiskStringList;

fn main() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("test.txt")
        .unwrap();
    let pager = Pager::new(file);
    let mut list = DiskStringList::new(pager);

    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut buf = String::new();

    loop {
        print!("string> ");
        stdout.flush().expect("flush stdout");
        buf.truncate(0);
        let n = stdin.read_line(&mut buf).expect("read line");
        let line = &buf[..n];
        match line.trim() {
            "quit" | "exit" => break,
            "" => continue,
            line => {
                let len = line.len();

                match &line[0..1] {
                    "i" => { 
                        let value = &line[2..len];
                        println!("Inserting {} ", value);
                        list.save_string(value);
                    },
                    "r" => { 
                        let value = &line[2..len];
                        match value.parse::<u64>() {
                            Ok(pos) => {
                                let mut buffer = list.read_string(pos);

                                let result = String::from_utf8_lossy(&buffer);
                                println!("String at {} ", result);
                            }
                            Err(e) => {
                                println!("Failed to convert the string to u64: {}", e);
                            }
                        }
                    },
                    "l" => {
                        for pos in 0..list.size {
                            let mut buffer = list.read_string(pos);

                            let result = String::from_utf8_lossy(&buffer);
                            println!("{}: {} ", pos, result);
                        }
                    }
                    value => { println!("command not found {} ", value); }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
