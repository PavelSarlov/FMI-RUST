#[derive(Debug)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    /// Конструира матрица с брой редове `rows` и брой колони `cols`. Данните би трябвало да бъдат
    /// поне `rows * cols`, но няма да тестваме с масиви, които са твърде малки или твърде големи.
    /// Ако искате да panic-нете при невалиден вход, ваш избор.
    ///
    /// Данните ще са наредени в плосък масив по редове. Очаква се да държите ownership над тях,
    /// така че си ги клонирайте и си ги наредете както ви е удобно.
    ///
    pub fn new(rows: usize, cols: usize, data: &[T]) -> Self {
        assert!(rows * cols == data.len());

        Matrix {
            rows,
            cols,
            data: data.clone()
                .iter()
                .map(|x| x.to_owned())
                .collect()
        }
    }

    pub fn by_row(&self) -> RowIter<T> {
        RowIter {
            curr: 0,
            matrix: self,
        }
    }

    pub fn by_col(&self) -> ColIter<T> {
        ColIter {
            curr: 0,
            matrix: self,
        }
    }
}

pub struct RowIter<'a, T> {
    // каквото ви трябва
    curr: usize,
    matrix: &'a Matrix<T>,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.curr < self.matrix.rows * self.matrix.cols {
            true => Some(&self.matrix.data[self.curr]),
            _ => None,
        };
        self.curr += 1;
        result
    }
}

pub struct ColIter<'a, T> {
    // каквото ви трябва
    curr: usize,
    matrix: &'a Matrix<T>,
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let cell = (self.curr % self.matrix.rows) * self.matrix.cols + self.curr / self.matrix.rows;

        let result = match self.curr < self.matrix.rows * self.matrix.cols {
            true => Some(&self.matrix.data[cell]),
            _ => None,
        };
        self.curr += 1;
        result
    }
}

#[cfg(test)]
mod mytests {
    use super::*;

    #[test]
    fn test_iter() {
        let mat = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);

        assert_eq!(mat.by_row().collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6]);
        assert_eq!(mat.by_col().collect::<Vec<_>>(), vec![&1, &4, &2, &5, &3, &6]);

        assert_eq!(mat.by_row().map(|x| x*2).collect::<Vec<_>>(), vec![2, 4, 6, 8, 10, 12]);
    }

    #[test]
    fn test_next() {
        let mat = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);

        let mut it = mat.by_row();
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&4));
        assert_eq!(it.next(), Some(&5));
        assert_eq!(it.next(), Some(&6));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let mut it = mat.by_col();
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&4));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&5));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), Some(&6));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_invalid_data_size() {
        assert!(std::panic::catch_unwind(|| Matrix::new(1,1,&[1,2])).is_err());
    }
}
