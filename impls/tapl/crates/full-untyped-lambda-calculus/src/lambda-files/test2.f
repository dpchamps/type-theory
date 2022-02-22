let tru = λt. λf. t;
let fls = λt. λf. f;
let realbool = λb.b true false;

realbool tru;