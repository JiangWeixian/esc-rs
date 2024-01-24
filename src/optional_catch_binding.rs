use swc_core::ecma::ast::*;
use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith};

pub fn optional_catch_binding() -> impl Fold + VisitMut {
    as_folder(OptionalCatchBinding {
        ..Default::default()
    })
}

#[derive(Debug, Default)]
struct OptionalCatchBinding {
    found_optional_catch_binding: bool,
}

impl VisitMut for OptionalCatchBinding {
    noop_visit_mut_type!();

    fn visit_mut_catch_clause(&mut self, cc: &mut CatchClause) {
        cc.visit_mut_children_with(self);

        if cc.param.is_some() {
            return;
        }
        self.found_optional_catch_binding = true
    }
}
