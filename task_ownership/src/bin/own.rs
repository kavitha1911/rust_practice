fn main(){
    let s1 = gives_ownership();
    let s2 = String::from("craeting a string");

    let s3 = takes_back_ownership(&s2);
    println!("{}", s3);
    println!("{}", s2);
}

fn gives_ownership() -> String{
    let some_string = String::from("lias");
    some_string
}

fn takes_back_ownership(a_str: &String) -> &String{
    &a_str
}