
use vector_math::vector::Vector;

fn main() {
  
  let mut v = Vector::new(3.0, 4.0, 5.0);
  println!("vetor original ({}, {}, {})",v.x, v.y, v.z);
  println!("Magnitude: {}", v.magnitude());

  v.normalize();   
  println!("vetor Normalizado ({}, {}, {})",v.x, v.y, v.z);     
}
