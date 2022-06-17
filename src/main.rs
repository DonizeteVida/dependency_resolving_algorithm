use std::{cell::RefCell, collections::HashSet, rc::Rc};

struct Node {
    name: &'static str,
    edges: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: &'static str) -> Self {
        Node {
            name,
            edges: vec![],
        }
    }

    fn add_edge(&mut self, node: &Rc<RefCell<Node>>) {
        self.edges.push(Rc::clone(node));
    }
}

fn dep_resolve(
    node: &Rc<RefCell<Node>>,
    resolved: &mut HashSet<&str>,
    unresolved: &mut HashSet<&str>,
) {
    let borrow = node.borrow();
    let name = borrow.name;

    unresolved.insert(name);
    println!("{}", name);

    for edge in borrow.edges.iter() {
        let edge_name = edge.borrow().name;

        if !resolved.contains(edge_name) {
            if unresolved.contains(edge_name) {
                panic!("Circular reference detected: {} -> {}", name, edge_name);
            }
            dep_resolve(edge, resolved, unresolved);
        } else {
            println!("already resolved {}", name);
        }
    }

    resolved.insert(name);
    unresolved.remove(name);

    println!("Resolved: {}", name);
}

fn main() {
    let mut resolved = HashSet::<&str>::new();
    let mut unresolved = HashSet::<&str>::new();

    let a = Rc::new(RefCell::new(Node::new("a")));
    let b = Rc::new(RefCell::new(Node::new("b")));
    let c = Rc::new(RefCell::new(Node::new("c")));
    let d = Rc::new(RefCell::new(Node::new("d")));
    let e = Rc::new(RefCell::new(Node::new("e")));

    a.borrow_mut().add_edge(&b);
    a.borrow_mut().add_edge(&d);
    b.borrow_mut().add_edge(&c);
    b.borrow_mut().add_edge(&e);
    c.borrow_mut().add_edge(&d);
    c.borrow_mut().add_edge(&e);
    d.borrow_mut().add_edge(&b);

    dep_resolve(&a, &mut resolved, &mut unresolved);
}
