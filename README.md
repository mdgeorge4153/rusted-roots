Rooty
=====

This project aims to reflect basic algebraic structures in the Rust type system.

This is a project I'm developing to help me learn Rust.  It has similar goals
and design to [alga](https://docs.rs/alga/0.9.3/alga/).  See also my
[explorations of the same problem in other languages](https://github.com/mdgeorge4153/algebra)

Here are my desiderata:

 - for number type implementors: it should be easy to add new group, ring,
   field implementations, and to automatically generate quickcheck-style tests.
   The documentation should clearly include exactly what must be implemented
   and what properties those implementations require.

 - for number type users: it should be easy to adapt existing number libraries
  

 - for number type users: it shouldn't be much harder to write code that is
   generic with respect to number types.  In particular, it should be possible
   to use + and * to manipulate elements of a ring.


 - for the library design: the trait definitions should be easy to work with
   and clearly translate to standard mathematical language.  For example, it
   should be easy to look at the Group trait and see "a Group is a Monoid with
   an inverse operation".  There should be a minimum of boilerplate.

 - for idiom design: the concepts and idioms should be generalizable, and where
   possible, standard rust idioms.  For example, it should be possible to adapt
   the partial order traits to work with set-like objects (such as planar regions).

 - maybe these designs could be helpful to people who want to integrate formal
   methods into the rust language.

