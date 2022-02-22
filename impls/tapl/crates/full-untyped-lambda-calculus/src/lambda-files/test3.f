let tru = λt. λf. t in
let fls = λt. λf. f in
let realbool = λb.b true false in

realbool tru;