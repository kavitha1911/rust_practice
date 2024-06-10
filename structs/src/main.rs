//session-7 21/05/2024

//#![no_std]
//#[warn(unused_variables)]
/* First topic is about --> ENUMERATION
*/

//use std::clone;
//#[repr(C)] --> this is an attribute...
// C memory layout , rust memory layout is different

//If we are creating the binary crate then cargo run --bin app1a 
#[derive(Debug)]
enum WeekDays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}
#[derive(Debug, Clone)]
enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeConstant(i32, i32, i32),

}

impl Message{
    fn print(&self) {
        match self {         
        Message::Quit => {println!("Quit")},
        Message::Move {x, y} => {println!("x:{x}, Y:{y}");},
        Message::Write(text) => {println!("{text}")},
        Message::ChangeConstant(r, g, b) => println!("{r} {g} {b}"),}
    }
    
fn move_to(&mut self, x:i32, y:i32){
    *self = Message::Move {x:x, y:y}; //dereference here
}
}
fn main() {
   // println!("Hello, world!");

    let week = WeekDays::Monday;
    println!("{:#?}", week); //unless the display trait is implemented it is not displayed using println! 
    //error occured std::fmt::Display
    // thing that is display trait can we can see the enumeration.
    let x = 200;
    let x2:Option<i32> = Some(200);
  /*  match x2{
       Some(y) => println!("{}", y),
       None => println!("its an error"),
    } */

   let message1 = Message::Quit;
   let message2 = Message::Move{x:100, y:200};
   let message3 = Message::Write("hello-ust".to_string());
   let message4 = Message::ChangeConstant(26, 67, 27);
   let mut message5 = Message::Quit;


 //  print_message(message1);
  // print_message(message2);
  // print_message(message3);
   //print_message(message4);
   message1.print();
   message2.print();
   message3.print();
   message4.print();
   message5.print();
   message5.move_to(100, 200);
   message5.print();


 /*   let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(k) => println!("Matched, x = {k}"),
        _ => println!("Default case, x = {:?}", x),
    }
 
    println!("at the end: x = {:?}, y = {y}", x);  */




}

//in
//function print_message where enumeration is an input
fn print_message(msg:Message){
    match msg{
        Message::Quit => {println!("Quit")},
        Message::Move {x, y} => {println!("x:{x}, Y:{y}");},
        Message::Write(text) => {println!("{text}")},
        Message::ChangeConstant(r, g, b) => println!("{r} {g} {b}"),
    }
}


/* 
fn get_value(v:Option<i32>){
    match v{
        Some(n) => {},
        None =>
    }
}*/
// Work to do
//implement a function that return a result


/* try to do as you are able to achieve 
if the number is even give error mesasge odd values are not square 
fn get_even_square(num:i32) ->Result<i32,String>{
    let num = 20;
    let number = match num {
        Ok(num) =>number,
        Err(e) => return Err(e),
    };
    println!("{}", num * num);
    Ok(())
}
*/


