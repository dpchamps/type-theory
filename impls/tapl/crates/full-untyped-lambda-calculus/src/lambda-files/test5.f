let tru = λt. λf. t in
let fls = λt. λf. f in
let pair = λf. λs. λb. b f s in
let fst = λp. p tru in
let snd = λp. p fls in
let realbool = λb.b true false in
realbool (fst (pair tru fls));