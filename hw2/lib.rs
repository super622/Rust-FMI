use std::ops::{Add,Mul};

#[derive(Debug)]
pub struct Matrix<T: Clone> {
    vec: Vec<Cell<T>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl<T: Clone> Matrix<T> {
    pub fn new(data: &[T; 4]) -> Matrix<T> {
        let mut matrix: Matrix<T> = Matrix{vec: Vec::new()};
        for i in 0..4{
            matrix.vec.push(Cell(data[i].clone()));
        }

        matrix
    }

    pub fn by_row(&self) -> Vec<Cell<T>> {
        self.vec.clone()
    }

    pub fn by_col(&self) -> Vec<Cell<T>> {
        let mut vector:Vec<Cell<T>> = Vec::new();

        vector.push(self.vec[0].clone());
        vector.push(self.vec[2].clone());
        vector.push(self.vec[1].clone());
        vector.push(self.vec[3].clone());
        
        vector
    }
}

impl Add<Cell<String>> for Cell<i32>{
    type Output = Cell<String>;

    fn add(self,other:Cell<String>) -> Self::Output{
        let number = self.0;
        let string = other.0;

        let mut result: Cell<String> = Cell(String::from(""));
        if number >= 0 {
            result = Cell(format!("{} {}",number.to_string(),string));
        }
        else{
            result = Cell(format!("{} {}",string.chars().rev().collect::<String>(),number.abs().to_string()));
        }

        result
    }
}

impl Mul<Cell<String>> for Cell<i32>{
    type Output = Cell<String>;

    fn mul(self, other: Cell<String>) -> Self::Output{
         let mut number = self.0;
         let mut string = other.0;
         
         if number == 0 {
            return Cell(String::from(""));
         }
         else if number < 0 {
            string = string.chars().rev().collect();
            number = number.abs();
         }

        Cell(format!("{}", string.repeat(number as usize)))
    }
}

impl Add<Matrix<String>> for Matrix<i32> {
    type Output = Matrix<String>;

    fn add(self, other:Matrix<String>) -> Self::Output{
        let mut result: Matrix<String> = Matrix{vec: Vec::new()};

        for i in 0..4 {
            result.vec.push(self.vec[i].clone() + other.vec[i].clone());
        }

        result
    }
}

//vec[0] = 00 vec[1] = 01
//vec[2] = 10 vec[3] = 11

impl Mul<Matrix<String>> for Matrix<i32> {
    type Output = String;

    fn mul(self, other: Matrix<String>) -> Self::Output{
        let mut string: String = String::new();
        let space: String = String::from(" ");
        string += &(self.vec[0].clone()*other.vec[0].clone()).0;
        string += &space;
        string += &(self.vec[1].clone()*other.vec[2].clone()).0;
        string += &space;
        string += &(self.vec[2].clone()*other.vec[1].clone()).0;
        string += &space;
        string += &(self.vec[3].clone()*other.vec[3].clone()).0;
        
        string
    }
}