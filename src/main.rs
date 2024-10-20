fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let num1 = 54;
    let num2 = 24;
    
    println!("Найбільший спільний дільник {} і {}: {}", num1, num2, gcd(num1, num2));
}