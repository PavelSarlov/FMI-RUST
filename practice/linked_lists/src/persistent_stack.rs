#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::rc::Rc;

pub struct List<T> {
    head:Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn append(&self, value: T) -> List<T> {
        let new_head = Rc::new(Node {
            value,
            next: self.head.as_ref().map(|node| Rc::clone(node)),
            // next: self.head.as_ref().map(Rc::clone),
            // next: self.head.clone()
        });

        List { head: Some(new_head) }
    }

    pub fn tail(&self) -> List<T> {
        let new_head = self.head.as_ref().and_then(|node| node.next.clone());

        // let new_head = match &self.head {
        //     Some(node) => {
        //         match &node.next {
        //             Some(next_node) => Some(next_node.clone()),
        //             None => None
        //         }
        //     },
        //     None => None,
        // }; 

        List { head: new_head }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { current: self.head.as_ref().map(Rc::as_ref) }
    }
}

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    } 
}
