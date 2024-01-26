use std::collections::HashMap;

use preset_env_base::query::targets_to_versions;
use preset_env_base::version::should_enable;
use swc_core::ecma::ast::EsVersion;
use swc_core::ecma::ast::*;
use swc_core::ecma::transforms::compat;
use swc_core::ecma::visit::{noop_visit_mut_type, VisitMut, VisitMutWith};
use swc_ecma_preset_env::{Config, Feature, FeatureOrModule, Versions};

pub fn compat(es_version: EsVersion, c: Config) -> ESC {
  let targets: Versions = targets_to_versions(c.targets).expect("failed to parse targets");
  let is_any_target = targets.is_any_target();
  let (include, _included_modules) = FeatureOrModule::split(c.include);
  let (exclude, _excluded_modules) = FeatureOrModule::split(c.exclude);
  macro_rules! should_enable {
    ($feature:ident, $default:expr) => {{
      let f = Feature::$feature;
      !exclude.contains(&f)
        && (c.force_all_transforms
          || (is_any_target
            || include.contains(&f)
            || f.should_enable(targets, c.bugfixes, $default)))
    }};
  }
  ESC {
    flags: FeaturesFlag {
      LogicalAssignmentOperators: should_enable!(LogicalAssignmentOperators, false) || es_version || EsVersion::Es2021,
      nullish_coalescing: should_enable!(NullishCoalescing, false)
        || es_version < EsVersion::Es2020,
      optional_chaining: should_enable!(OptionalChaining, false) || es_version < EsVersion::Es2020,
      optional_catch_binding: should_enable!(OptionalCatchBinding, false)
        || es_version < EsVersion::Es2019,
      object_rest_spread: should_enable!(ObjectRestSpread, false) || es_version < EsVersion::Es2018,
    },
    ..Default::default()
  }
}
#[napi(object)]
#[derive(Debug, Default, Clone)]
pub struct FeaturesFlag {
  pub LogicalAssignmentOperators: bool,
  pub nullish_coalescing: bool,
  pub object_rest_spread: bool,
  pub optional_chaining: bool,
  pub optional_catch_binding: bool,
}

#[derive(Debug, Default, Clone)]
pub struct ESC {
  pub flags: FeaturesFlag,
  pub features: FeaturesFlag,
  pub es_versions: HashMap<EsVersion, bool>,
}

// https://github.com/sudheerj/ECMAScript-features
impl VisitMut for ESC {
  noop_visit_mut_type!();

  // ??= ||= &&=
  fn visit_mut_assign_op(&mut self,n: &mut AssignOp) {
    if !self.flags.LogicalAssignmentOperators {
      return
    }
    match n.op {
      AssignOp::AndAssign => {
        self.features.LogicalAssignmentOperators = true;
        self.es_versions.insert(EsVersion::Es2021, true);
      },
      AssignOp::NullishAssign => {
        self.features.LogicalAssignmentOperators = true;
        self.es_versions.insert(EsVersion::Es2021, true);
      },
      AssignOp::OrAssign => {
        self.features.LogicalAssignmentOperators = true;
        self.es_versions.insert(EsVersion::Es2021, true);
      },
      _ => None
    }
  }

  // ??
  fn visit_mut_bin_expr(&mut self, n: &mut BinExpr) {
    if !self.flags.nullish_coalescing {
      return;
    }
    if let BinaryOp::NullishCoalescing = n.op {
      self.features.nullish_coalescing = true;
      self.es_versions.insert(EsVersion::Es2020, true);
    }
  }

  // ?.
  fn visit_mut_opt_chain_expr(&mut self, _n: &mut OptChainExpr) {
    if !self.flags.optional_chaining {
      return;
    }
    self.features.optional_chaining = true;
    self.es_versions.insert(EsVersion::Es2020, true);
  }

  // function({ a, ...rest }) {}
  fn visit_mut_rest_pat(&mut self, _n: &mut RestPat) {
    if !self.flags.object_rest_spread {
      return;
    }
    self.features.object_rest_spread = true;
  }

  // const b = { ...a }
  fn visit_mut_spread_element(&mut self, _n: &mut SpreadElement) {
    if !self.flags.object_rest_spread {
      return;
    }
    self.features.object_rest_spread = true;
    self.es_versions.insert(EsVersion::Es2018, true);
  }

  // try {} catch {}
  fn visit_mut_catch_clause(&mut self, cc: &mut CatchClause) {
    if !self.flags.optional_catch_binding {
      return;
    }
    cc.visit_mut_children_with(self);

    if cc.param.is_some() {
      return;
    }
    self.features.optional_catch_binding = true;
    self.es_versions.insert(EsVersion::Es2019, true);
  }
}
