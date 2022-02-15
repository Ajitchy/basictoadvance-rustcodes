//Program 16 -- Twisted prime Number
//A number is called a twisted prime number if it is a prime number and reverse of this number is also a prime number.
pub fn twistedprime(mut n:i32) -> i32{
    let mut reverse;
    let mut sum = 0;
    let  mut flag = 0;
    while n!=0 {
        reverse = n % 10;
        sum = sum * 10 + reverse;
        n = n/10;
    }
    for j in 2..(sum/2){
        if (sum%j)==0{
            flag =1;
            break;
        }
    }
    flag
}