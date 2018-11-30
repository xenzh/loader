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
    Req(T),
    Opt(T),
}

#[derive(Debug)]
pub enum Fld<T: Debug> {
    Unit(Val<T>),
    UnitList(Val<T>),
    Composite(Val<Vec<Fld<T>>>),
    CompositeList(Val<Vec<Fld<T>>>),
}

#[derive(Debug)]
pub struct FieldList {
    fields: Vec<Fld<Subject>>,
}

impl FieldList {
    pub fn find(&self, _subj: Subject) -> Option<&Fld<Subject>> {
        // find field, recursive
        unimplemented!();
    }
}


pub trait Method : Debug {
    fn name(&self) -> &str;
    fn object(&self) -> Object;
    fn req(&self) -> &FieldList;
    fn rsp(&self) -> &FieldList;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_album_meta() {
        let req: Fld<Subject> = Fld::Composite(Val::Req(vec![
            Fld::Unit(Val::Req(Subject::Artist)),
            Fld::Unit(Val::Req(Subject::Name)),
            Fld::Unit(Val::Opt(Subject::Mbid)),
        ]));

        let rsp: Fld<Subject> = Fld::Composite(Val::Req(vec![
            Fld::Unit(Val::Req(Subject::Name)),
            Fld::CompositeList(Val::Req(vec![
                Fld::Unit(Val::Req(Subject::Name)),
                Fld::Unit(Val::Opt(Subject::Duration)),
            ])),
        ]));

        println!("\nreq: {:?}\n\nrsp: {:?}\n", req, rsp);

    }
}
