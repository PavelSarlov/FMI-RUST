use linked_lists::mutable_stack::List;

#[test]
fn test_empty_pop() {
    let mut list = List::<u32>::new();
    assert_eq!(list.pop(), None);
}

#[test]
fn test_push_pop() {
    let mut list = List::new();

    list.push(1);
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);

    list.push(1);
    list.push(2);
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
    assert_eq!(list.pop(), None);
}

#[test]
fn test_non_copy_values() {
    let mut list = List::new();
    list.push(String::from("one"));
    list.push(String::from("two"));

    assert_eq!(list.pop(), Some(String::from("two")));
}

#[test]
fn test_peek() {
    let mut list = List::new();

    assert_eq!(list.peek(), None);

    list.push(1);
    assert_eq!(list.peek(), Some(&1));

    list.push(2);
    assert_eq!(list.peek(), Some(&2));
    *list.peek_mut().unwrap() = 42;
    assert_eq!(list.peek(), Some(&42));

    list.pop();
    list.pop();
    assert_eq!(list.peek(), None);

    list.peek_mut().map(|v| *v = 42);
    assert_eq!(list.peek(), None);
}

#[test]
fn test_into_iterator() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    
    assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![3, 2, 1]);
}

#[test]
fn test_iterator() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    
    assert_eq!(list.iter().collect::<Vec<_>>(), vec![&3, &2, &1]);
    assert_eq!(list.peek(), Some(&3));
}

#[test]
fn test_iterator_mut() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    for item in list.iter_mut() {
        *item *= 2;
    }

    assert_eq!(list.iter().collect::<Vec<_>>(), vec![&6, &4, &2]);
    assert_eq!(list.peek(), Some(&6));
}
