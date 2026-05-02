fn function_above() {
    println!("This function placed above the main function!");
}

fn main() {
    function_above();
    function_below();
    let res = another_function(1923, 'E'); 
    println!("The result is {res}");
}

fn function_below() {
    println!("This function placed below the main function!");
}

fn another_function(x: i32, unit_label: char) -> i32{
    println!("The value of x is {unit_label}{x}");
    29
}