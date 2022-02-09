# Fixed Point Combinators for Babies

 _aka understanding the Y Combinator_

 ## Lightning Round

 Lambda calculus is a system that expresses computation. It's symbolic, terse and has surprisingly few rules.

 The notation is simple: the symbol λ is to be read as "the function that yields...".  Variables are introduced as λx., which is read "the function that, for each x, yiels..."

 There can be _free_ and _bound_ variables. 

 In math, free and bound variables are often expressed:

 ``` 
let f(x) = x^n
 ```

 Where x is _bound_ to the function `f` whereas n is _free_.

We could express two more functions

```
let g(y) = y^n
let h(z) = z^m
```

Where f and g exhibit equality and h is different because the two free variables may be different.

### Combinators

A term with no free variables is said to be _closed_, or also a Combinator.

### Beta Reduction

A term that is\ expressed in a form that can be reducible are known as a redex, e.g. (λx.x)y.

Rewriting that term via substition is called beta-reduction, e.g. λy.

A term of which beta reduction cannot be applied to is said to be in  "Normal Form"

### Divergence

Terms with no normal form are said to _diverge_. For example, 

```
omega = (λx. x x)(λx. x x)

((λx. x x) (λx. x x)) // apply substitution, etc...
```

# `λf. (λx. f(λy. x x y)) (λx. f(λy. x x y))`


The simple rules of lambda calculus do not allow for a function to perform application _to itself_.

The Y-Combinator is a divergent combinator that can be used to express recursive functions.

Let's consider an extremly simple recursive function:

```
factorial n:
    if n = 0 return 1
    else n * factorial(n-1)
```

If we were to look at the expansion of the above definition

```
factorial n:
    if n = 0 return 1
    else n * (if n-1 = 0 return 1
              else n-1 * (if n-2 = 0 return 1
                          else n-2 * (if n-3 return 1
                                      else n-3 * ...)
                          )
              )  
```


Observe the repetitive structure of the above expansion. We resume the outer function at a _fixed point_.

let's define this in terms of the lambda calculus:

```
g = λfn. λn. if eq n 0 then 1 else times n fn n-1
      ^-------------------------------------^
    This function must recieve a function that is _itself_  
```

Now let's observe some properties of the fixed point combinator:

```
Y = λf. (λx. f(λy. x x y)) (λx. f(λy. x x y))
g = λx.x

Y g
  (λf. (λx. f(λy. x x y)) (λx. f(λy. x x y))) g
  λx. g(λy. x x y) λx. g(λy. x x y)
  g(λy. λx. g(λy. x x y) λx. g(λy. x x y) y)
        --------------------------------- Study this structure
  g(λy. (λx. g(λy. x x y) λx. g(λy. x x y)) y)      
  g(λy. ((λf. (λx. f(λy. x x y)) (λx. f(λy. x x y))) g) y)
  g(λy. (Y g) y)
```

What we can see by applying beta-reduction to the `Y g` terms is that we get a repetitive structure where `g` is eventually applied to `Y g`.

Let's now revisit the factorial function:

```
g = λfn. λn. if eq n 0 then 1 else times n fn n-1
```

And finally we'll consider

```
factorial = Y g
```

and apply some creative expansion

```
Y g
g(λy. (Y g) y)
g(λy. (g(λy. (Y g) y)) y)
g(λy. (g(λy. (g(λy. (Y g) y)) y)) y)
```

and then apply some creative whitespace

```
g(
    λy. ( 
            g (
                λy. (
                        g
                            (
                                λy. (...) y
                            )
                    ) y
            )
        ) y
)


(λfn. λn. if eq n 0 then 1 else times n fn n-1)(
    λy. ( 
            (λfn. λn. if eq n 0 then 1 else times n fn n-1) (
                λy. (
                        (λfn. λn. if eq n 0 then 1 else times n fn n-1)(
                            λy. (...) y
                        )
                    ) y
            )
        ) y
)
```


Here we can see the divergent property of the Y combinator suits the repititive expansion of the factorial function. For every time n is not equal 0, we need _something_ to expand with one less than n. This structure is generalized. For example, a fibonacci function:

```
fibonacci n = 
    if n < 2 return 1
    return fibonacci n-1 + fibonacci n-2
```

Or expressed in lambda calculus:

```
g = λfn. λn. if lessthan n 2 then 1 else add (fn n-1) (fn n-2)
fibonnaci = Y g
```



