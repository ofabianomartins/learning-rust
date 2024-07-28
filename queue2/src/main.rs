use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Node { 
    pub value: i32,
    pub next: Link,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self { value, next: None }
    }
}

pub struct Queue {
    head: Link,
    tail: Link,
    size: usize
}

impl Queue {
    pub fn new() -> Self {
        Queue { head: None, tail: None, size: 0 }
    }

    pub fn push(&mut self, value: i32) {
        let node = Rc::new(RefCell::new(Node::new(value)));
        if let Some(prev_tail) = self.tail.take() {
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|prev_head| {
            self.size -= 1;
            match prev_head.borrow_mut().next.take() {
                Some(node) => {
                    self.head = Some(node);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(prev_head).ok().unwrap().into_inner().value
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
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
