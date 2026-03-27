    /*
fn main (){
    let person: (String, i32, bool) = ("Alice".to_string(), 30, true);

    //Accessing elements of the tuple
    println!("Name {}", person.0);
    println!("Age: {}", person.1);
    println!("Is Employed? {}", person.2);
}
    
fn both_multiply_and_add(a: i32, b: i32) -> (i32, i32){
    return (a*b, a+b);
}

fn main(){
    let result = both_multiply_and_add(5,6);
    println!("product = {}", result.0);
    println!("sum = {}", result.1);
}
    */

fn both_multiply_and_add(a: i32, b: i32) -> (i32, i32){
    return (a*b, a+b);
}

fn main(){
    let (product, sum) = both_multiply_and_add(5,6);
    println!("product = {}", product);
    println!("sum = {}", sum);
}