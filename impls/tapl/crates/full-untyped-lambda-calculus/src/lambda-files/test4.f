let tru = λt. λf. t ;
let fls = λt. λf. f ;
let pair = λf. λs. λb. b f s ;
let fst = λp. p tru ;
let snd = λp. p fls ;
let realbool = λb.b true false;
realbool (snd (pair tru fls));