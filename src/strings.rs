//Primitive Str -->>>> Immutable fixed len String Somewhere in memory 
// String -->>>> Growable, heap allocated data structures - use when you need to modify or own string data
pub fn run (){

    let hello = "hello";//Primitive Str
    let mut hello2 = String::from("huzaifa");//String

    println!("{}, {}",hello,hello2);
    
    println!("lenght is {}",hello2.len());

    hello2.push(' ');
    hello2.push_str("Ali"); // String :: from
    // capacity
    println!("{}",hello2.capacity());
    // is empty
    println!("{}",hello2.is_empty());
    //contains (in)
    println!("{}",hello2.contains("Ali"));
    //replace
    println!("{}",hello2.replace("Ali","Ali H"));
    
    hello2 = hello2.replace("Ali","Ali H");
// loop on string
   for word in hello2.split_whitespace(){
       println!("{}",word);
   }
   // String with capacity 
   let mut s = String :: with_capacity(10);
   s.push('a');
    s.push('b');

    // Assertion testing
   assert_eq!(2,s.len());


}