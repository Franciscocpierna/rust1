 pub mod  vector{   
    pub struct Vector{
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }
   impl  Vector{  
    pub fn new(x: f64, y: f64, z: f64)-> Self{
        Vector{x, y, z}
    }
    pub fn magnitude(&self) -> f64{
        (self.x.powi(2)+ self.y.powi(2)+ self.z.powi(2)).sqrt()
    }
    pub fn normalize(&mut self){
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }
 }
   
}
#[cfg(test)]
mod tests{
    use super::vector::Vector;
    const EPSILON: f64 = 1e-10;  
    #[test]
    fn test_vector_magnitude(){
        let v = Vector::new(3.0, 4.0, 5.0);
        assert!((v.magnitude() - 7.0710678118655).abs() < EPSILON);
        
    }
    #[test]
    fn test_vector_normalize(){
        let mut v = Vector::new(3.0, 4.0, 5.0);
        v.normalize();
        assert!((v.magnitude() - 1.0).abs() < EPSILON);
    }
}