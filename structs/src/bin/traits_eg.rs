fn main()
{
    println!("hello");
    let mut instance = Mystruct{value:4};
    instance.modify();
}
//Traits
struct Mystruct{
    value:i32,
}

impl Mystruct{
    fn modify(&mut self){
        self.value += 1;
    }
}


/*



*/