pub fn run(){
    let str = "huz";
    let  mut string = str.to_string();
    let str1 = &string.to_uppercase();
    println!("{}", str1);
    println!("{}", string);
    // let s2 = str + str1;
    // let s2 = string + str1; // ownership 
    string.push_str("m");
 }