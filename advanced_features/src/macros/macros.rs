#[macro_export]
macro_rules! vec {
   ( $( $x: expr ),* ) => {
      {
         let mut temp_vec = Vec::new();

         $(
            temp_vec.push($x);
         )*

         temp_vec
      }
   };
}

pub fn try_macro() {
    let v: Vec<u32> = vec![1, 2, 3];
}
