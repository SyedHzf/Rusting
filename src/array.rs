
pub fn run(){
    let numbers: [i32;5] = [1,2,3,4,5];
    println!("{:?}",numbers);
    
    println!("{}", numbers[2]);

    println!("{}", numbers.len());

    println!("{}", std::mem::size_of_val(&numbers));

    let slice : &[i32] = &numbers;
    println!("{:?}",slice);
}