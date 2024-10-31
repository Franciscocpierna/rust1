
pub fn busca(numero: Vec <i32>, numero1: i32)->i32 {
    
    for (i,&v) in numero.iter().enumerate(){
       if v == numero1{
        return i as i32;
       }

    }
  -1
}


/* professor

pub fn binary_search(nums: &Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}
*/