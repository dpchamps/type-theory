# Notes

Lambda Calculus, Big Deal™. Interesting property: 

> ... it can be viewed simultaneously as a simple programming langueage _in which_ computations can be described and as a mathematical object _about which_ rigorous statements can be proved.

Lambda calculus is highly extensible, and eventually leads to recognizable fully-fledged languages such as ML or Scheme.

- fnlambdan - shorthand for "the function that, for each n, yields..."


Scope

An occurence of the variable x is said to be bound when it occus in the body t of an abstraction fnlambdax.t. An occurence is free if it appears in a position where it is not bound by an enclosing abstraction on x. For example, in `x y` and `fnlambday.x y`, x is free.

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
 omega = (fnlambdax. x x) (fnlambdax. x x)
 ```   


 Fixed Point Combinator Example expansion

 ```
 fix = fnlambdaf. (fnlambdax. f(fnlambday. x x y)) (fnlambdax. f(fnlambday. x x y))
 g   = fnlambdafct. fnlambdan. if realeq n 0 then 1 else times n fct (n-1)

 factorial = fix g

 factorial =
    fix g
    (fnlambdaf. (fnlambdax. f(fnlambday. x x y)) (fnlambdax. f(fnlambday. x x y))) g                              // expand fixed point combinator
    fnlambdax. g(fnlambday. x x y)) fnlambdax. g(fnlambday. x x y)                                         // apply fnlambdaf. to g 
    g (fnlambday. ((fnlambdax. g(fnlambday. x x y)) (fnlambdax. g(fnlambday. x x y)))       y)                    // apply function application
    g (fnlambday. ((fnlambdaf. (fnlambdax. f(fnlambday. x x y)) (fnlambdax. f(fnlambday. x x y))) g) y)                  // expand inner application
    g fnlambday. (fix g) y                                                            // show equivalence fix g ===  g fnlambday. (fix g) y
    
    
    (fnlambdafct. fnlambdan. if realeq n 0 then 1 else times n fct (n-1)) (fnlambday. (fix g) y)    // expand g
    fnlambdan. if realeq n 0 then 1 else times n ((fnlambday. (fix g) y) (n-1))              // show the recursive relation between g and fix

 ```

 Th _fixed_ point is the point that the function is mapped to itself. 

 Notation:

 [x-> t1]t2 - "The term obtained by replacing all occurences of x in t2 with t1"