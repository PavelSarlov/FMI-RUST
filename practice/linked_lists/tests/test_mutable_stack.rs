use linked_lists::mutable_stack::List;

#[test]
fn test_push_pop() {
    let mut list = List::new();

    list.push(1);
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);

    // list.push(1);
    // list.push(2);
    // assert_eq!(list.pop(), Some(2));
    // assert_eq!(list.pop(), Some(1));
    // assert_eq!(list.pop(), None);
    // assert_eq!(list.pop(), None);
}
