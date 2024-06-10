//struct in rust
fn main(){
    //let l1 = Rec{ length:23.0 , breadth:21.0, a:0.0, b:0.0};
    // Creating an instance like below
    let l1 = Rec::new(10.11, 13.12);
    println!("{} the area of rect", l1.area());
    println!("{} the peri of rect", l1.perimeter());
   let l2 = Unit;
   l2.greet();

}
//normal struct
struct Rec{
    length:f32,
    breadth:f32,
    //a: f64,
    // The condition here is to store the area in a;
    a:f32,
    // to store the perimeter in b
    b:f32,
}

impl Rec{
    //creating an instance we use something called new and then the instance is Self. 
    //u r returning the rectangle with thesse values  
    //If we are using uupercase Self, we are actually creating instance
fn new(l:f32, b:f32) -> Self{
    Rec{length:l, breadth:b, a:0.0, b:0.0}
 }
    //and here we are using the small (self) where we are using the instance by a period operator. 
    fn area(&self) -> f32{
        return self.length * self.breadth ;
    }

    fn perimeter(&self) -> f32{
        return 2 as f32 *self.length + self.breadth ;
    }


}
//new type Struct
//struct Square(f32);
//Empty Struct
//struct Square()

//unit struct or empty struct 
 struct Unit;
 impl Unit{
    fn greet(&self){
        println!("Hi , I am calling unit strcut");
    }
 }
struct Address{
    country:String,
    pincode:String,
}
struct Person{
    name:String,
    email:String,
    address:Address,
}


//const will be difficult as there will be problem in the interoperability
//static variables are a global variable
// Struct at located in the stack
// Lifetime Aliason --ve
//non lexical has been incorporated in rust