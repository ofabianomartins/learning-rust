use std::fmt;

#[derive(Clone, PartialEq, Debug)]
enum Node {
    Elem(i32, Box<Node>),
    Nil
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Node::Elem(elem, child) = self {
            write!(f, "{}: {}", elem, child)
        } else {
            write!(f, "Nil")
        }
    }
}


fn new() -> Box<Node> {
    return Box::new(Node::Nil);
}

fn push(root: &mut Box<Node>, elem: i32) {
    if let Node::Elem(_, _) = root.as_mut() {
        *root = Box::new(Node::Elem(elem, Box::new(*root.clone())));
    } else {
        *root = Box::new(Node::Elem(elem, Box::new(Node::Nil)));
    }
}

fn pop(mut root: &mut Box<Node>) -> Option<i32> {
    let old_root = std::mem::replace(&mut **root, Node::Nil);
    
    if let Node::Elem(elem, child) = old_root {
        *root = child;
        Some(elem)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;
    use crate::new;
    use crate::push;
    use crate::pop;

    #[test]
    fn test_push_empty_list() {
        let mut root: Box<Node> = new();
        push(&mut root, 10);
        assert_eq!(root, Box::new(Node::Elem(10, Box::new(Node::Nil))));
    }

    #[test]
    fn test_push_one_value_list() {
        let mut root: Box<Node> = new();
        push(&mut root, 10);
        push(&mut root, 12);
        assert_eq!(
            root,
            Box::new(Node::Elem(12, Box::new(Node::Elem(10, Box::new(Node::Nil)))))
        );
    }

    #[test]
    fn test_push_two_value_list() {
        let mut root: Box<Node> = new();
        push(&mut root, 10);
        push(&mut root, 12);
        push(&mut root, 14);
        assert_eq!(
            root,
            Box::new(Node::Elem(14, Box::new(Node::Elem(12, Box::new(Node::Elem(10, Box::new(Node::Nil)))))))
        );
    }

    #[test]
    fn test_pop_from_empty() {
        let mut root: Box<Node> = new();
        assert_eq!(pop(&mut root), None);
        assert_eq!(root, Box::new(Node::Nil));
    }

    #[test]
    fn test_push_one_value_list_and_pop() {
        let mut root: Box<Node> = new();
        push(&mut root, 10);
        push(&mut root, 12);
        push(&mut root, 14);
        assert_eq!(pop(&mut root), Some(14));
        assert_eq!(
            root,
            Box::new(Node::Elem(12, Box::new(Node::Elem(10, Box::new(Node::Nil)))))
        );
    }

}



