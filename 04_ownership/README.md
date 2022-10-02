# Ownership
Rust manages a computers memory through the concept of ownership. The basic idea is that declared variables will be "dropped" as soon as they go out of scope (e.g. the function they're in ends). This is in contrast with many other languages who perform regular "garbage collection" which looks for all unused varibales and drops them all at once (which can consume a lot of processing). Ownership makes for faster programs but does require the code to take account of it. 

Some things to remember while coding are:
- by default, variables will be dropped and will be unusable as soon as they go out of scope (look out for `}` characters :)).
- try not to pass variables into functions. Use references instead so that variables do not get dropped.