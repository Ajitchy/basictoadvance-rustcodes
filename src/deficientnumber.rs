//The deficient number can be defined as the number for which the sum
// of the proper divisors is lesser than the number itself. As number 21
//with its proper divisors (1, 3 and 7) has sum (11) lesser than itself.
//14. Program to determine whether a given number is a Deficient number

pub fn isdef(n:i32) -> i32{
    i32::from(divsum(n) < (2 * n))

}

fn divsum(n:i32) -> i32 {
    let mut sum = 0;
    for i in 1..((n as f32).sqrt() as i32){
        if n% i  ==0{
            if n/i  == i {
                sum = sum + i;
            }
            else {
                sum = sum + i ;
                sum = sum + (n/i);
            }
        }
    }
    sum
}