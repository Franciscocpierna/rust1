pub struct Matrix{
   pub rows: usize,
   pub cols: usize,
   pub data: Vec<Vec<f64>>,


}

impl Matrix{
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Self{
        Matrix{rows, cols, data}
    }
    pub fn add(&self, other: &Matrix) ->Result<Matrix, &'static str>{
        if self.rows  != other.rows || self.cols != other.cols{
            return Err("As dimensões das matrizes não são compativeis");

        }
        let mut result_data = Vec::with_capacity(self.rows);
        for i in 0..self.rows{
            let mut row = Vec::with_capacity(self.cols);
            for j in 0..self.cols{
                row.push(self.data[i][j] + other.data[i][j]);
            }
            result_data.push(row);
        }
        Ok(Matrix::new(self.rows, self.cols, result_data))
    }
}