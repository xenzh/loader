use std::fmt::Debug;


#[derive(Debug)]
pub enum Object {
    Track,
    Album,
    Artist,
    Tag,
}


#[derive(Debug)]
pub enum Subject {
    Name,
    Artist,
    ReleaseDate,
    Tracks,
    Mbid,
    Duration,
}

#[derive(Debug)]
pub struct Target {
    obj: Object,
    subj: Subject,
}


#[derive(Debug)]
pub enum Val<T: Debug> {
    Required(T),
    Optional(T),
}

#[derive(Debug)]
pub enum Fld<T: Debug> {
    Primitive(Val<T>),
    PrimitiveList(Val<T>),
    Complex(Val<Vec<Fld<T>>>),
    ComplexList(Val<Vec<Fld<T>>>),
}

pub trait Method : Debug {
    fn name(&self) -> &str;
    fn object(&self) -> Object;
    fn req(&self) -> &Fld<Subject>;
    fn rsp(&self) -> &Fld<Subject>;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_album_meta() {
        let req: Fld<Subject> = Fld::Complex(Val::Required(vec![
            Fld::Primitive(Val::Required(Subject::Artist)),
            Fld::Primitive(Val::Required(Subject::Name)),
            Fld::Primitive(Val::Optional(Subject::Mbid)),
        ]));

        let rsp: Fld<Subject> = Fld::Complex(Val::Required(vec![
            Fld::Primitive(Val::Required(Subject::Name)),
            Fld::ComplexList(Val::Required(vec![
                Fld::Primitive(Val::Required(Subject::Name)),
                Fld::Primitive(Val::Optional(Subject::Duration)),
            ])),
        ]));

        println!("\nreq: {:?}\n\nrsp: {:?}\n", req, rsp);

    }
}
