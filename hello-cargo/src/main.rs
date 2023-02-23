fn foo() {
    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;
    // Declare second variable binding, shadows existing variable "shadow_num" 
    let shadow_num= shadow_num + 5; 
    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2; 
    println!("The number is {}.", shadow_num);
    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger); 
    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃';
    println!("{} - {} - {}", uppercase_s, lowercase_f, smiley_face);
}

fn main() {
    foo()
}
