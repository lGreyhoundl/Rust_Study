// fn main(){
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The legnth of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main(){
//     let mut s = String::from("hello");
//     change(&mut s);

//     println!("{}", s);
// }

// fn change(some_string: &mut String){
//     some_string.push_str(", world");
// }

// fn main(){
//     let reference_to_notion = dangle();
// }

// fn dangle() -> &String{
//     let s = String::from("hello");

//     &s
// }