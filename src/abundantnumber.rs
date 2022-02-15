// Program 15 -- Abundant number
//The number for which the sum of its proper divisors is greater than the number itself.
pub fn checkabundant(n:i32) -> i32 {
    i32::from(getsum(n) > n)
}
fn getsum(n:i32) -> i32 {
    let mut sum =0;
    for i in 1..((n as f32).sqrt() as i32){
        if n % i == 0 {
            if n/i == i {
                sum = sum + i;
            }
            else {
                sum = sum + i;
                sum = sum + (n/i);
            }
        }
    }
    sum = sum -n;
    sum
}