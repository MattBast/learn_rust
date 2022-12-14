use std::{ 
	sync::{ mpsc, Arc, Mutex }, 
	thread 
};

/// A thread pool is used to reserve threads on the host machine
/// for the web server as well as limit how many threads it can use.
pub struct ThreadPool {
	/// workers is a list of the threads in the pool
	workers: Vec<Worker>,
	/// sender is the producer part of a channel used to send closures
	/// to a thread/worker that then executes the closure
	sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
	/// Create a new ThreadPool.
	///
	/// The size is the number of threads in the pool.
	// 
	// # Panics
	///
	/// The `new` function will panic is the size 0 or less.
	pub fn new(size: usize) -> ThreadPool {
		// check that the thread pool is instantiated with more than 0
		// threads. Panic if not.
		assert!(size > 0);

		// create a channel to send closures to execute to the threads
		let (sender, receiver) = mpsc::channel();

		// Create a receiver that we can clone and grant a copy of to each thread/worker.
		// The Mutex ensures that only one worker receives a job. No jobs executions are 
		// duplicated.
		let receiver = Arc::new(Mutex::new(receiver));

		// specifying the size of the threads vec saves us some memory as well
		// giving us a bit of a performance boost. It does however mean the size
		// of this vector cannot grow or shrink.
		let mut workers = Vec::with_capacity(size);

		// spawn and store a list of threads/workers for the pool to use
		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		return ThreadPool { workers, sender: Some(sender) };
	}

	/// Use a thread in the thread pool to run a closure.	
	pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static, {
		// This function uses a thread in the pool to execute a custom closure function.
		// It uses the F type to represent the closure function. It also specifies the 
		// FnOnce trait which tells us that the closure will only be executed once. We also 
		// add Send which allows us to move the closure from one thread to another and the 
		// 'static lifetime because we don't know how long the closure will take to execute.

		// put the closure function received in a generic box
		let job = Box::new(f);

		// use the pools channel to send the job to a worker
		self.sender.as_ref().unwrap().send(job).unwrap();
	}
}

/// The Drop trait for ThreadPool allows all workers to finsih processing their
/// current job before shutting them all down.
impl Drop for ThreadPool {
	fn drop(&mut self) {
		// shut down down the sender so no new jobs are sent to the workers
		drop(self.sender.take());

		// shut down the workers as they finish their jobs
		for worker in &mut self.workers {
			println!("Shutting down worker {}", worker.id);
			
			// take ownership of the threads handle
			if let Some(thread) = worker.thread.take() {
				// then call the join method to shut down the thread
				thread.join().unwrap();
			}
		}
	}
}

/// A wrapper for a thread in a thread pool. Is used to execute closures provided
// by the pool.
struct  Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	/// creates a worker with a unique id and a thread with an empty closure
	pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		// start a loop within a new thread waiting for a job to come down
		// the message passing channel.
		let thread = thread::spawn(move || loop {
			let message = receiver.lock().unwrap().recv();

			// wait to receive a job. Shut down the worker if it discnnects
			// from the jobs channel
			match message {
				Ok(job) => {
					println!("Worker {id} received a job. Currently executing job.");
					job();
				}
				Err(_) => {
					println!("Worker {id} disconnected. Shutting it down.");
					break;
				}
			}
			
		});

		return Worker { id, thread: Some(thread) };
	}
}

// represents a closure to be run on a thread/worker. We use a
// type aias instead of a struct as we just need a generic type
// with no methods.
type Job = Box<dyn FnOnce() + Send + 'static>;