use std::thread;
use std::time::Duration;

fn main() {
    // create a new thread that will run its closure indpenedently of the process
    // running the main thread. Soawning the thread returns a handle that can be
    // to force the main thread to wait for the spawned thread to finish.
    let handle_1 = thread::spawn(|| {
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
    handle_1.join().unwrap();

    // threads run closures within the scope of the thread meaning variables
    // created in the main thread cannot be used in spawned threads by default
    let v = vec![1, 2, 3];

    // using the move keyword though allows us to force the threads closure
    // to take ownership of the variables it needs.
    let handle_2 = thread::spawn(move || {
        println!("Second spawned threads vector: {:?}", v);
    });

    handle_2.join().unwrap();
}
