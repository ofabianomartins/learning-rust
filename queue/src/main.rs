use std::{borrow::BorrowMut, mem};

type Link = Option<Box<Node>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Node { 
    pub value: i32,
    pub next: Link,
}

pub struct Queue {
    head: Link
}

impl Queue {
    pub fn new() -> Self {
        Queue { head: None }
    }

    pub fn push(&mut self, value: i32) {
        // Starts a new node
        let new_node = Box::new(Node { value: value, next: None });
        // Take head reference
        if let Some(head) = &mut self.head {
            // borrow reference to a clean code
            let mut start = head;
            loop {
                // if has next node point to it 
                if let Some(_) = &start.next {
                    start = (start.next.as_mut().unwrap()).borrow_mut()
                } else {
                    // if not break the loop
                    break
                }
            }
            start.next = Some(new_node)
        } else {
            self.head = Some(new_node)
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head.take(), None) {
            None => None, 
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_left() {
        let mut queue = Queue::new();

        queue.push(1);
        queue.push(3);
        queue.push(4);
        queue.push(5);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), None);
    }
}
