//lifetime in rust session - 6 
fn main() {
    println!("Hello, world!");
    let s1 = String::from("hello");
    let s2 = String::from("worldest");

    let x = 45;
    {
        let y = x;
        let z = 100;
        {
            let z1 = 101;

        }
        let s3 = &s1;
    }
    println!("{}",length(&s1));

    let mut b = Box::new(100);
    println!("{}", b);
    square(&mut b); 
    println!("new valeu:{}",b);
    let mut a = Box::new(200);
    println!("{}", a);

    let my_value = 10;
    
    println!("{}", my_value);
    println!("{}", add(my_value));


    let c = square1(a);
 
    println!("{c}");

    let mut num = Box::new(400);
    println!("{}", num);
    let f = square2(&mut num);
   println!("one:{}", f);
    //f will get a refernce of a variable inside the function
   square2(&mut num);
   println!("mut borrow: {}", num);

}
//Borrow the owner one after the particular data

/*fn  square1(v: &mut Box<i32>){
    **v = **v + **v;
}

fn square2(mut v:Box<i32>) -> Box<i32>{
    *v = *v + *v;
    return v;
}

fn square3(v: &mut Box<i32>) -> &i32{
    **v = **v +_ **v;
    let k:&i32 = &**v;
    
} */


fn length <'a>(s1:&'a str) ->i32{
    return s1.len() as i32
}  // do not deallocate s1 but give back the ownership to the original variable

fn longest <'a>(s1:&'a str, s2:&'a str) ->&'a str{
    if s1.len() > s2.len(){
        return s1
    }
    else {
        return s2
    }
    
    return "";
}


/* 
fn longest(s1:&str, s2:&str) ->&str{
    if s1.len() > s2.len(){
        return s1
    }
    else {
        return s2
    }
    
    return "";
}
*/

//borrowing so that ownership comes back to the same variable
fn square(v:&mut Box<i32>){
   **v = **v + **v;
}
// we are not borrowing but giving back the ownership of the same variable
fn square1(mut v:Box<i32>) -> Box<i32>{
    *v = *v+ *v;
    return v; 
} 


fn square2<'b>(v:&'b mut Box<i64>) -> Box<i64>{
    let k:i64  = **v as i64 * **v as i64; //stack allocated
    //let k = &**v;
    //creating new variable inside the function
    //k// problem of c / c++
    return Box::new(k);
 }
/* KEY POINTS 
   DANGLING REFERENCES
   when the reference exists, but the data it points to has been dropped.

 */

 fn add(mut a:i32) -> i32{
    a = 16;
    return a;
 }
struct A<'a>{
    s1:&'a str,
    s2: String,

}

