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