use solution::*;

fn main() {
    let mat = Matrix::<u32>::new(&[1,2,3,4]);

    assert_eq!(mat.by_row()[0], Cell(1));
    assert_eq!(mat.by_col()[1], Cell(3));
    
    assert_eq!(Cell(22) + Cell(String::from("years ago")), Cell(String::from("22 years ago")));
    assert_eq!(Cell(0) + Cell(String::from("expectation")), Cell(String::from("0 expectation")));   
    assert_eq!(Cell(-4) + Cell(String::from("xirtam")), Cell(String::from("matrix 4")));
    
    assert_eq!(Cell(3) * Cell(String::from("woah!")), Cell(String::from("woah!woah!woah!")));
    assert_eq!(Cell(0) * Cell(String::from("woah?")), Cell(String::from("")));
    assert_eq!(Cell(-3) * Cell(String::from(",regdab")), Cell(String::from("badger,badger,badger,")))
}
