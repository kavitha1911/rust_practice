fn main() {
    //println!("Hello, world!");
    let k = 100;
    let mut l = k;
    l+=1;
    println!("{}", k);
    println!("{}", l);
    let s = String::from("Hello world");
    let s1 = s;
    //  println!("{}", s);
    //println!("{}", s1);
    // option in rust

    let mut num:String = String::from("ust global");
    //there is no null or nil in option 
    // It has rust
    let mut comp_name:Option<String> = Some(String::from("ust global"));
    let mut c_2:Option<String> = None;
    let mut number1:Option<i32> = None;
    let mut number2:Option<Box<i32>> = None;
    match comp_name{
        Some(n) => println!("{}", n),
        None => println!("none"),
    }

} 
/*
fn increment(num:Option<i32>){
    match num{
  //      Some(n) => 
    }
}
*/

//traits in rust