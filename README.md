# learning_rust

A repository of rust projects and small tools that I have written for whatever reason. They likely have a point and cover a new topic, but if they aren't self explanatory I'll include a description below or in the dir.

### reader
A quick script that basically just reads a file line by line. I assume it doesn't load everything into memory at once, works like a stream instead. Single threaded but loops through rockyou nice and quickly (so far)!

### generator
Uses the rand crate to create 1000 (hardcoded) randomly generated strings. Each iteration of the generator uses the current iteration value as it's length, i.e 1, 2, 3, 4... 1000. As I am just using the rand library I am solely relying on the rng engine to do this for me.

### recurse
@h0mbre_ put out a tweet about attempting to recursively obtain a process and all it's children whilst keeping track of the level of recursion (depth?) for each identified process. I tried to achieve this using recursion and then maintaing a vector of nodes (sort of) with knowledge of their pid, their level of recursion and their parent pid. 
  
### License  
I'm just a simple skid. Licensing isn't a big issue to me, I post things that I find helpful online in the hope that others can:  
 A) learn from the code  
 B) find use with the code or   
 C) need to just have a laugh at something to make themselves feel better  
  
Either way, if this helped you - cool :)  
