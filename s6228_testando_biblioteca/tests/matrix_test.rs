use s6228_testando_biblioteca::Matrix;

#[test]
fn test_add_matrices(){
    let matrix_a = Matrix::new(2, 3, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let matrix_b = Matrix::new(2, 3, vec![vec![7.0, 8.0, 9.0], vec![10.0, 11.0, 12.0]]);
    let result = matrix_a.add(&matrix_b).unwrap();
    assert_eq!(
        result.data,
        vec![vec![8.0, 10.0, 12.0], vec![14.0, 16.0,18.0]]
    );
}

#[test]
fn test_multiply_matrices(){
    let matrix_a = Matrix::new(2, 3, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
    let matrix_b = Matrix::new(3, 2, vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]);

    //c11, c12, c21, c22 => c11= 1*7 + 2*9 + 3*11 = 58
   //                      c12= 1*8 + 2*10 + 3*12 = 64
   //                      c21= 4*7 + 5*9 + 6*11 = 139
   //                      c22= 4*8 + 5*10 + 6*12 = 154                
   // 
   let result = matrix_a.multipy(&matrix_b).unwrap();
   assert_eq!(
       result.data,
       vec![vec![58.0, 64.0], vec![139.0, 154.0]]
   );


}