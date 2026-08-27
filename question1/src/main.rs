
#[derive(Clone, PartialEq, Debug)]
enum Node {
    Elem(i32, Box<Node>),
    Nil
}

fn insert(root: &mut Node, elem: i32) {
    if let Node::Elem(_, child) = root {
        insert(child, elem);
    } else if let Node::Nil = root {
        *root = Node::Elem(elem, Box::new(Node::Nil));
    }
}

fn remove(root: &mut Node) {
    if let Node::Elem(_, child) = root {
        if let Node::Nil = **child {
            *root = Node::Nil;
        } else {
            remove(child);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;
    use crate::insert;
    use crate::remove;

    #[test]
    fn test_insert_empty_list() {
        let mut root: Node = Node::Nil;
        insert(&mut root, 10);
        assert_eq!(root, Node::Elem(10, Box::new(Node::Nil)));
    }

    #[test]
    fn test_insert_one_value_list() {
        let mut root: Node = Node::Nil;
        insert(&mut root, 10);
        insert(&mut root, 12);
        assert_eq!(
            root,
            Node::Elem(10, Box::new(Node::Elem(12, Box::new(Node::Nil))))
        );
    }

    #[test]
    fn test_remove_from_empty() {
        let mut root: Node = Node::Nil;
        remove(&mut root);
        assert_eq!(root, Node::Nil);
    }

    #[test]
    fn test_insert_one_value_list_and_remove() {
        let mut root: Node = Node::Nil;
        insert(&mut root, 10);
        insert(&mut root, 12);
        remove(&mut root);
        assert_eq!(
            root,
            Node::Elem(10, Box::new(Node::Nil))
        );
    }

}



