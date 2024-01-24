use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};

pub fn optional_chaining() -> impl Fold + VisitMut {
    as_folder(OptionalChaining {
        ..Default::default()
    })
}

#[derive(Debug, Default)]
struct OptionalChaining {
    found_optional_chaining: bool,
}

impl VisitMut for OptionalChaining {
    noop_visit_mut_type!();

    fn visit_mut_opt_chain_expr(&mut self, _n: &mut OptChainExpr) {
        self.found_optional_chaining = true;
    }
}
