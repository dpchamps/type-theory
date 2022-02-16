pub type OnVarArgs<'a> = (i32, &'a FileInfo, &'a Var);

pub trait Visit {
    fn visit<F: Copy + Fn(OnVarArgs) -> Self>(&self, initial_container_size: i32, on_var: F) -> Self;
}

pub trait Shift {
    fn shift_n(&self, d: i32, c: i32) -> Self;
    fn shift(&self, d: i32) -> Self
    where 
        Self: Sized
    { 
        self.shift_n(d, 0)
    }
}

pub trait Substitute {
    fn substitute(&self, j: i32, s: &Self) -> Self;
    fn substitute_top(&self, s: &Self) -> Self 
    where
        Self: Sized + Shift
    {
        self.substitute(0, &s.shift(1)).shift(-1)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct FileInfo {
    filename: String,
    line_num: u32,
    line_col: u32
}   

impl FileInfo {
    pub fn new(filename: &str, line_num: u32, line_col: u32) -> Self {
        Self {
            filename: filename.into(),
            line_col,
            line_num
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Binding {
    NameBind,
    TermBind(Box<Term>)
}

impl Shift for Binding {
    fn shift_n(&self, d: i32, c: i32) -> Self {
        match self {
            Binding::NameBind => Binding::NameBind,
            Binding::TermBind(box term) => Binding::TermBind(box term.shift_n(d, c))
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Var {
    index: i32,
    container_size: i32
}

impl Shift for Var {
    fn shift_n(&self, d: i32, c: i32) -> Self {
        let index = match self.index {
            index if index >= c => self.index + c,
            _ => self.index
        };

        Var {
            index,
            container_size: self.container_size + d
        }
    }

    fn shift(&self, d: i32) -> Self {
        self.shift_n(d, 0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    String(FileInfo, String),
    Var(FileInfo, Var),
    True(FileInfo),
    False(FileInfo),
    If(FileInfo, Box<Term>, Box<Term>, Box<Term>),
    Let(FileInfo, String, Box<Term>, Box<Term>),
    Record(FileInfo, Vec<(String, Box<Term>)>),
    Projection(FileInfo, Box<Term>, String),
    Abstraction(FileInfo, String, Box<Term>),
    Application(FileInfo, Box<Term>, Box<Term>),
    Zero(FileInfo),
    Successor(FileInfo, Box<Term>),
    Predecessor(FileInfo, Box<Term>),
    IsZero(FileInfo, Box<Term>),
    Float(FileInfo, f32),
    TimesFloat(FileInfo, Box<Term>, Box<Term>)
}

impl Visit for Term {
    fn visit<F>(&self, initial_container_size: i32, on_var: F) -> Term 
    where
        F: Copy + Fn(OnVarArgs) -> Self
    {
        fn walk<F>(on_var: F, container_size: i32, term: &Term) -> Term 
        where F: Copy + Fn(OnVarArgs) -> Term
        {
            match term {
                Term::String(_, _) => term.clone(),
                Term::Var(file_info, var) => on_var((container_size, file_info, var)),
                Term::True(_) => term.clone(),
                Term::False(_) => term.clone(),
                Term::If(file_info, box t1, box t2, box t3) => 
                    Term::If(
                        file_info.clone(), 
                        box walk(on_var, container_size, t1), 
                        box walk(on_var, container_size, t2),
                        box walk(on_var, container_size, t3)
                    ),
                Term::Let(file_info, name, box t1, box t2) => 
                    Term::Let(
                        file_info.clone(),
                        name.clone(),
                        box walk(on_var, container_size, t1),
                        box walk(on_var, container_size+1, t2)
                    ),
                Term::Projection(file_info, box t1, l) =>
                    Term::Projection(
                        file_info.clone(),
                        box walk(on_var, container_size, t1),
                        l.clone()
                    ),
                Term::Record(file_info, fields) => 
                    Term::Record(
                        file_info.clone(),
                        fields.iter().map(|(field_name, field_term)| (String::from(field_name), box walk(on_var, container_size, field_term))).collect()
                    ),
                Term::Abstraction(file_info, name, box t2) => 
                    Term::Abstraction(file_info.clone(), name.clone(), box walk(on_var, container_size+1, t2)), 
                Term::Application(file_info, box t1, box t2) =>
                    Term::Application(
                        file_info.clone(), 
                        box walk(on_var, container_size, t1),
                        box walk(on_var, container_size, t2)
                    ),
                Term::Zero(_) => term.clone(),
                Term::Successor(file_info, box t1) => Term::Successor(file_info.clone(), box walk(on_var, container_size, t1)),
                Term::Predecessor(file_info, box t1) => Term::Predecessor(file_info.clone(), box walk(on_var, container_size, t1)),
                Term::IsZero(file_info, box t1) => Term::IsZero(file_info.clone(), box walk(on_var, container_size, t1)),
                Term::Float(_, _) => term.clone(),
                Term::TimesFloat(file_info, box t1, box t2) => 
                    Term::TimesFloat(
                        file_info.clone(),
                        box walk(on_var, container_size, t1),
                        box walk(on_var, container_size, t2)
                    )
            }
        }


        walk(on_var, initial_container_size, &self)
    }
}

impl Shift for Term {
    fn shift_n(&self, d: i32, c: i32) -> Term {
        self.visit(
            c, 
            |(c, file_info, var)| Term::Var(file_info.clone(), var.shift_n(c, d))
        )
    }
}

impl Substitute for Term {
    fn substitute(&self, j: i32, s: &Self) -> Self {
        self.visit(
            0,
            |(c, file_info, var)| {
                match var.index {
                    _ if var.index == j + c => s.shift(c),
                    _ => Term::Var(file_info.clone(), var.clone())
                }
            }
        )
    }
}