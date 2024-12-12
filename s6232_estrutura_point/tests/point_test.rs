
use s6232_estrutura_point::Point; // cargo test --test point_test

#[test]
fn test_distance_between_points(){
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    assert_eq!(p1.distance(&p2), 5.0);//9 + 16 = 25 => sqrt => 5 
    
} 

#[test]
fn test_move_point_to_new_position(){
    let mut p = Point::new(0.0, 0.0);
    p.move_to(5.0, 5.0);
    assert_eq!(p.x, 5.0);
    assert_eq!(p.y, 5.0);
}