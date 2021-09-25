use std::sync::{Arc,Mutex};
use std::thread;
// use std::rc::Rc;

pub fn run(){
     let counter = Arc::new(Mutex::new(0));

    // let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
         let counter = Arc::clone(&counter);

        // let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;   

        });
            handles.push(handle);
    
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result count-> {}", *counter.lock().unwrap());


    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     println!("{:?}", num);
    //     *num = 1;
    // }
    // println!("{:?}", m);
}