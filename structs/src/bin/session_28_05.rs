/* SESSION - 10
28/05/2024
ZERO COST ABSTRACTION- WHICHEEVR THE COST sTRUCT--> RUST ENCOURAGES U TO DEFINE 
THERE WOULD BE NOT RUN TIME . Improve the performance of application
traits give clarity to code

//Rust don't encourage the object oriented programming
// One defi about trait is that it is an interface where you can implement for different types.
//not encourage run time polymorphism
//One more about trait is a set of methods.
//Rust does not encourage you to do polymorphism 
//there is no inheritance it looks like the structure dont actually follows the rule of oops

* oops is not for performance but for the flexibility.
* security to the code.
*PAss object through methods



TRAITS- It can bring some constraint to input and output arguments.
When we create a trait
when u pass a trait as a object as a  parameter. some constraint/ boundaries




*/


fn main(){

    //     let a:i32 = 10;
    //   //  let b = a.sum(5);
    //     println!("Initial value:{}", a);
    //     println!("after summing:{}", b);
    //     println!("get method:{}", b.get());


    // let mut num1 = Number::new(100);
    // let sum = num1.add(100).get();
    // println!("{},", sum);

    let a:i32 = 10;
    let b:i32 = 15;
    println!("{} the sum ", a.sum(b));
    println!("{} the sub ", a.subtract(b)); 
    println!("{} the multiply ", a.multiply(b));
    println!("{} the divide", a.divide(b));   


    let c:f32 = 10.12;
    let d:f32 = 13.12; 
    println!("{} the sum ", c.sum(d));
    println!("{} the sub ", c.subtract(d)); 
    println!("{} the multiply ", c.multiply(d));
    println!("{} the divide", c.divide(d));  
    
    let e:f64 = 19.0;
    let f:f64 = 12.12;
    println!("{} the sum ", e.sum(f));
    println!("{} the sub ", e.subtract(f)); 
    println!("{} the multiply ", e.multiply(f));
    println!("{} the divide", e.divide(f));  
}

/*trait Calculator<t>{
    fn sum(self, other:T) -> T;
    fn sub(self, other:T) -> T;
} */
/* 
trait Math{
   // type Output: Summing;
    fn get(&self) -> i32;
    fn add(&self, num:i32) -> &dyn Math;

}

// impl Math for i32{
//   //  type Output = i32;
//     fn get(&self) -> i32{
//         *self
//     }

//     fn add(&mut self, num:i32) -> &dyn Math;
    
// }

struct Number{
    num:i32
}

impl Number{
    fn new(num:i32) -> Self{
        Number{num:num}
    }
}
impl Math for Number{
    fn get(&self) -> i32{
        return self.num;
    }

    fn add(&self, num:i32) -> &dyn Math{
        self.num += num;
        return self;

    }
}
*/

trait Calculator<T>{
    fn sum(&self, others:T) -> T;

    fn subtract(&self, others:T) -> T;

    fn multiply(&self, others:T) -> T;

    fn divide(&self, others:T) -> T;

}


impl Calculator<i32> for i32{
    fn sum(&self, others:i32) -> i32{
        self + others
    }
    fn subtract(&self, others:i32) -> i32 {
        self - others
    }

    fn multiply(&self, others:i32) -> i32 {
        self * others
    }

    fn divide(&self, others:i32) -> i32 {
        self / others
    }
}


impl Calculator<f32> for f32{
    fn sum(&self, others:f32) -> f32{
        self + others
    }
    fn subtract(&self, others:f32) -> f32 {
        self - others
    }

    fn multiply(&self, others:f32) -> f32 {
        self * others
    }

    fn divide(&self, others:f32) -> f32 {
        self / others
    }
}


impl Calculator<f64> for f64{
    fn sum(&self, others:f64) -> f64{
        self + others
    }
    fn subtract(&self, others:f64) -> f64 {
        self - others
    }

    fn multiply(&self, others:f64) -> f64 {
        self * others
    }

    fn divide(&self, others:f64) -> f64 {
        self / others
    }
}

//create a trait called calculator
/* 

To do task
Sum, subtract, multiply, divide, 
-implement this on i32, f32, f64 types.

-Create a trait to implement Sum method
trait Summing
get(&self,) -> i32
sum(self, num:i32) -> Summing

Assignment - 1 - 
*/