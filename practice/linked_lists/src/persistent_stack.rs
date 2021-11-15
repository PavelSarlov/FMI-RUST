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
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }

}
