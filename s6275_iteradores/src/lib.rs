
 //iteradores => iterators

//pub trait iterator{
//
//    type item;
//    fn next(&mut self) -> Option<Self::item>;
    // metodo para obter o proximo elemento
//}

#[test]
fn test_iter_demonstration(){
  let v1 = vec![1, 2, 3];
  let mut v1_iter = v1.iter();

  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);

    
}


/*fn main() {
  
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  
  /*for val in v1_iter{
    println!("valores => {}", val);
  }*/
  
  /*println!("v1 => {:?}", v1);
  println!("v1_iter => {:?}", v1_iter);
  let v2: i32 = v1.iter().sum();
  println!("v1 => {}", v2);
*/
}
*/