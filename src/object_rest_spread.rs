use swc_core::ecma::ast::*;
use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};

pub fn object_rest_spread() -> impl Fold + VisitMut {
    as_folder(ObjectRestSpread {
        ..Default::default()
    })
}

#[derive(Debug, Default)]
struct ObjectRestSpread {
    found_object_rest_spread: bool,
}

impl VisitMut for ObjectRestSpread {
    noop_visit_mut_type!();

    fn visit_mut_rest_pat(&mut self, n: &mut RestPat) {
        self.found_object_rest_spread = true;
        println!("object_rest_spread: {:?}", n)
    }
    fn visit_mut_spread_element(&mut self, _n: &mut SpreadElement) {
        self.found_object_rest_spread = true
    }
}
