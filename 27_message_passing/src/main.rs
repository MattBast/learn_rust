// import the "multiple producer, single consumer" channel. As the name suggests,
// this type of channel can have multiple producers transmit messages to it but
// only one consumer receiving messages from it.
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // tx is used to transmit data to a channel. rx is used
    // to receive data from it
    let (tx, rx) = mpsc::channel();

    // use the move keyword to move ownership from the main thread to
    // the spawned thread
    thread::spawn(move || {
        let val = String::from("hi");
        // and send the string value to the channel.
        // the send command returns a `Result<T, E>` enum. If this were
        // production code this would need to be handled. In the meantime
        // unwrap is used to get the value from the Ok type or to panic
        tx.send(val).unwrap();

        // variables transmitted to a channel move ownership of the variable to
        // the channel. This means the value cannot be used again after it has
        // been transmitted.
    });

    // use the `recv()` method to get a Result<T, E> enum from the channel. 
    // `recv()` is a blocking method (that is it will wait for a message to
    // appear on the channel). The `try_recv()` method could also have been 
    // used here which is none blocking and can be used to poll a channel for
    // messages while the programme gets on with something else.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // to show that the receiver will block for the lifetime of the transmitting
    // thread here's an example where a spawned thread sends mutliple messages.
    let (tx, rx) = mpsc::channel();

    // to produce messages from multiple transmitters, clone the transmitter
    let tx_cloned = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("second"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("some"),
            String::from("more"),
            String::from("messages"),
        ];

        for val in vals {
            tx_cloned.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for message in rx {
        println!("Got: {}", message);
    }
}
