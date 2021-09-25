use std::{thread,time::Duration};

pub fn run(){
    let v = vec![1,2,3,4,5];
    let handle = thread::spawn(move || { // giving the ownership of the "v"
  
        println!("{:?}", v);
        // for i in 1..10{
        //     println!("{} spawn", i);
        //     thread::sleep(Duration::from_millis(1));


        // }

    });
    // drop(v);
    // for i in 0..5{
    //     println!("{} main",i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    handle.join().unwrap();

}