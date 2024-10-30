use std::sync::mpsc;
use std::thread;

fn main(){
    //let (tx, rx) = mpsc::channel();

    //thread::spawn(move || {
    //    let val = String::from("hi");
    //    tx.send(val).unwrap();
    //});

    //let recieved = rx.recv().unwrap();

    //println!("Got {recieved}");
    let ans: u64 = soln();
    println!("The Sum from 0 -> 10^8 is {}", ans);
}


// Multithread program to calculate sum from 1 - 10^8

fn soln() -> u64 {
    let (tx, rx) = mpsc::channel();
    
    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in 0..10000000 {
                ans = ans + (i*10000000 + j);
            }
            producer.send(ans).unwrap();
        });
    }
    // The reciever doesnt know when the threads finish 
    // and it waits till infinity for recieving data
    // so we explicitly drop tx
    drop(tx);
    let mut ans: u64 = 0;
    for val in rx{ 
        ans = ans + val;
        println!("Val found");
    }
    ans
}
