use matrix_iteration::*;

#[test]
fn test_iteration_0() {
    let data = [1, 2,
                3, 4];
    let matrix = Matrix::new(2, 2, &data);

    assert_eq!(matrix.by_row().collect::<Vec<_>>(), vec![&1, &2, &3, &4]);
    assert_eq!(matrix.by_col().collect::<Vec<_>>(), vec![&1, &3, &2, &4]);
}
