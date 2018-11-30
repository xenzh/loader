// eval: (conditions, meta) -> plan
//
// GET tracks WHERE \
// artist is a and (year is 2018 or year is 2015) and tag not in (pop, electro) and track.no is 1
//
// I would:
// 1. get Artist:Albums
// 2. for each of them get Album:Info
// 2. filter albums by year and tag from Album:Info
// 3. Return Album:Tracks for all albums

// Basic eval blocks:
// * get_target(p: &mut Plan, tgt: Target, inp: &[Target]) -> Result<Hops, UnableTo>;
//   find method returning tgt and requiring inp. If no exact match, call recursively on missing tgts
//
// * stream(plan_item_with_single_return_tgt, how_much) -> plan_item_with_stream_return_tgt
//   someone has to do Target -> &[Target] transformation by making multiple same method requests
//
// * filter(p: &mut Plan, source: &FieldList, subj: Subject, inp: &[Target]) -> Result<Hops, UnableTo>;
//   filter target from method response (maybe streamed) fields by applying imp values
//   if not enough, may call get_target() to retrieve missing source filter fields.

// Steps:
// 1. Get who_returns(FIELD) order by [max_req, max_rsp]
// 2.


use std::borrow::Borrow;
use crate::meta::method::{Target, Method};


pub struct Methods {
    dummy: Vec<Box<Method>>,
}

impl Methods {
    // methods return list / iter of methods fitting any of criteria, ordered by number of matching
    // criteria

    fn who_returns(&self, _tgts: &[Target]) -> impl Iterator<Item = &Method> {
        self.dummy.iter().map(|b| b.borrow())
    }

    fn who_takes(&self, _tgts: &[Target]) -> impl Iterator<Item = &Method> {
        self.dummy.iter().map(|b| b.borrow())
    }
}


#[derive(Debug)]
pub struct PlanItem;


pub struct MethodMatch<'m> {
    method: &'m Method,
    missing_input: Vec<Target>,
    extra_input: Vec<Target>,
}

pub trait Planner {
    fn stream();

    fn filter();

    // Select methods than take 'from' input and return 'to' output
    fn transform(&self, methods: &Methods, from: &[Target], to: Target) -> &[MethodMatch];
}
