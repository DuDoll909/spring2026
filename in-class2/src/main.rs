use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

// fn main() {
//     let mut handles= vec![];
//     for i in 1..=3{
//         let handle = thread::spawn(move ||{
//         println!("Thread {}",i);
//     });
//     handles.push(handle);
//     }

//     for handle in handles{
//         handle.join().unwrap();
//     }
//     println!("All threads completed!");
//     }


fn main(){
    let total = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _i in 0..5{
        let cnt = total.clone();
        let handle = std::thread::spawn(move ||{
            for _j in 0..10{
                *cnt.lock().unwrap() +=1;
            }
        });
        handles.push(handle);
    };


for handle in handles{
    handle.join().unwrap();
}

    println!("Result: {}", *total.lock().unwrap());
}
