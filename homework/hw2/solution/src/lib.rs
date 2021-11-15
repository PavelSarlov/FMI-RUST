#[derive(Debug)]
pub struct Matrix<T: Clone> {
    // Каквито данни ви вършат работа
    pub m: [[Cell<T>; 2]; 2] 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl<T: Clone> Matrix<T> {
    /// Данните се очаква да бъдат подадени със статичен масив -- вижте по-долу за примери за
    /// конструиране. Какви може да са елементите? Ще тестваме само с два типа: String и i32.
    ///
    /// Очаква се да бъдат подадени по редове, от ляво надясно. Тоест, ако подадем като вход списък
    /// с елементи: 1, 2, 3, 4, се очаква конструираната матрица:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Забележете, че подаваме като вход някакъв slice -- reference тип. Не очакваме матрицата да
    /// държи reference, клонирайте си данните, за да имате ownership.
    ///
    pub fn new(data: &[T; 4]) -> Matrix<T> {
        Matrix::<T> {
            m: [[Cell(data[0].clone()), Cell(data[1].clone())], 
                [Cell(data[2].clone()), Cell(data[3].clone())]]
        }
    }

    /// Връща вектор, който съдържа в себе си всички 4 елемента на матрицата, наредени по редове,
    /// от ляво надясно и от горе надолу, обвити в `Cell`. Тоест, ако матрицата изглежда така:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Очакваме `.by_row` да върне елементите в ред: 1, 2, 3, 4
    ///
    pub fn by_row(&self) -> Vec<Cell<T>> {
        let mut vec = Vec::with_capacity(4);

        for i in 0..2 {
            for j in 0..2 {
                vec.push(self.m[i][j].clone());
            }
        }
        vec
    }

    /// Връща вектор, който съдържа в себе си всички 4 елемента на матрицата, наредени по колони,
    /// от горе надолу и от ляво надясно, Обвити в `Cell`. Тоест, ако матрицата изглежда така:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Очакваме `.by_col` да върне елементите в ред: 1, 3, 2, 4
    ///
    pub fn by_col(&self) -> Vec<Cell<T>> {
        let mut vec = Vec::with_capacity(4);

        for j in 0..2 {
            for i in 0..2 {
                vec.push(self.m[i][j].clone());
            }
        }
        vec
    }
}


use std::ops::{Add, Mul};

impl Add<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn add(self, other: Cell<String>) -> Self::Output {
        if self.0 >= 0 {
            Cell(format!("{} {}", self.0, other.0))
        }
        else {
            Cell(format!("{} {}", other.0.chars().rev().collect::<String>(), -self.0))
        }
    } 
}

impl Mul<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn mul(self, other: Cell<String>) -> Self::Output {
        if self.0 >= 0 {
            Cell(other.0.repeat(self.0 as usize))
        }
        else {
            Cell(other.0.chars().rev().collect::<String>().repeat(-self.0 as usize))
        }
    }
}

impl Add<Matrix<String>> for Matrix<i32> {
    type Output = Matrix<String>;

    fn add(self, other: Matrix<String>) -> Self::Output {
        let mut result = Matrix::new(&[String::from(""), String::from(""), String::from(""), String::from("")]);

        for i in 0..2 {
            for j in 0..2 {
                result.m[i][j] = self.m[i][j].clone() + other.m[i][j].clone();
            }
        }
        result
    }
}

impl Mul<Matrix<String>> for Matrix<i32> {
    type Output = String;

    fn mul(self, other: Matrix<String>) -> Self::Output {
        let mut result = String::new();

        for i in 0..2 {
            for j in 0..2 {
                result = format!("{} {}", result, (self.m[i][j].clone() * other.m[j][i].clone()).0);
            }
        }
        result.trim().to_string()
    }
}

#[cfg(test)]
mod custom_tests {
    use super::*;

    fn string_cell_vec(s1: &str, s2: &str, s3: &str, s4: &str) -> Vec<Cell<String>> {
        [s1, s2, s3, s4].into_iter().map(String::from).map(Cell).collect::<Vec<Cell<String>>>()
    }

    #[test]
    pub fn custom_test() {
        let mat = Matrix::<i32>::new(&[1,2,3,4]);

        assert_eq!(mat.by_row()[0], Cell(1));
        assert_eq!(mat.by_col()[1], Cell(3));
        
        assert_eq!(Cell(22) + Cell(String::from("years ago")), Cell(String::from("22 years ago")));
        assert_eq!(Cell(0) + Cell(String::from("expectation")), Cell(String::from("0 expectation")));   
        assert_eq!(Cell(-4) + Cell(String::from("xirtam")), Cell(String::from("matrix 4")));
        
        assert_eq!(Cell(3) * Cell(String::from("woah!")), Cell(String::from("woah!woah!woah!")));
        assert_eq!(Cell(0) * Cell(String::from("woah?")), Cell(String::from("")));
        assert_eq!(Cell(-3) * Cell(String::from(",regdab")), Cell(String::from("badger,badger,badger,")));

        let mat1 = Matrix::new(&[String::from("this"), String::from("is"), 
                                String::from("a"), String::from("matrix")]);
        assert_eq!((mat + mat1).by_row(), string_cell_vec("1 this", "2 is", "3 a", "4 matrix"));

        let mat1 = Matrix::new(&[String::from("this"), String::from("is"), 
                                String::from("a"), String::from("matrix")]);
        let mat2 = Matrix::new(&[-1,0,2,1]);
        assert_eq!((mat2 + mat1).by_row(), string_cell_vec("siht 1", "0 is", "2 a", "1 matrix"));

        let mat1 = Matrix::new(&[String::from("this"), String::from("is"), 
                                String::from("a"), String::from("matrix")]);
        let mat = Matrix::<i32>::new(&[1,2,3,4]);
        assert_eq!(mat * mat1, String::from("this aa isisis matrixmatrixmatrixmatrix"));

        let mat1 = Matrix::new(&[String::from("this"), String::from("is"), 
                                String::from("a"), String::from("matrix")]);
        let mat2 = Matrix::new(&[-1,0,2,1]);
        assert_eq!(mat2 * mat1, String::from("siht  isis matrix"));
    }
}
