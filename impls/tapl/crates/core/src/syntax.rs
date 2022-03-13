pub trait Shift {
    fn shift_n(&self, d: i32, c: i32) -> Self
    where
        Self: Sized+Clone 
    {
        self.clone()
    }

    fn shift(&self, d: i32) -> Self
    where
        Self: Sized+Clone,
    {
        self.shift_n(d, 0)
    }
}

pub trait Substitute {
    fn substitute(&self, j: i32, s: &Self) -> Self 
    where Self: Sized + Clone
    {
        self.clone()
    }

    fn substitute_top(&self, s: &Self) -> Self
    where
        Self: Sized + Shift + Clone,
    {
        self.substitute(0, &s.shift(1)).shift(-1)
    }
}

pub trait Term: PartialEq + Clone + Shift + Substitute {}