type Link = Option<Box<Node>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Node { 
    pub value: i32,
    pub next: Link,
}

pub struct Stack {
    head: Link
}

impl Stack {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node { 
            value: value, 
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
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
    fn insert_two_options() {
        let mut queue = Stack::new();

        queue.push(1);
        queue.push(3);

        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), None);
    }
}
