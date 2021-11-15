#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        self.head = Some(Box::new(Node {
            value,
            next: None
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => Some(node.value),
        }
    }
}
