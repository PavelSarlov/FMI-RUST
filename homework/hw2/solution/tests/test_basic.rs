use solution::*;

fn string_cell_vec(s1: &str, s2: &str, s3: &str, s4: &str) -> Vec<Cell<String>> {
    [s1, s2, s3, s4].into_iter().map(String::from).map(Cell).collect::<Vec<Cell<String>>>()
}

#[test]
fn test_basic() {
    // cells addition
    let strCell = Cell(String::from("text"));
    assert_eq!((Cell(4) + Cell(String::from("badger"))).0, String::from("4 badger"));
    assert_eq!(Cell(22) + Cell(String::from("years ago")), Cell(String::from("22 years ago")));
    assert_eq!(Cell(0) + Cell(String::from("expectation")), Cell(String::from("0 expectation")));
    assert_eq!(Cell(-4) + Cell(String::from("xirtam")), Cell(String::from("matrix 4")));
    assert_eq!(Cell(-1) + strCell.clone(), Cell(String::from("txet 1")));
    
    // standart comparisons
    assert_eq!(Cell(String::from("text")), strCell);
    assert_ne!(Cell(0), Cell(1));
    assert_ne!(Cell(String::from("")), Cell(String::from("text")));
    
    // cells multipliction
    assert_eq!(Cell(3) * Cell(String::from("woah!")), Cell(String::from("woah!woah!woah!")));
    assert_eq!(Cell(0) * Cell(String::from("woah?")), Cell(String::from("")));
    assert_eq!(Cell(-3) * Cell(String::from(",regdab")), Cell(String::from("badger,badger,badger,")));
    assert_eq!((Cell(2) * Cell(String::from("mushroom"))).0, String::from("mushroommushroom"));

    // matrix operations
    let matrix1 = Matrix::new(&[1, 2, 3, 4]);
    let matrix2 = Matrix::new(&[
        String::from("one"), String::from("two"),
        String::from("three"), String::from("four")
    ]);

    assert_eq!(matrix1.by_row()[0], Cell(1));
    assert_eq!(matrix1.by_row()[1], Cell(2));
    assert_eq!(matrix1.by_row()[2], Cell(3));
    assert_eq!(matrix1.by_col()[1], Cell(3));
    assert_eq!(matrix1.by_col()[2], Cell(2));

    assert_eq!(
        (matrix1 + matrix2).by_row(),
        string_cell_vec("1 one", "2 two", "3 three", "4 four")
    );
    assert_eq!(
        (Matrix::new(&[1, 2, -3, 4]) + Matrix::new(&[
                                        String::from("one"), String::from("two"),
                                        String::from("abac"), String::from("four")
                                    ])).by_row(),
        string_cell_vec("1 one", "2 two", "caba 3", "4 four")
    );
    
    let matrix1 = Matrix::new(&[1, 2, 1, 1]);
    let matrix2 = Matrix::new(&[
        String::from("one"), String::from("two"),
        String::from("three"), String::from("four")
    ]);
    assert_eq!(matrix1 * matrix2, String::from("one threethree two four"));
    assert_eq!(
        (Matrix::new(&[1, 2, -3, 4]) * Matrix::new(&[
                                        String::from("one"), String::from("two"),
                                        String::from("abac"), String::from("four")
                                    ])),
        String::from("one abacabac owtowtowt fourfourfourfour")
    );
    assert_eq!(
        (Matrix::new(&[1, 0, -3, 4]) * Matrix::new(&[
                                        String::from("one"), String::from("two"),
                                        String::from("abac"), String::from("four")
                                    ])),
        String::from("one  owtowtowt fourfourfourfour")
    );
}
