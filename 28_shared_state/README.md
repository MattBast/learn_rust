# Concurrency: shared state concurrency
Message passing is but one way of sharing data between independent threads. Shared state concurrency (or communicating with shared memory as it's known in other languages) is one such alternative. It's advantage over message passing is that it allows for something similar to multiple ownership of a varaible. While message passing says you cannot use a value once it's put into a channel, shared ownership has no such restriction. It however is a little more complex.

## Mutexes
A mutex is a good example of shared state concurrency. It stands for mutual exclusion whereby a piece of data can be accessed by multiple threads. When a thread wants access it acquires a lock for the data so no other thread can access the value while thread 1 has access. When thread 1 is done, it releases the lock so others can access the data.

Locking and unlocking values is usually very tricky to get right which is why many adopt message passing instead. Hopever Rusts ownership and type system handles much of this trickiness for you.

**Note** that mutexes and shared state concurrency generally is subject to deadlocking which can be tricky to handle. There are many deadlock strategies that can be researched on Google though :). 