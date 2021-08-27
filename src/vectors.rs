
pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}",numbers);
    numbers[2] = 10;

    println!("{:?}",numbers);
    
    println!("{}", numbers[2]);

    println!("{}", numbers.len());

    println!("{}", std::mem::size_of_val(&numbers));

    // push and pop
    numbers.push(1);
    numbers.pop();

 
    let slice : &[i32] = &numbers;
    println!("{:?}",slice);

// loop 
for x in numbers.iter(){
    println!("loop : {}",x)
}
for x in numbers.iter_mut(){
    *x*=2;
}
    println!("{:?}",numbers);
}