
use s6235_retangulo_testes::Rectangle; // cargo test --test point_test



#[test]
fn test_area_of_rectangle() {
    let rect = Rectangle::new(0.0, 0.0, 4.0, 5.0);
    assert_eq!(rect.area(), 20.0);
}

#[test]
fn test_perimeter_of_rectangle() {
    let rect = Rectangle::new(0.0, 0.0, 4.0, 5.0);
    assert_eq!(rect.perimeter(), 18.0);
}

#[test]
fn test_contains_point_inside_rectangle() {
    let rect = Rectangle::new(0.0, 0.0, 4.0, 5.0);
    assert!(rect.contains_point(2.0, 3.0));
}

#[test]
fn test_contains_point_outside_rectangle() {
    let rect = Rectangle::new(0.0, 0.0, 4.0, 5.0);
    assert!(!rect.contains_point(6.0, 7.0));
}