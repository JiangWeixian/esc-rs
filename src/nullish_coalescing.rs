use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};

pub fn nullish_coalescing() -> impl Fold + VisitMut {
    as_folder(NullishCoalescing {
        ..Default::default()
    })
}

#[derive(Debug, Default)]
struct NullishCoalescing {
    found_nullish_coalescing: bool,
}

impl VisitMut for NullishCoalescing {
    noop_visit_mut_type!();

    fn visit_mut_bin_expr(&mut self, n: &mut BinExpr) {
        if let BinaryOp::NullishCoalescing = n.op {
            println!("found_nullish_coalescing {:?}", n);
            self.found_nullish_coalescing = true;
        }
    }
}
