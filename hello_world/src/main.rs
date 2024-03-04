
fn get_names(names: Vec<&'static str>) -> Vec<&'static str> {

  names.iter()
    .map(|s| s.to_string()) 
    .collect()
}


fn main() {
  let a = vec![1, 2, 3];
  let b = vec![4, 5, 6];

  let mut sum = vec![0; a.len()];

  for i in 0..a.len() {
    sum[i] = a[i] + b[i];
  }

  println!("Sum vector is: {:?}", sum);

  let x: f32 = 1.5;
 let y = 2.3; // y is f64 by default
 let sum = x + y;
 println!("Sum is {}", sum);

 let x = 1.5;
 let y: f32 = x as f32;
let x: f32 = 1.5; 
let y = x as f64;
let x = 5;
let _y = x as f32; 

let z = 6.7 as i32;
println!("{}", z);

let names = vec!["Sam", "Sara"];
let name_list = get_names(names);


}



