// 23/05/2024

//TOPIC:  TRAITS AND CODE ORGANIZATION
// LIFETIMES - (Scocpe)In function, interfaces, type, if my type doesnt have references , all the objects and variables or in a function they are all created instantiated in the same scope and died in the same scope.
// When there are multiple references multiple comes to the picture compiler doesnt able to understand, lifetimes into the picture
//Borrow checker infers the lifetime of str, instance, accordingly.

    //There is a owner of something , the value is borrowed temporarily to some variable or some other scope , you borrow from somebody 
    //Discussion abiut TRAITS* Different traits on various types
   // code organization
    //Packages [Library and normal crates]


/* INTERFACE - NEED OF INTERFACE
  * It is a soecification not implementation
  *different objects same method/implementation but different behavior.
  *Polymorphism / Multiple inheritance use interface
  hierarchy of system,

  * glue different objects 
  *it can be constraint
  *generics

  Creating an object or types have to follow the rules. the rules are defined in a trait
  Multilevel Inheritance through traits in rust
    "Derived inheritances"
 */

 /*    

  */

  fn main(){
    //Create a object
    let r1 = Rect::new(12.34, 13.45);
    area_of(&r1);
    //reference to the 100.
    let k = &100; //  create memory to store i32 and store 100 in it . No name for that only a reference  
   // *k = 101; // 101 will be updated to the same memory where 100 is stored.
    println!("{}", k);
    //here the adress is stored in stack
    let s1 = Square::new(2.0);
    area_of(&s1);
    println!("{:?}", s1);



    /*
    int num1 = 100;
    imt *ptr = &num1;
    ptr2 = new(int);
    
    there is something called type inference - When you assign a value to a variable based on the value it automatically takes the type.
     
     create a pointer in rust w/o any memory.
     *

     */
}
//any object implements the traits

//This shape will take the shape trait
// we need to use impl or dyn
//this is a trait type
//Any object that satisfies the trait or impl the trait
// dyn we can give there is a dynamic dispatch
//impl vs dyn - impl try to use stack (most of the case) dyn - use heap always at run time 
// fn area_of(o: &dyn Shape){
//Array is a statically craeted variable and vectors are dynamically created variable.

fn area_of(o: &impl Shape){  
o.display();  
let a = o.area();
println!("Area is{} ",a );
let p = o.perimeter();
println!("Perimeter is {}", p);
}

//How to craete trait
/*trait Shape{
    //shape is object
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    //there are shape which has two diffrent behavior
} */

//This is how inheritance -- implementing inheritance with traits


trait Area{
    //shape is object
    fn area(&self) -> f32;
    //there are shape which has two diffrent behavior
}
trait Perimeter{
    //shape is object
    fn perimeter(&self) -> f32;
    //there are shape which has two diffrent behavior
}
// to achieve a inheritance for shape in rust
// Area+ Perimeter is multi level inheritance
// Shape= inheritance
trait Shape: Area+Perimeter{
    fn display(&self){
        println!("The shape for area and perimeter is");
    }

}

//self, &self, &mut self and Self
#[derive(Debug)]
struct Rect{
    l:f32, // attribute
    b:f32, 
}
// enforcing rect two use two differnet method
//It is used to define implementation on types.

impl Area for Rect{
    fn area(&self) -> f32{
        return self.l * self.b;
    }
}
impl Perimeter for Rect{
    fn perimeter(&self) -> f32 {
        return 2 as f32 * self.l + self.b;
    }
} 

impl Shape for Rect{
    fn display(&self) {
        println!(" the shape for length{} and breadth is {}", self.l, self.b);
    }
}

impl Rect{
    fn display(&self){
        println!("Rect is {} peri {}", self.l, self.b);
    }
    //Constructor
    fn new(l:f32, b:f32) -> Self{
        Rect{l:l, b:b}
    }

    fn default() -> Self{
        Rect{l:0.0, b:0.0}
    }
}

#[derive(Debug)]
struct Square(f32);
impl Area for Square{
    fn area(&self) -> f32{
        return self.0 * self.0
    }
}
impl Perimeter for Square{
    fn perimeter(&self) -> f32 {
       return 4 as f32 * self.0
    }
}

impl Shape for Square{
    fn display(&self){
        println!(" Square side is : {}", self.0);
    }
}

impl Square{
    fn new(s:f32) -> Self{
        Square(s)

    } 
}

//How can we have a default trait implementation in rust
// Interface in trait
// When should we implement methods and when should we implement trait
