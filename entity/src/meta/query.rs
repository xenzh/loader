use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Debug)]
pub enum Value {
    Str(String),
    Int(isize),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Value::Str(ref s) => s.fmt(f),
            Value::Int(ref i) => i.fmt(f),
        }
    }
}


#[derive(Debug)]
pub enum Set {
    Str(Vec<String>),
    Int(Vec<isize>),
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Set::Str(ref v) => format_vec(v, f),
            Set::Int(ref v) => format_vec(v, f),
        }
    }
}


type OpRef = usize;

#[derive(Debug)]
enum Op<Arg> {
    Is(Arg, Value),
    In(Arg, Set),

    Not(OpRef),
    Or(OpRef, OpRef),
    And(OpRef, OpRef),
}


#[derive(Debug)]
pub struct Expr<Arg> {
    ops: Vec<Op<Arg>>,
}

impl<Arg> Expr<Arg> {
    pub fn new() -> Expr<Arg> {
        Expr { ops: Vec::new() }
    }

    pub fn root(&self) -> Option<OpRef> {
        if self.ops.is_empty() { None }
        else { Some(self.ops.len() - 1) }
    }

    pub fn is(&mut self, arg: Arg, val: Value) -> OpRef {
        self.push(Op::Is(arg, val))
    }

    pub fn _in(&mut self, arg: Arg, set: Set) -> OpRef {
        self.push(Op::In(arg, set))
    }

    pub fn not(&mut self, op: OpRef) -> OpRef {
        self.push(Op::Not(op))
    }

    pub fn or(&mut self, lhs: OpRef, rhs: OpRef) -> OpRef {
        self.push(Op::Or(lhs, rhs))
    }

    pub fn and(&mut self, lhs: OpRef, rhs: OpRef) -> OpRef {
        self.push(Op::And(lhs, rhs))
    }

    fn push(&mut self, op: Op<Arg>) -> OpRef {
        self.ops.push(op);
        self.ops.len() - 1
    }
}

impl<Arg: Display> Display for Expr<Arg> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.root() {
            None => write!(f, "(empty)"),
            Some(op) => format_op(&self.ops[op], &self.ops, f),
        }
    }
}


//formatters

fn format_vec<T: Display>(v: &Vec<T>, f: &mut Formatter) -> FmtResult {
    write!(f, "[")?;
    let mut it = v.iter().peekable();
    while let Some(ref e) =it.next() {
        if it.peek().is_some() { write!(f, "{}, ", e)?; }
        else { write!(f, "{}", e)?; };
    }
    write!(f, "]")
}

fn format_op<Arg: Display>(op: &Op<Arg>, ops: &Vec<Op<Arg>>, f: &mut Formatter) -> FmtResult {
    let format_idx = |idx: OpRef, f: &mut Formatter| format_op(&ops[idx], ops, f);

    match *op {
        Op::Is(ref a, ref v) => write!(f, "({} == {})", a, v),
        Op::In(ref a, ref s) => write!(f, "({} in {})", a, s),
        Op::Not(o) => {
            write!(f, "(not ")?;
            format_idx(o, f)?;
            write!(f, ")")
        },
        Op::Or(l, r) => {
            write!(f, "(")?;
            format_idx(l, f)?;
            write!(f, " || ")?;
            format_idx(r, f)?;
            write!(f, ")")
        },
        Op::And(l, r) => {
            write!(f, "(")?;
            format_idx(l, f)?;
            write!(f, " && ")?;
            format_idx(r, f)?;
            write!(f, ")")
        },
    }
}


#[derive(Debug)]
pub struct Query<Subject, Object> {
    get: Subject, // artist, album
    from: Object, // recenttrack, year
    cond: Expr<(Subject, Object)>, // artist.name == "x"
}

// map subject->[object]: identifier (request)
// map subject->[object]: provides (response)


#[cfg(test)]
mod tests {
    use std::fmt::*;
    use super::{Value, Set, Expr};

    #[derive(Debug)]
    enum Arg {
        ArtistName,
        AlbumYear,
        AlbumTags,
    }

    impl Display for Arg {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "{}", match *self {
                Arg::ArtistName => "artist.name",
                Arg::AlbumYear => "album.year",
                Arg::AlbumTags => "album.tags",
            })
        }
    }

    #[test]
    fn sample_expression() {
        let mut e = Expr::<Arg>::new();

        let name = e.is(Arg::ArtistName, Value::Str("corpseeyedtoads".to_owned()));
        let year1 = e.is(Arg::AlbumYear, Value::Int(2015));
        let year2 = e.is(Arg::AlbumYear, Value::Int(2018));
        let tags = e._in(Arg::AlbumTags, Set::Str(vec!["pop".to_owned(), "synth".to_owned()]));

        let years = e.or(year1, year2);
        let not_tags = e.not(tags);
        let years_tags = e.and(years, not_tags);

        let _ = e.and(name, years_tags);
        println!("Expression: {}", e);
    }
}
