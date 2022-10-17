use std::thread;
use std::time::Duration;

fn main() {
    // create a new thread that will run its closure indpenedently of the process
    // running the main thread. Soawning the thread returns a handle that can be
    // to force the main thread to wait for the spawned thread to finish.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // when the main thread ends the entire programme ends meaning the spawned
    // thread may not be able to complete before the main thread shuts it down
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // so to stop the main thread shutting down the spawned thread we can use
    // the spawned threads handle to make the main thread wait for the
    // spawned thread to finish.
    handle.join().unwrap();
}
