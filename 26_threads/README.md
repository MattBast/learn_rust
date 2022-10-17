# Concurrency: threads
By default Rust runs code linearly. That is it does one thing at a time before moving onto the next task. However we may want some independent parts to run at the same time to improve performance. For example a web server may want to process multiple threads at the same time to enable the processing of requests at the same time. This is called concurrency where each task runs on its own thread. This improves performance but also increases complexity.

Some complexities to consider are:

- **Race conditions:** threads are accessing data or resources in an inconsistent order
- **Deadlock:** two or more threads are waiting for each other preventing each other from executing
- **Bugs:** that are very difficult to reproduce and fix