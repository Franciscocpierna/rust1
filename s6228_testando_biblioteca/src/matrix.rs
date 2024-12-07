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

    pub fn multipy(&self, other: &Matrix) -> Result<Matrix, &'static str>{
        // 3 x 2  2 x 5 => 3x5 para multiplicação de matrizes
         if self.cols != other.rows{
           return Err("as dimensões das marizes não são compativeis para multiplicação");
         }
         let mut result_data = Vec::with_capacity(self.rows);
         for i in 0..self.rows{
            let mut rows = Vec::with_capacity(self.cols);
            for j in 0..other.cols{
               let mut sum = 0.0;
               for k in 0..self.cols{
                 sum += self.data[i][k] * other.data[k][j];
               }
               rows.push(sum);
            }
            result_data.push(rows);    
        }
       
        Ok(Matrix::new(self.rows, other.cols, result_data))   
    }
    pub fn print(&self){
        for row in &self.data{
            for &element in row{
                print!("{}", element);
            }
            println!();
        }
    }
}