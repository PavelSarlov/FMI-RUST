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
        let old_head = self.head.take();
        self.head = Some(Box::new(Node { value, next: old_head }));

        // match self.head.take() {
        //     None => self.head = Some(Box::new(Node { value, next: None })),
        //     Some(old_head) => self.head = Some(Box::new(Node { value, next: Some(old_head) })),
        // }
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.value)

        // head.map(|node| {
        //     self.head = node.next;
        //     node.value
        // })
        
        // if let Some(node) = head {
        //     self.head = node.next;
        //     Some(node.value)
        // }
        // else {
        //     None
        // }

        // match head {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.value)
        //     }
        // }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    pub fn iter(&self) -> Iter<T> {
        let first = self.head.as_ref().map(Box::as_ref);

        Iter { current: first }
    }
    
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let first = self.head.as_mut().map(Box::as_mut);

        IterMut { current: first }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self)  {
        while let Some(_) = self.pop() {
            // DROP!
        } 
    }
}

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take()?;
        self.current = current.next.as_ref().map(Box::as_ref);

        Some(&current.value)
    }
}

pub struct IterMut<'a, T> {
    current: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take()?;
        self.current = current.next.as_mut().map(Box::as_mut);

        Some(&mut current.value)
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

pub struct IntoIter<T> { list: List<T> }

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    } 
}

