//13. Program to print pronic number between 1 to 100
pub fn pronic(i:i32) -> bool{
    let mut flag =false;
    for j in 1..i{
        //check for pronic by multiplying consecutive numbers
        if j*(j+1) == i{
            flag = true;
            break;
        }
    }
    flag
}