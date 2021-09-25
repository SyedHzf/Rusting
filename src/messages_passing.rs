use std::{sync::mpsc,thread,time::Duration};
pub fn run(){
    let (tx,rx) = mpsc::channel();

    thread::spawn(move ||{
        let msg = vec![
        String::from("Hi"),
        String::from("Dear!"),
        String::from("How Are You?"),
        String::from("Please reply"),
        ];
        for i in msg{
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
            
    }}
    );

    // let received = rx.recv();
    for received in rx{
        println!("Received {:?}", received);

    }
}