# rForth
A Forth interpreter written in rust
# TODO
- add branching word 
  - `branch` (simple jump )  
  - `branch?` (conditional jump depending on the last value on the stack)  
- add `immediate` word (if word is `immediate` then executed at compile time else interpreted time)
example from [here](https://www.forth.com/starting-forth/11-forth-compiler-defining-words/) :
```
To give an immediate example, let’s define

: SAY-HELLO   ." Hello" ; IMMEDIATE 

We can execute SAY-HELLO interactively, just as we could if it were not immediate.

SAY-HELLO↵ Hello ok 

But if we put SAY-HELLO inside another definition, it will execute at compile time:

: GREET   SAY-HELLO ." I speak Forth " ;↵ Hello ok  

rather than at execution time:

GREET↵ I speak Forth ok 

```
# Disclaimer
> I'm not a pro rust programmer neither a pro forth programmer  
> if the code is aweful please give me some tips :)  

## restart
Need to restart with a cleaner code base
with a get_next() function instead of a whole function comparing each words
