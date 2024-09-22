pub mod pager;
pub mod buffer;
pub mod disk_string_list;

use std::fs::OpenOptions;
use std::io;
use std::io::Write;

use pager::Pager;
use buffer::Buffer;
use disk_string_list::DiskStringList;

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("test.txt")
        .unwrap();
    let pager = Pager::new(file);
    let buffer = Buffer::new(pager);
    let mut list = DiskStringList::new(buffer);

    list.load_file();

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
                        println!("Inserting \"{}\"; ", value);
                        list.save_string(value);
                    },
                    "r" => { 
                        let value = &line[2..len];
                        match value.parse::<u32>() {
                            Ok(pos) => {
                                println!("{}) {} ", pos, list.read_string(pos));
                            }
                            Err(e) => {
                                println!("Failed to convert the string to u64: {}", e);
                            }
                        }
                    },
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
