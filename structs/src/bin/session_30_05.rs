/*   

*/

//impl is used to give better performnace because of the static dispatch

fn main(){
    let mut t1 = Number::new(100);
    println!("{:?}", t1);
    let sum = t1.add(100).add(200).get();
    println!("{}", sum);
}

trait Math{
    fn add(&mut self,num:i32) ->&mut dyn Math; // the instance which we are calling themselves
    fn get(&mut self) ->i32;
    //dyn -> run time polymorphism oops confused
}



#[derive(Debug)]
struct Number(i32);


impl Number{
    fn new(num:i32) -> Self{
        return Number(num);
    }
}
impl Math for Number{
    fn add(&mut self, num:i32) ->&mut dyn Math {
        self.0 = self.0+num;
        return self;
    }

    fn get(&mut self) ->i32{
        return self.0;
    }
}