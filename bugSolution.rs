fn main() {
    let mut x = 5;
    { //Creating a scope to limit the life time of the mutable reference y.
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 15;
    println!("x = {}", x); //Output: x = 15
}

//Alternative way of solving the issue:
fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    //Using another variable avoids any conflict
    let mut x2 = x; 
    x2 = 15;
    println!("x = {}", x); //Output: x = 10
    println!("x2 = {}", x2); //Output: x2 = 15
}