use std::thread;
use std::time::Duration;

fn main(){
    // This spawn a thread and is performed seperately
    thread::spawn( || {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("Hi number {i} from the Handle thread");
            thread::sleep(Duration::from_millis(1));
        }
    });


    // We wait till the spawn thread finish before going to main thread
    handle.join().unwrap();
    // we use the move keyword because the closure will take the 
    // ownership of the values it uses from the envirnoment thus transferring
    // the ownership of the values from one thread to another
    {    
        let v = vec![1,2,3,4];
        // move moves the ownership to that thread
        thread::spawn(move || {
            println!("{:?}", v);
        });
    }


    // This is done on the main thread
    for i in 1..5 {
        println!("Hi number {i} from the main Thread");
        thread::sleep(Duration::from_millis(1));
    }
}
