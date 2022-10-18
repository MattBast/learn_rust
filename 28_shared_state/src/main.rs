use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    basic_mutex();
    shared_ownership_mutex();
}

fn basic_mutex() {
    let m = Mutex::new(5);

    {
        // to access the mutexs value we need to acquire the mutexs lock.
        // this is a blocking method so the programme will wait for the lock
        // to be released if another thread has acquired it. If a thread with
        // the lock panics this method will also return an error.
        let mut num = m.lock().unwrap();
        
        // mutex is a smart pointer. To get the value from num we need to
        // Deref (de-reference) the value which can be achieved with the *
        // shortcut.
        *num = 6;

        // when the num variable goes out of scope the lock is automatically
        // released.
    }

    println!("m = {:?}", m);   
}

fn shared_ownership_mutex() {
    // mutexs like any other variable can only have one owner by default. So if
    // multiple threads are to use it concurrently we need to enable shared
    // ownership. We can use the Arc pointer for this purpose. Arc is very similar
    // to the Rc pointer except that it allows for multi-threading. It is less
    // performant though which is why we use Rc in single threaded applications.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // spawn ten threads
    for _ in 0..10 {
        // create an rc clone meaning the new thread can share ownership
        // with the other threads.
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // each thread increments the counter by one
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        // keep a list of handles so the main thread doesn't shut down the threads
        handles.push(handle);
    }

    // instruct the main thread to wait for the spawned threads to close before
    // ending and forcing them to shut down
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", * counter.lock().unwrap());
}