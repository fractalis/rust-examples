fn main() {
    let x: u32 = 10;
    println!("The value of x is {}", x);
    let y: u32 = 20;
    println!("The value of y is {}", y);
    let z: f64 = 3.14159;
    println!("The value of z is {}", z);

    let sum: u32 = x+y;
    let difference: u32 = y-x;
    let product: u32 = x*y;
    let quotient: u32 = y/x;
    let remainder: u32 = y%x;

    println!("Sum: {} Difference: {} Product: {} Quotient: {} Remainder: {}", sum, difference, product, quotient, remainder);
    let is_true: bool = true;
    println!("The value of is_true is {}", is_true);

    let tup: (i32, f64, u8) = (500, 3.14, 1);

    let (xx, xy, xz) = tup;
    println!("xx: {} xy: {} xz: {}", xx, xy, xz);

    let a = [1,2,3,4,5];

    let aa: [i32; 5] = [1,2,3,4,5];

    let first = a[0];
    let second = aa[1];

    println!("First: {}, Second: {}", first, second);


}
