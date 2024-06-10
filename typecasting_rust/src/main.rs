/* SESSION 5 09/05/ 2024
CONVERSION LOOPS
 */

 fn main() {
   // println!("Hello, world!");
    // variable conversions
    let age:u8 = 42; //1 byte
    let num:u16 = age as u16; // 2 bytes
    println!("{num}");

    // /this types of implicit is not possible in rust.
    // We have to convert it by a keyword calling as
    //in rust there are two traits
    //into () from() - these are traits are nothing but similar to interfaces
    let ok:bool = true;
    let num2:u8 = ok as u8;
    println!("{num2}");

    let a1:char = 'A';
    let i1:u16 = a1 as u16;
    println!("{i1}");

    let ch: char = 'i';
    let ni: u16 = ch as u16;
    println!("{ni}");

    let n2:u16= 65;
    let char1 = char::from_u32(n2 as u32);  //
    match char1{
        Some(ch)=> println!("{ch}"),
        None => {},
        
    }


    let b:i8 = 0b0000001;
    println!("byte is ={}", b);

    let c:u8 = 0x01;
    println!("hex is = {}", c);

   // let bool1:bool = true;
  //  let ch1:char = bool1 as char;

//String typcating into integers or floating point

    let s1:&str = "3.14";
    let s2 = s1.parse::<f64>();
    match s2 {
        Ok(t) => {println!("{}", t)}
        Err(k) => println!("nothing to show{k}")
    }

    let mut count:u32 = 0;  
    loop{
       
         count+=1;
         println!("{}", count);
         if count == 20{
            break;
         }

        }
    let arr = [0, 1, 2, 3];
    for n in arr{
        println!("{:?}", n);
    }
    //Conversion of bool to f64
    let m1:bool = true;
    let m2 = m1 as i32 as f64;
    // Conversion of u8 to u64
    let ui1:u8 = 21;
    let ui2 = ui1 as u64;

    // Conversion i8 to i64
    let ii1: i8 = 32;
    let ii2: i64 = ii1 as i64;

    // conversion of f32 to f64 
    let fi1: f32 = 21.33;
    let fi2: f64 = fi1 as f64;

    //conversion of char to float
    let ch1: char = 'A';
    let ch2: f64 = ch1 as i32 as f64; 
    
    //conversion of string to f64
    let st1 = "108";
    let st2 = st1.parse().unwrap_or(0.0);
       /*    match st2{
            Ok(t1) => println!("{}", t1),
            Err(_) => println!("it's an error"),
        } */

    let sum1 = m2 + ui2 as f64 + ii2 as f64 + fi2 + ch2 + st2;
    println!("tje total sum is :{}", sum1);


}


//bool, u8 to u64, i8 to i64, f32 to f64, char, string with "108"
//sum:f64 = 

 


//Rust also supports impilcits typecasting but sometime t=rust we dont use implicit typecasting

//implicit and explicit 
//converting integers from different types of types
//implement from and into trait from this example
// option is very much important in rust it is a type in enumeration 




//Error trait
//Error : Debug, display
//Error methods
// Debug methods
//Display methods



// Loops in rust

