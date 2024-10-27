
pub struct Solution;



impl Solution{
pub fn two_sums(v: Vec<i32>, alvo: i32) -> Vec<i32>{
     println!(" v= {:?}", v);
     println!("{}", alvo);
     for (i,e1) in v.iter().enumerate(){
        //println!("(i,e1)= {:?}",(i, e1));
        
    
       for(j,e2) in v.iter().skip(i + 1).enumerate() {
          // println!("(j, e2)= {:?}",(j, e2));
         if e1 + e2 == alvo{
            //println!("[e1, e2] = {:?}, [i, j] = {:?}",vec![e1, e2], vec![i, j]);
            //println!("[i, j + 1 + 1] = {:?}", vec![i, j + 1 + i]);
            return vec![i as i32, j as i32+1+i as i32];
         }
       }
    
      }
      vec![]
  }
}