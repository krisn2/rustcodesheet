use std::thread::spawn;
pub fn test_threads(){
    let mut x =0u128;
    for i in 1..500 {
        x += i;
    }
    println!("Main thread finished a little bit of work")
}

pub fn spawn_threads(){
   let thread_fn = || {
    let mut x =0u128;
    for i in 1..500_000 {
        x += i;
    }
    println!("val:{x}");
   };
   println!("Strating new worker thread..");
   let handle = spawn(thread_fn);
   let handle2 = spawn(thread_fn);
   println!("Worker thread finished a  work");

   loop {
       test_threads();
       if handle.is_finished() && handle2.is_finished() {
           println!("All threads finished");
           break;
       }
   }
}