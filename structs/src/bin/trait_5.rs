fn main(){

    let num1 = Number::new(100);

}

//created a trait called math

trait Math{
    //Associated type
    //asserted type in traits type T;
    // type U;
    fn get(&self) -> i32;
    fn add (&mut self, num:i32) -> &mut impl Math;

}

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

    fn add(&mut self, num:i32) -> &dyn Math{
         self.num+=num;
         return self;
    }
}