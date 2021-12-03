use matrix_iteration::*;

fn main() {
    let m = Matrix::<u32>::new(3,2,&[1,2,3,4,5,6]);
    println!("{:?}", m);
    
    println!("{:?}", m.by_row().collect::<Vec<_>>());
    println!("{:?}", m.by_col().collect::<Vec<_>>());
}
