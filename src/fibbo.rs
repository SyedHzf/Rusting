pub fn fibbonacci(num: u32) {
    let mut n1 = 0;
    let mut n2 = 1;

    if num == 1 {
        print!("{}",n1);
    }else if num == 2{
        print!("{}",n2);
    }else{
        print!("{},{},",n1,n2);
        for _i in 0..num{
            let n3 = n1 + n2;
            n1 = n2 ;
            n2 = n3;
            print!("{},",n3);
        }
    }
}