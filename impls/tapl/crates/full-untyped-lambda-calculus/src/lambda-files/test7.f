let czero = λs. λz. z ; 
let tru = λt. λf. t ;
let fls = λt. λf. f ;
let and = λb. λc. b c fls ;
let scc = λn. λs. λz. s (n s z) ;
let plus = λm. λn. λs. λz. m s (n s z) ;
let times = λm. λn. m (plus n) czero ;
let pair = λf. λs. λb. b f s ;
let fst = λp. p tru ;
let snd = λp. p fls ;
let iszro = λm. m (λx. fls) tru ;
let zz = pair czero czero ;
let ss = λp. pair (snd p) (plus (scc czero) (snd p)) ;
let prd = λm. fst (m ss zz) ;
let equal = λm. λn. and (iszro (m prd n)) (iszro (n prd m)) ; 
let leq = λm. λn. iszro (n prd m);
let Y = λf. (λx. f(λy. x x y)) (λx. f(λy. x x y)) ;
let cn = λfn. λn. if iszero n then czero else scc (fn (-n)) ;
let churchnat = Y cn ;
let realeq = λm. λn. (equal m n) true false ;
let realLeq =  λm. λn. (leq m n) true false ;
let realnat = λm. m (λx. + x) 0 ;
let realbool = λb.b true false ; 


let fib = λfn. λn. if realLeq n (scc (scc czero)) then (scc czero) else (plus (fn (prd n)) (fn (prd (prd n)))) ;
let fibonnaci = Y fib;

realnat (fibonnaci (churchnat 1));
realnat (fibonnaci (churchnat 2));
realnat (fibonnaci (churchnat 3));
realnat (fibonnaci (churchnat 4));
realnat (fibonnaci (churchnat 5));
realnat (fibonnaci (churchnat 6));
realnat (fibonnaci (churchnat 7));
realnat (fibonnaci (churchnat 8));