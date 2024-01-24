mod esc;
#[macro_use]
extern crate napi_derive;

use anyhow::{anyhow, Context};
use esc::{compat, FeaturesFlag};
use preset_env_base::query::Query;
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use swc_compiler_base::{parse_js, IsModule};
use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::errors::Handler;
use swc_core::common::{sync::Lrc, FileName, SourceMap, GLOBALS};
use swc_core::ecma::ast::EsVersion;
use swc_core::ecma::visit::VisitMutWith;
use swc_ecma_parser::Syntax;
use swc_ecma_preset_env::{Config, Targets};
use swc_error_reporters::handler::{try_with_handler, HandlerOpts};

fn try_with<F, Ret>(cm: Lrc<SourceMap>, skip_filename: bool, op: F) -> Result<Ret, anyhow::Error>
where
  F: FnOnce(&Handler) -> Result<Ret, anyhow::Error>,
{
  GLOBALS.set(&Default::default(), || {
    try_with_handler(
      cm,
      HandlerOpts {
        skip_filename,
        ..Default::default()
      },
      |handler| {
        let result = catch_unwind(AssertUnwindSafe(|| op(handler)));

        let p = match result {
          Ok(v) => return v,
          Err(v) => v,
        };

        if let Some(s) = p.downcast_ref::<String>() {
          Err(anyhow!("failed to handle: {}", s))
        } else if let Some(s) = p.downcast_ref::<&str>() {
          Err(anyhow!("failed to handle: {}", s))
        } else {
          Err(anyhow!("failed to handle with unknown panic message"))
        }
      },
    )
  })
}

fn parse_target(target: Option<String>) -> EsVersion {
  if target.is_none() {
    return EsVersion::EsNext;
  }
  let es_version = match target.unwrap().as_str() {
    "esnext" => EsVersion::EsNext,
    "es2022" => EsVersion::Es2022,
    "es2021" => EsVersion::Es2021,
    "es2020" => EsVersion::Es2020,
    "es2019" => EsVersion::Es2019,
    "es2018" => EsVersion::Es2018,
    "es2017" => EsVersion::Es2017,
    "es2016" => EsVersion::Es2016,
    "es2015" => EsVersion::Es2015,
    "es5" => EsVersion::Es5,
    "es3" => EsVersion::Es3,
    _ => EsVersion::EsNext,
  };
  es_version
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ParseOptions {
  pub target: Option<String>,
  pub browserslist: String,
  pub filename: String,
  pub code: String,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct DetectResult {
  pub features: FeaturesFlag,
  #[napi(object)]
  pub es_versions: HashMap<String, bool>,
}

#[napi]
pub fn detect(options: ParseOptions) -> Result<DetectResult, napi::Error> {
  let ParseOptions {
    filename,
    code,
    browserslist,
    target,
  } = options;
  let cm: Lrc<SourceMap> = Default::default();
  let fm = cm.new_source_file(FileName::Custom(filename.into()), code.clone().into());

  let env_targets: Targets = Targets::Query(Query::Single(browserslist));
  println!("browserslist {:?}", env_targets);
  let es_version = parse_target(target);

  try_with(cm.clone(), false, |handler| {
    let comments = SingleThreadedComments::default();
    let mut module = parse_js(
      cm,
      fm,
      &handler,
      EsVersion::EsNext,
      Syntax::Es(Default::default()),
      IsModule::Bool(true),
      Some(&comments),
    )
    .context("failed to parse code")?;
    let mut esc = compat(
      es_version,
      Config {
        targets: Some(env_targets),
        mode: None,
        ..Default::default()
      },
    );
    module.visit_mut_with(&mut esc);
    Ok(DetectResult {
      features: esc.features,
      es_versions: esc
        .es_versions
        .into_iter()
        .map(|(key, value)| (format!("{:?}", key), value))
        .collect::<std::collections::HashMap<String, bool>>(),
    })
  })
  .map_err(|err| napi::Error::from_reason(format!("{:?}", err)))
}