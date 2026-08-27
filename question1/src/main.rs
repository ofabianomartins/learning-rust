
#[derive(Clone, PartialEq, Debug)]
enum Node {
    Elem(i32, Box<Node>),
    Nil
}

fn new() -> Box<Node> {
    return Box::new(Node::Nil);
}

fn insert(root: &mut Box<Node>, elem: i32) {
    if let Node::Elem(_, child) = root.as_mut() {
        insert(child, elem);
    } else {
        *root = Box::new(Node::Elem(elem, Box::new(Node::Nil)));
    }
}

fn remove(root: &mut Box<Node>) {
    // 1. Caso base: lista vazia
    if let Node::Nil = root.as_ref() {
        return;
    }

    // 2. Verifica se o filho imediato é o último nó (Nil)
    let is_child_nil = match root.as_ref() {
        Node::Elem(_, child) => matches!(child.as_ref(), Node::Nil),
        Node::Nil => false,
    };

    if is_child_nil {
        // Transforma o nó em Nil
        *root = Box::new(Node::Nil);
    } else if let Node::Elem(_, child) = root.as_mut() {
        // Caso contrário, continua descendo recursivamente
        remove(child);
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;
    use crate::new;
    use crate::insert;
    use crate::remove;

    #[test]
    fn test_insert_empty_list() {
        let mut root: Box<Node> = new();
        insert(&mut root, 10);
        assert_eq!(root, Box::new(Node::Elem(10, Box::new(Node::Nil))));
    }

    #[test]
    fn test_insert_one_value_list() {
        let mut root: Box<Node> = new();
        insert(&mut root, 10);
        insert(&mut root, 12);
        assert_eq!(
            root,
            Box::new(Node::Elem(10, Box::new(Node::Elem(12, Box::new(Node::Nil)))))
        );
    }

    #[test]
    fn test_insert_two_value_list() {
        let mut root: Box<Node> = new();
        insert(&mut root, 10);
        insert(&mut root, 12);
        insert(&mut root, 14);
        assert_eq!(
            root,
            Box::new(Node::Elem(10, Box::new(Node::Elem(12, Box::new(Node::Elem(14, Box::new(Node::Nil)))))))
        );
    }

    #[test]
    fn test_remove_from_empty() {
        let mut root: Box<Node> = new();
        remove(&mut root);
        assert_eq!(root, Box::new(Node::Nil));
    }

    #[test]
    fn test_insert_one_value_list_and_remove() {
        let mut root: Box<Node> = new();
        insert(&mut root, 10);
        insert(&mut root, 12);
        remove(&mut root);
        assert_eq!(
            root,
            Box::new(Node::Elem(10, Box::new(Node::Nil)))
        );
    }

}



