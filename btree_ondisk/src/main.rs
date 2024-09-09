pub mod btree;
pub mod node;
pub mod pager;

use std::fs::File;
use std::io;
use std::io::Write;

use btree::Btree;
use pager::Pager;

fn main() {
    let file = File::create("main_foo_11.txt").expect("file not found");
    let pager = Pager::new(file);
    let mut btree: Btree = Btree::new(pager);

    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let mut buf = String::new();

    loop {
        print!("btree> ");
        stdout.flush().expect("flush stdout");
        buf.truncate(0);
        let n = stdin.read_line(&mut buf).expect("read line");
        let line = &buf[..n];
        match line.trim() {
            "quit" | "exit" => break,
            "" => continue,
            line => {
                let len = line.len();

                match &line[0..6] {
                    "insert" => { 
                        let value = &line[7..len];
                        println!("Inserting {} ", value);
                        btree.push(value.parse::<u8>().unwrap())
                    },
                    value => { println!("command not found {} ", value); }
                }
                btree.print_tree();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use btree::tests;
}
