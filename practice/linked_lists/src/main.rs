use linked_lists::mutable_stack::List;

fn main() {
    let mut list = List::new();

    for i in 0..1000000 {
        list.push(i);
    }

    println!("{:?}", list.peek());
}
