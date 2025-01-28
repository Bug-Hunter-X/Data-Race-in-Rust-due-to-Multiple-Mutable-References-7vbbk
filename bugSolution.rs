fn main() {
    let mut x = 5;
    { //This block limits the scope of the mutable reference
        let y = &mut x;
        *y = 6;
    }
    { //This block limits the scope of the mutable reference
        let z = &mut x;
        *z = 7;    
    }
    println!("{}", x);
}
//Alternative Solution using clone:
fn main() {
    let mut x = 5;
    let y = x.clone();
    let z = x.clone();
    let mut y = y;
    let mut z = z; 
    y = 6;
    z = 7;
    println!("{}", y);
    println!("{}", z);
}