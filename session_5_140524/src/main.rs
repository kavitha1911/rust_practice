// Sorry this is session 6

use std::collections::btree_map::{Keys, Values};
use std::mem;
use std::collections::HashMap;
fn main() {

    let arr: [i32; 5] = [3,2,8,7,9];
    println!("{:?}", arr);

    for v in arr{
        println!("v in array is :{:?}", v);
    }
    //println!("Hello, world!");

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let mut a = 10;
    v.push(a);
    println!("{:?}", v);
    a = 76;
    v.push(a);
    println!("{:?}",v);

    let a1:[i32; 4] = [2, 5, 7, 8];
    get_vec(a1);    

    println!("{}", string_method("lo"));

    slice_fn();
    let xs = [3,5,6,7,8,8,2,4,5];
    let xy = [0;500];
    analyze_slice(&xs);

    let ab:[&str; 4] = ["ji", "ki", "ri", "lp"];
    array_strings(ab);
    println!("{:?}", &ab);   



    let mut ss = String::from("hello");
    slice_string(&mut ss);

    let re = 50;
    let qe = 80;
    //reverse_no(re, qe);
    println!("{} {}", re, qe);

    let arr:[i32;4] = [2, 4, 5, 6];
    let l = get_sum_of_array(&arr);
    println!(" The u of the array is :{}", l);

    coolections_maps();

}

fn get_vec(v:[i32; 4]){
    println!("my new vec is : {:?}", v);

}

fn string_method(v:&str) -> &str{
    return "hello world";
}

/* --------------------------
*
 *   Slices in Rust
 --------------------------*/
fn slice_fn(){
    let v:[i32; 11] = [2, 4, 5, 6, 7, 9, 1, 4, 6, 88, 9];
    println!("{}", v.len());
    println!("the size is :{}", mem::size_of_val(&v));
    let s = &v[1..=8];
    println!("{:?}", s);



}
/* 
fn reverse_no(*mut a:[i32] , *mut b:[i32]){
        let temp = *a;
        *a = *b;
        *b = temp;

} */
//slices length is not known of compile time
/*
    * THE first word is a pointer to the data.
    the second word is the length of the slice
    Slices can be used to borrow a section of an arary and have the type signature &[T]
    
 */
// function for automatically borrowed as slice
fn analyze_slice(s: &[i32]) {
    println!("len = {}",s.len());
    println!("index = {}", s[0]);
    let v = &s[..8];
    println!("v  value in slice = {:?}", v);

}

fn slice_string(s:&mut String){

    println!("{}",s);
    s.push_str("world");
    println!("{}",s);      
}

fn array_strings(s:[&str;4]){
     println!("{:?}", s);
}
/* fn reverse_no(&a:[i32], &b:[i32]){
        let temp = a;
        a = b;
        b = temp;
}
*/

fn get_sum_of_array(arr:&[i32; 4]) -> i32{

    let mut sum:i32 = 0;
    for v in arr{
        sum += v;
    }

     println!("sum of the array is: {}", sum);
    0
}

/*  
HASH  MAP
Vectors store values by an integer index, HAshmap s stores values by key.
HAshmap keys can be booleans, integers, strings, or any other type that implements the eq and hash traits.
--> maps are heap allocated
--> hashmaps -> values are mapped to keys
--> key and value
--> hash maps, it creates some buckets --> 8 buckets
--> buckets are arrays or linked list
//key::5608 value called "bangalore -> yeshwantpur",

*/

fn coolections_maps(){
    let mut mymap:HashMap<String, String> = HashMap::new();
    mymap.insert("232".to_string(), "shalini".to_string());
    mymap.insert("233".to_string(), "meghana".to_string());
    println!("{:?}", mymap);
    let pincode:String = "232".to_string();
    let v = mymap.get(&pincode);
    match v{
        Some(vr) => println!("The id is matched {}", vr),
        None => println!("No value is found"),

    }

    //iterate through the map

    for (Keys,Values) in &mymap{
        println!("{:?} {:?}", Keys, Values);
    }

    //Removing an element from the ma
    mymap.remove(&String::from("232"));
    for (Keys, Values) in &mymap{
        println!("{} {}", Keys, Values);
    }

}





