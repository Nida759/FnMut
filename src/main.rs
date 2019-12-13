// fn hey<T:FnMut()>(mut y:T){
//     y();
// }
// fn main() {
//     let mut x = String::from("Outgoing");
//     let z = || x.push_str(" heavy rain"); 
//     // println!("{}",x);
//     hey(z);
//     println!("{}",x);
// }

// #![allow(unused)]
// fn main() {
// fn do_twice<F:FnMut()>(mut func: F)
// {
//     func();
//     func();
// }
// let mut x: usize = 2;
// {
//     let add_two_to_x = || x += 5;
//     do_twice(add_two_to_x);
// }
// println!("{}",x);
// }