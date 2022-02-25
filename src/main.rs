//1.Check Whether a number is positive or negative
/*
use std::io;
fn main(){
    println!("please enter the number");
    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read user input");
    let number : i32 = input_number
        .trim()
        .parse()
        .expect("Input is not a number");
    if number > 0{
        println!("Number is positive");
    }else if number < 0 { println!("Number is negative");
    }else { println!("Number is neither positive nor negative"); }

}
*/
//2.Check Whether a number is even or odd
/*
use std::io;

fn main(){
    println!("Enter a number:");
    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read user input");
    let number: i32 = input_number
        .trim()
        .parse()
        .expect("Input is not a number");
    if number%2==0{
        println!("Number is even");
    }else { println!("Number is odd"); }
}
*/
//3.program to calculate area of rectangle
/*
use std::io;
fn main(){
    println!("Please enter length and breath of rectangle:");
    let mut input_length =String::new();
    let mut input_breadth = String::new();
    io::stdin()
        .read_line(&mut input_length)
        .expect("Failed to read length");
    io::stdin()
        .read_line(&mut input_breadth)
        .expect("Failed to read breadth");

    let length: f32 = input_length
        .trim()
        .parse()
        .expect("Input string was not a number");
    let breadth: f32 = input_breadth
        .trim()
        .parse()
        .expect("Input was not a number");
    let area = length * breadth;
    println!("Area of rectangle is:{}", area);

}
*/
//4. program to calculate volume of sphere
/*
use std::io;
fn main(){
    println!("Enter value of radius");
    let mut input_radius = String::new();
    io::stdin()
        .read_line(&mut input_radius)
        .expect("Failed to read radius");
    let radius: f32 = input_radius
        .trim()
        .parse()
        .expect("Input value is not a float");
    let volume = 4.0/3.0*(3.14)* radius.powf(3.0);
    println!("Volume of sphere is:{}", volume);

}
*/
//5. Program to find area of pentagon
/*
use std::io;
fn main(){
    println!("Enter the value of length of apothem and length of side");
    let mut input_apothem = String::new();
    let mut input_side = String::new();
    io::stdin()
        .read_line(&mut input_apothem)
        .expect("Failed to read apothem");
    io::stdin()
        .read_line(&mut input_side)
        .expect("failed to read side");

    let apothem: f32 = input_apothem
        .trim()
        .parse()
        .expect("Entered value is not float");
    let side: f32 = input_side
        .trim()
        .parse()
        .expect("Entered value is not float");
    let area_of_pentagon = 5.0/2.0*(side*apothem);
    println!("Area of pentagon is: {}", area_of_pentagon);
}
*/
//6. Find volume of cube
/*
use std::io;

fn main(){
    println!("Enter length of side of cube");
    let mut input_length =String::new();
    io::stdin()
        .read_line(&mut input_length)
        .expect("Failed to read length");
    let length: f32 = input_length
        .trim()
        .parse()
        .expect("Entered value is not a float");
     let volume = length.powf(3.0);
    //let volume = length*length*length;
    println!("Volume of cube is: {}", volume);
}
*/
//7.Program to calculate CGPA Percentage
/*
use std::io;
fn main(){
  println!("Enter the sum of grades in all subject and Total number of subjects");
  let mut input_sum =String::new();
  let mut input_num= String::new();
  io::stdin()
      .read_line(&mut input_sum)
      .expect("Failed to read the sum");
  io::stdin()
      .read_line(&mut input_num)
      .expect("Failed to read the number");
  let sum:f32 = input_sum
      .trim()
      .parse()
      .expect("Entered value was not a float");
  let number_of_subject:f32 =  input_num
      .trim()
      .parse()
      .expect("Entered value is not a float");
  let cgpa = sum/number_of_subject;
  println!("CGPA Percentage of Student is : {}%", cgpa);
}
*/

//8. convert fahrenheit to celsius
/*
use std::io;

fn main() {
    println!("Please enter temperature in fahrenheit:");
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read a line of user input");
    let fahrenheit: f32 = input_string
        .trim()
        .parse()
        .expect("Input string was not a float");
    let celsius = 5.0*(fahrenheit - 32.0)/9.0;
    println!("{} °F = {} °C", fahrenheit, celsius);
}
 */
/*
//9. Program to check Disarium Number = 1^1 + 7^2 + 5^3 = 1 + 49 + 125 = 175
use std::io;
fn main(){
    println!("Enter the number");
    let mut input_number =String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read the number");
    let mut number: i32 = input_number
        .trim()
        .parse()
        .expect("Entered value is not a number");

    let mut sum:i32 = 0;
    let mut rem;
    let mut len = calculate_length(number);
    let  input_number = number.clone();
    while number > 0 {
        rem = number % 10;
        sum = sum + rem.pow(len as u32);
        number = number / 10;
        len = len -1;
    }
    if sum == input_number {
        println!("Entered number:{} is a disarium number", input_number);
    } else {
        println!("Entered number is not a disarium number");
    }
}
fn calculate_length(mut number:i32) -> i32 {
    let mut length = 0;
    while number!=0{
        length = length + 1;
        number = number/ 10;
    }
    length
}
*/
//10.Program to check happy number
/*use std::io;
fn main(){
    println!("Enter the number");
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Not able to read entered value");
    let mut happy_number: i32 = input_number.trim().parse().expect("Entered number is not integer");
    let mut result = sum_of_square(happy_number);
    while result!= 1 && result!=4{
        result = sum_of_square(happy_number);
        println!("{}",result);
        if result == 1{
            println!("Entered value is a happy number");
        }else{
            happy_number = result;
        }
    }
}
fn sum_of_square(mut happy_number:i32) -> i32{
    let mut sum = 0;
    let mut rem;
    while happy_number!= 0{
        rem = happy_number % 10;
        sum = sum + rem.pow(2);
       happy_number = happy_number / 10;
    }
    sum

}
*/
//11.program to print happy number between 1 to 100
/*
fn main() {
    for i in 1..100 {
      let mut result = i;

    while result != 1 && result != 4 {
        result = sum_of_square(result);
        if result == 1 {
            println!("{}", i);
        }
    }
    }
}
fn sum_of_square(mut result:i32) -> i32{
    let mut sum = 0;
    let mut rem = 0;
    while result > 0{
        rem = result % 10;
        sum = sum + rem.pow(2);
        result = result / 10;
    }
    sum
}
*/
/*
//12. Program to check if given number is pronic number or not.
//A number is said to be pronic number if it is a product of two consecutive numbers.
// Like 6 = 2*3;

use std::io;
fn main() {
    println!("Enter the number");
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Not able to read the given number");
    let pronic_number: i32 = input_number.trim().parse().expect("Entered number is not integer");
    let num: i32 = pronic_number;
    let mut flag = false;
    for j in 0..num{
        if j* (j+1) > num{
            break;
        }
        if j * (j + 1) == num {
            flag = true;
            println!("Entered number:{} is pronic number",num);
            break;
        }
    }

    if !flag {println!("Entered number {} is Not a pronic", num);}
}
 */
//From here onwards we will start using MODULE i.e we will be making multiple source
// files and then calling each code in the main.rs file

use crate::deficientnumber::isdef;
use crate::pronicnumber::pronic;
use std::io;
use crate::abundantnumber::checkabundant;
use crate::twistedprimenumber::twistedprime;

mod pronicnumber;
mod deficientnumber;
mod abundantnumber;
mod twistedprimenumber;


fn main(){

    //13. print pronic number main call
    for i in 1..100{
        if pronic(i){
            println!("pronic numbers between 1 to 100:{}",i);
        }
    }

    println!("Enter the number");
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Not able to read the given number");
    let  n: i32 = input_number.trim().parse().expect("Entered number is not integer");

    //Program 14 -- deficient number main call
    if isdef(n) == 0{
        println!("The number is not deficient");
    } else { println!("Number is deficient");
    }

    //Program 15 -- Abundant number main call
    if checkabundant(n) > n{
        println!("Entered number is abundant");
    }else { println!("Entered number is not abundant");
    }

    // program 16 -- Twisted Prime number main call
    if twistedprime(n) ==0 {
        println!("Entered number is Twisted Prime ");
    }else {
        println!("Not a Twisted Prime");
    }

    // program 17 -- Abundant1to100 main call
    //println!(" Abundant number between 1 to {} is {}",n,abundant1to100(n));
    for j in 1..=n{
        if checkabundant(j) > j{
            println!("Abundant number between 1 to {}:{}",n,j);
        }
    }

    // Program 18 -- Program to print all Kaprekar numbers between 1 to 100


}