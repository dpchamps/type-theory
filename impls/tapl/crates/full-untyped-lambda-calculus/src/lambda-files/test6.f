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
let Y = λf. (λx. f(λy. x x y)) (λx. f(λy. x x y)) ;
let cn = λfn. λn. if iszero n then czero else scc (fn (-n)) ;
let churchnat = Y cn ;
let realeq = λm. λn. (equal m n) true false ;
let realnat = λm. m (λx. + x) 0 ;
let realbool = λb.b true false ; 
let fct = λfn. λn. if realeq n czero then (scc czero) else (times n (fn (prd n))) ;
let factorial = Y fct ;

realnat (factorial (churchnat 5));