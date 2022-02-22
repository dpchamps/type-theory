let czero = λs. λz. z in 
let tru = λt. λf. t in
let fls = λt. λf. f in
let and = λb. λc. b c fls in
let scc = λn. λs. λz. s (n s z) in
let plus = λm. λn. λs. λz. m s (n s z) in
let times = λm. λn. m (plus n) czero in

let pair = λf. λs. λb. b f s in
let fst = λp. p tru in
let snd = λp. p fls in

let iszro = λm. m (λx. fls) tru in
let zz = pair czero czero in
let ss = λp. pair (snd p) (plus (scc czero) (snd p)) in
let prd = λm. fst (m ss zz) in
let equal = λm. λn. and (iszro (m prd n)) (iszro (n prd m)) in 
let Y = λf. (λx. f(λy. x x y)) (λx. f(λy. x x y)) in
let cn = λfn. λn. if iszero n then czero else scc (fn (-n)) in
let churchnat = Y cn in
let realeq = λm. λn. (equal m n) true false in
let realnat = λm. m (λx. + x) 0 in
let realbool = λb.b true false in 
let fct = λfn. λn. if realeq n czero then (scc czero) else (times n (fn (prd n))) in
let factorial = Y fct in
realnat (factorial (churchnat 5));