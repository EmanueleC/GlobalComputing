# GlobalComputing
## Repository for Global Computing course at University of Padua

### Exercises

From "Reactive systems" (L. Aceto, A. Ingolfsdottir, K.G. Larsen, J. Srba)

* Show that trace equivalence is a congruence for CCS.

* Ex. 3.9 Prove that string bisimilarity and strong bisimilarity coincide.

### Project

#### Prisoner's Game

50 prisoners kept in separate cells got a chance to be released: From time to time one of them will be carried in a special room (in no particular order, possibly multiple times consecutively, but with a fair schedule to avoid infinite wait) and then back to the cell.

The room is completely empty except for a switch that can turn the light either on or off (the light is not visible from outside and cannot be broken). At any time, if any of them says that all the prisoners have already entered the room at least once and this is true, then all prisoners will be released (but if it is false, then the chance ends and they will never be released). Before the challenge starts, the prisoners have the possibility to discuss together some "protocol" to follow.

Can you find a winning strategy for the prisoners?
Note that the initial state of the light in the room is not known. 

### Rust Section

Rust, a multiparadigm language with ownership

How does Rust prevent data races? ... and more

https://www.rust-lang.org/en-US/

https://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html
