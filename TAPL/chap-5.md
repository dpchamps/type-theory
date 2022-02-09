# Notes

Lambda Calculus, Big Deal™. Interesting property: 

> ... it can be viewed simultaneously as a simple programming langueage _in which_ computations can be described and as a mathematical object _about which_ rigorous statements can be proved.

Lambda calculus is highly extensible, and eventually leads to recognizable fully-fledged languages such as ML or Scheme.

- λn - shorthand for "the function that, for each n, yields..."


Scope

An occurence of the variable x is said to be bound when it occurs in the body t of an abstraction λx.t. An occurence is free if it appears in a position where it is not bound by an enclosing abstraction on x. For example, in `x y` and `λy.x y`, x is free.

A term with no free variables is said to be closed, and are also called combinators.

redex: Reducible Expression

beta-reduction: the operation of rewriting a redex into normal form

Strategies for evaluation

- Full beta reduction
    Any redex may be reduced at any time. At each step, pick some redex anywhere inside the term being evaluated and reduce it.
- Normal order
    The leftmost, outermost redex is always reduced first
- Call by name
    Does not allow for reductions inside abstractions. 
- Call by value
    Only outermost redexes are reduces and where a redex is reduced only when its right-hand side has already been reduced to a value (a term that is finished computing and cannot be reduced any further)

 Divergence: terms with no normal form are said to diverge. 

 For examples:

 ```
 omega = (λx. x x) (λx. x x)
 ```   


 Fixed Point Combinator Example expansion

 ```
 fix = λf. (λx. f(λy. x x y)) (λx. f(λy. x x y))
 g   = λfct. λn. if realeq n 0 then 1 else times n fct (n-1)

 factorial = fix g

 factorial =
    fix g
    (λf. (λx. f(λy. x x y)) (λx. f(λy. x x y))) g                              // expand fixed point combinator
    λx. g(λy. x x y)) λx. g(λy. x x y)                                         // apply λf. to g 
    g (λy. ((λx. g(λy. x x y)) (λx. g(λy. x x y)))       y)                    // apply function application
    g (λy. ((λf. (λx. f(λy. x x y)) (λx. f(λy. x x y))) g) y)                  // expand inner application
    g λy. (fix g) y                                                            // show equivalence fix g ===  g λy. (fix g) y
    
    
    (λfct. λn. if realeq n 0 then 1 else times n fct (n-1)) (λy. (fix g) y)    // expand g
    λn. if realeq n 0 then 1 else times n ((λy. (fix g) y) (n-1))              // show the recursive relation between g and fix

 ```

 Th _fixed_ point is the point that the function is mapped to itself. 

 Notation:

 [x-> t1]t2 - "The term obtained by replacing all occurences of x in t2 with t1"