fn main() {
    let may_overflow = 250u8;
    println!("The value of may_overflow is {may_overflow}");
    
    let wrapping = may_overflow.wrapping_add(6);
    let checked = may_overflow.checked_add(6);
    let overflowing = may_overflow.overflowing_add(6);
    let saturating = may_overflow.saturating_add(6);
    println!("The value of wrapping is {wrapping:#?}");
    println!("The value of checked is {checked:#?}");
    println!("The value of overflowing is {overflowing:#?}");
    println!("The value of saturating is {saturating:#?}");

    // addition
    let sum = 5 + 10;
    println!("The value of sum is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let not_truncated = -5.0 / 3.0;
    println!("The value of quotient is {quotient}");
    println!("The value of truncated is {truncated} when quotient is {not_truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is {remainder}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of tup is {tup:?}");
    println!("The values of x, tup.1/y, z are: {x}, {}/{y}, {z}", tup.1);

    let empty: () = ();
    println!("The value of empty is {empty:?}");

    let a: [i32; 5] = [5; 5];
    println!("The array a is {a:#?}");
    println!("The array a[0] is {}", a[0]);
}