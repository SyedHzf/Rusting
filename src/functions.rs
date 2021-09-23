pub fn run(){

    greeting("kiya haal hain", "Huzaifa");
    let sum = add(5,7);
    println!("{}",sum);
    let n3 = 3;
    let mul = |n1: i32 , n2: i32| {
        
        n1*n2*n3
    };
    println!(" multiply {}", mul(3,2) );
}
    
fn greeting(greet : &str, name : &str){
    println!("{} you {}", greet,name);}

fn add(num1 : i32 , num2 : i32) -> i32 {

   return num1+num2

}