use std::collections::HashMap;

use preset_env_base::query::targets_to_versions;
use preset_env_base::version::should_enable;
use swc_core::ecma::ast::EsVersion;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{noop_visit_type, Visit, VisitWith};
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
      class_static_block: should_enable!(ClassStaticBlock, false) || es_version < EsVersion::Es2022,
      private_methods: should_enable!(PrivateMethods, false) || es_version < EsVersion::Es2022,
      class_properties: should_enable!(ClassProperties, false) || es_version < EsVersion::Es2022,
      logical_assignment_operators: should_enable!(LogicalAssignmentOperators, false)
        || es_version < EsVersion::Es2021,
      nullish_coalescing: should_enable!(NullishCoalescing, false)
        || es_version < EsVersion::Es2020,
      optional_chaining: should_enable!(OptionalChaining, false) || es_version < EsVersion::Es2020,
      optional_catch_binding: should_enable!(OptionalCatchBinding, false)
        || es_version < EsVersion::Es2019,
      // https://babeljs.io/docs/babel-plugin-transform-object-rest-spread
      object_rest_spread: should_enable!(ObjectRestSpread, false) || es_version < EsVersion::Es2018,
      async_to_generator: should_enable!(AsyncToGenerator, false) || es_version < EsVersion::Es2017,
      exponentiation_operator: should_enable!(ExponentiationOperator, false)
        || es_version < EsVersion::Es2016,
      // alias es6
      block_scoping: should_enable!(BlockScoping, false) || es_version < EsVersion::Es2015,
      arrow_functions: should_enable!(ArrowFunctions, false) || es_version < EsVersion::Es2015,
      parameters: should_enable!(Parameters, false) || es_version < EsVersion::Es2015,
      spread: should_enable!(Spread, false) || es_version < EsVersion::Es2015,
      template_literals: should_enable!(TemplateLiterals, false) || es_version < EsVersion::Es2015,
      sticky_regex: should_enable!(StickyRegex, false) || es_version < EsVersion::Es2015,
      shorthand_properties: should_enable!(ShorthandProperties, false)
        || es_version < EsVersion::Es2015,
      computed_properties: should_enable!(ComputedProperties, false)
        || es_version < EsVersion::Es2015,
      destructuring: should_enable!(Destructuring, false) || es_version < EsVersion::Es2015,
      // TODO:
      classes: should_enable!(Classes, false) || es_version < EsVersion::Es2015,
      regenerator: should_enable!(Regenerator, false) || es_version < EsVersion::Es2015,
      // duplicate_keys: should_enable!(DuplicateKeys, false) || es_version < EsVersion::Es2015,
      // instanceOf: should_enable!(InstanceOf, false) || es_version < EsVersion::Es2015,
      for_of: should_enable!(ForOf, false) || es_version < EsVersion::Es2015,
      function_name: should_enable!(FunctionName, false) || es_version < EsVersion::Es2015,
      // literals: should_enable!(Literals, false) || es_version < EsVersion::Es2015,
      new_target: should_enable!(NewTarget, false) || es_version < EsVersion::Es2015,
      object_super: should_enable!(ObjectSuper, false) || es_version < EsVersion::Es2015,
      typeof_symbol: should_enable!(TypeOfSymbol, false) || es_version < EsVersion::Es2015,
      // unicode_escapes: should_enable!(UnicodeEscapes, false) || es_version < EsVersion::Es2015,
      // unicode_regex: should_enable!(UnicodeRegex, false) || es_version < EsVersion::Es2015,
    },
    ..Default::default()
  }
}
#[napi(object)]
#[derive(Debug, Default, Clone)]
pub struct FeaturesFlag {
  pub regenerator: bool,
  pub function_name: bool,
  pub new_target: bool,
  pub object_super: bool,
  pub typeof_symbol: bool,
  pub for_of: bool,
  pub classes: bool,
  pub spread: bool,
  pub class_properties: bool,
  pub destructuring: bool,
  pub computed_properties: bool,
  pub shorthand_properties: bool,
  pub sticky_regex: bool,
  pub template_literals: bool,
  pub parameters: bool,
  pub arrow_functions: bool,
  pub block_scoping: bool,
  pub exponentiation_operator: bool,
  pub class_static_block: bool,
  pub private_methods: bool,
  pub async_to_generator: bool,
  pub logical_assignment_operators: bool,
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
impl Visit for ESC {
  noop_visit_type!();

  // const a = function() {}
  fn visit_fn_expr(&mut self, n: &FnExpr) {
    n.visit_children_with(self);
    if self.flags.function_name {
      self.features.function_name = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
  }

  // var a = class {}
  fn visit_class_expr(&mut self, n: &ClassExpr) {
    n.visit_children_with(self);
    if self.flags.function_name {
      self.features.function_name = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
  }

  // new.target
  fn visit_meta_prop_expr(&mut self, n: &MetaPropExpr) {
    n.visit_children_with(self);
    if self.flags.new_target {
      self.features.new_target = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
  }

  // for of
  fn visit_for_of_stmt(&mut self, n: &ForOfStmt) {
    n.visit_children_with(self);
    if self.flags.for_of {
      self.features.for_of = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
  }

  // Class
  fn visit_class_decl(&mut self, n: &ClassDecl) {
    n.visit_children_with(self);
    if self.flags.classes {
      self.features.classes = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
  }

  // const obj = { ["key"]: value }
  fn visit_computed_prop_name(&mut self, n: &ComputedPropName) {
    n.visit_children_with(self);
    if self.flags.computed_properties {
      self.es_versions.insert(EsVersion::Es2015, true);
      self.features.computed_properties = true;
    }
  }

  // class A { a = '' }
  fn visit_class_prop(&mut self, n: &ClassProp) {
    n.visit_children_with(self);
    if self.flags.class_properties {
      self.es_versions.insert(EsVersion::Es2021, true);
      self.features.class_properties = true;
    }
  }

  // Visit object prop
  // const obj = { a, b }
  fn visit_prop(&mut self, n: &Prop) {
    n.visit_children_with(self);
    match n {
      Prop::Shorthand(..) => {
        if self.flags.shorthand_properties {
          self.es_versions.insert(EsVersion::Es2015, true);
          self.features.shorthand_properties = true;
        }
        return;
      }
      Prop::Method(..) => {
        if self.flags.shorthand_properties {
          self.es_versions.insert(EsVersion::Es2015, true);
          self.features.shorthand_properties = true;
        }
        return;
      }
      _ => (),
    }
  }
  // /Foo\s+(\d+)/y
  fn visit_regex(&mut self, n: &Regex) {
    if n.flags.contains("y") && self.flags.sticky_regex {
      self.features.sticky_regex = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
  }
  // template string
  fn visit_tpl(&mut self, n: &Tpl) {
    n.visit_children_with(self);
    if self.flags.template_literals {
      self.features.template_literals = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
  }

  // const let
  fn visit_var_decl_kind(&mut self, n: &VarDeclKind) {
    n.visit_children_with(self);
    if self.flags.block_scoping {
      match n {
        VarDeclKind::Const => {
          self.features.block_scoping = true;
          self.es_versions.insert(EsVersion::Es2015, true);
          return;
        }
        VarDeclKind::Let => {
          self.features.block_scoping = true;
          self.es_versions.insert(EsVersion::Es2015, true);
          return;
        }
        _ => (),
      }
    }
  }

  // static
  fn visit_static_block(&mut self, n: &StaticBlock) {
    n.visit_children_with(self);
    if self.flags.class_static_block {
      self.features.class_static_block = true;
      self.es_versions.insert(EsVersion::Es2022, true);
    }
  }

  // #private
  fn visit_private_method(&mut self, n: &PrivateMethod) {
    n.visit_children_with(self);
    if self.flags.private_methods {
      self.es_versions.insert(EsVersion::Es2022, true);
      self.features.private_methods = true;
    }
  }

  fn visit_private_prop(&mut self, n: &PrivateProp) {
    n.visit_children_with(self);
    if self.flags.private_methods {
      self.es_versions.insert(EsVersion::Es2022, true);
      self.features.private_methods = true;
    }
  }

  // async function a() {}
  fn visit_function(&mut self, n: &Function) {
    n.visit_children_with(self);
    // function a({ x, y }) {}
    if contains_destructuring(&n.params) && !contains_object_rest(&n.params) {
      self.features.destructuring = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
    // function a({ x, ...rest }) {}
    if contains_object_rest(&n.params) && self.flags.object_rest_spread {
      self.features.object_rest_spread = true;
      self.es_versions.insert(EsVersion::Es2018, true);
    }
    for param in &n.params {
      match param.pat {
        // function (x=1) {}
        Pat::Assign(..) => {
          if self.flags.parameters {
            self.es_versions.insert(EsVersion::Es2015, true);
            self.features.parameters = true;
          }
          return;
        }
        // function (...args) {}
        Pat::Rest(..) => {
          if self.flags.parameters {
            self.es_versions.insert(EsVersion::Es2015, true);
            self.features.parameters = true;
          }
          return;
        }
        _ => (),
      }
    }
    if n.is_async && self.flags.async_to_generator {
      self.es_versions.insert(EsVersion::Es2017, true);
      self.features.async_to_generator = true
    }
    if n.is_generator && self.flags.regenerator {
      self.es_versions.insert(EsVersion::Es2015, true);
      self.features.regenerator = true
    }
  }

  // const b = async () => {}
  fn visit_arrow_expr(&mut self, n: &ArrowExpr) {
    n.visit_children_with(self);
    // async arrow function
    if n.is_async && self.flags.async_to_generator {
      self.es_versions.insert(EsVersion::Es2017, true);
      self.features.async_to_generator = true;
    }
    // arrow function
    self.es_versions.insert(EsVersion::Es2015, true);
    self.features.arrow_functions = true;
  }

  // ??= ||= &&=
  fn visit_assign_op(&mut self, n: &AssignOp) {
    n.visit_children_with(self);
    match n {
      // &&=
      AssignOp::AndAssign | AssignOp::NullishAssign | AssignOp::OrAssign => {
        if self.flags.logical_assignment_operators {
          self.features.logical_assignment_operators = true;
          self.es_versions.insert(EsVersion::Es2021, true);
        }
        return;
      }
      // **=
      AssignOp::ExpAssign => {
        if self.flags.exponentiation_operator {
          self.features.exponentiation_operator = true;
          self.es_versions.insert(EsVersion::Es2016, true);
        }
        return;
      }
      _ => (),
    }
  }

  // ??
  fn visit_bin_expr(&mut self, n: &BinExpr) {
    n.visit_children_with(self);
    match n.op {
      // ??
      BinaryOp::NullishCoalescing => {
        if self.flags.nullish_coalescing {
          self.features.nullish_coalescing = true;
          self.es_versions.insert(EsVersion::Es2020, true);
        }
        return;
      }
      // **
      BinaryOp::Exp => {
        if self.flags.exponentiation_operator {
          self.features.exponentiation_operator = true;
          self.es_versions.insert(EsVersion::Es2016, true);
        }
        return;
      }
      _ => (),
    };
    // typeof Symbol() === 'symbol' or 'symbol' === typeof Symbol
    if let Expr::Unary(UnaryExpr {
      op: op!("typeof"), ..
    }) = *n.left
    {
      if is_symbol_literal(&n.right) && self.flags.typeof_symbol {
        self.features.typeof_symbol = true;
        self.es_versions.insert(EsVersion::Es2015, true);
      }
    }
    if let Expr::Unary(UnaryExpr {
      op: op!("typeof"), ..
    }) = *n.right
    {
      if is_symbol_literal(&n.left) && self.flags.typeof_symbol {
        self.features.typeof_symbol = true;
        self.es_versions.insert(EsVersion::Es2015, true);
      }
    }
  }

  // ?.
  fn visit_opt_chain_expr(&mut self, n: &OptChainExpr) {
    n.visit_children_with(self);
    if self.flags.optional_chaining {
      self.features.optional_chaining = true;
      self.es_versions.insert(EsVersion::Es2020, true);
    }
  }

  // GOOD: [...a, "foo"];
  //       foo(...a);
  fn visit_expr_or_spread(&mut self, n: &ExprOrSpread) {
    n.visit_children_with(self);
    if n.spread.is_some() && self.flags.spread {
      self.features.spread = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
  }

  fn visit_var_declarators(&mut self, n: &[VarDeclarator]) {
    // const { a } = { a: 1 }
    if contains_destructuring(n) && !contains_object_rest(n) && self.flags.destructuring {
      self.features.destructuring = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
    // const { a, ...rest } = { a: 1 }
    if contains_object_rest(n) && self.flags.object_rest_spread {
      self.features.object_rest_spread = true;
      self.es_versions.insert(EsVersion::Es2018, true);
    }
    if contains_object_super(n) && self.flags.object_super {
      self.features.object_super = true;
      self.es_versions.insert(EsVersion::Es2015, true);
    }
    n.visit_children_with(self);
  }
  // const b = { ...a }
  fn visit_spread_element(&mut self, n: &SpreadElement) {
    n.visit_children_with(self);
    if self.flags.object_rest_spread {
      self.features.object_rest_spread = true;
      self.es_versions.insert(EsVersion::Es2018, true);
    }
  }

  // try {} catch {}
  fn visit_catch_clause(&mut self, cc: &CatchClause) {
    cc.visit_children_with(self);

    if cc.param.is_some() {
      return;
    }
    if self.flags.optional_catch_binding {
      self.features.optional_catch_binding = true;
      self.es_versions.insert(EsVersion::Es2019, true);
    }
  }
}

fn contains_object_super<N>(node: &N) -> bool
where
  N: VisitWith<ObjectSuperVisitor> + ?Sized,
{
  let mut v = ObjectSuperVisitor { found: false };
  node.visit_with(&mut v);
  v.found
}

#[derive(Default)]
struct ObjectSuperVisitor {
  found: bool,
}

impl Visit for ObjectSuperVisitor {
  noop_visit_type!();

  fn visit_super_prop_expr(&mut self, _n: &SuperPropExpr) {
    self.found = true;
  }
}

fn is_symbol_literal(e: &Expr) -> bool {
  match e {
    Expr::Lit(Lit::Str(Str { value, .. })) => matches!(&**value, "symbol"),
    _ => false,
  }
}

fn contains_destructuring<N>(node: &N) -> bool
where
  N: VisitWith<DestructuringVisitor> + ?Sized,
{
  let mut v = DestructuringVisitor { found: false };
  node.visit_with(&mut v);
  v.found
}

#[derive(Default)]
struct DestructuringVisitor {
  found: bool,
}

impl Visit for DestructuringVisitor {
  noop_visit_type!();

  fn visit_pat(&mut self, n: &Pat) {
    n.visit_children_with(self);
    match n {
      Pat::Ident(..) => (),
      _ => self.found = true,
    }
  }
}

fn contains_object_rest<N>(node: &N) -> bool
where
  N: VisitWith<RestVisitor> + ?Sized,
{
  let mut v = RestVisitor { found: false };
  node.visit_with(&mut v);
  v.found
}

#[derive(Default)]
struct RestVisitor {
  found: bool,
}

impl Visit for RestVisitor {
  noop_visit_type!();

  fn visit_object_pat_prop(&mut self, prop: &ObjectPatProp) {
    match *prop {
      ObjectPatProp::Rest(..) => self.found = true,
      _ => prop.visit_children_with(self),
    }
  }
}
