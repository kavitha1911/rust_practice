/*

TRAIT BOUND - Require a generic type to implement specific traits
* Guarantees the generic type will have nececssary behaviors



*/
use std::any;
use std::fmt;
fn print_type<T:fmt::Display> (item:T){
    println!("{} is {}", item, any::type_name::<T>());


}
fn main(){
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    
}