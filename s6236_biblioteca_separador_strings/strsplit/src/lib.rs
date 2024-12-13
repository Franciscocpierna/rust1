//#[warn(missing_debug_implementations, rust_2018_edition, missing_docs)]
#[derive(Debug)]
pub struct strSplit<'a>{
   remainder: Option<&'a str>,
   delimeter: &'a str,
}

impl<'a> strSplit<'a>{
    pub fn new(haystack: &'a str, delimeter: &'a str) -> Self{
       Self{
        remainder: Some(haystack),
        delimeter,
       }
    }

   // pub fn foo(x: &'a str, y: &'b str) -> &'a str{x} 
}

impl<'a> Iterator for strSplit<'a>{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item>{
       if let Some(ref mut remainder) = self.remainder{
            if let Some(next_delim) = remainder.find(self.delimeter){
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimeter.len())..]; 
                Some(until_delimiter)}
            else{
                self.remainder.take()
              }
        }else {
            None
        }

    } 
}

#[test]
fn it_works(){
    let haystack = "a b c d e";
    let letter = strSplit::new(haystack, " ");
    assert!(letter.eq(vec!["a", "b", "c", "d", "e"].into_iter()));

    
}

#[test]
fn tail(){
    let haystack = "a b c d ";
    let letter = strSplit::new(haystack, " ");
    assert!(letter.eq(vec!["a", "b", "c", "d", ""].into_iter()));

    
}