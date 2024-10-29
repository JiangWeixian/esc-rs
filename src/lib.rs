mod esc;
#[macro_use]
extern crate napi_derive;

use anyhow::{anyhow, Context};
use esc::{compat, Detail, FeaturesFlag, Line};
use preset_env_base::query::Query;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::panic::{catch_unwind, AssertUnwindSafe};
use swc_compiler_base::{parse_js, IsModule};
use swc_core::common::comments::SingleThreadedComments;
use swc_core::common::errors::Handler;
use swc_core::common::{sync::Lrc, FileName, SourceMap, GLOBALS};
use swc_core::ecma::ast::EsVersion;
use swc_core::ecma::visit::VisitWith;
use swc_ecma_parser::Syntax;
use swc_ecma_preset_env::{Config, Targets};
use swc_error_reporters::handler::{try_with_handler, HandlerOpts};
use sourcemap::SourceMap as RawSourceMap;

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
  pub es_versions: HashMap<String, bool>,
  pub details: Vec<Detail>,
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
  let es_version = parse_target(target);

  try_with(cm.clone(), false, |handler| {
    let comments = SingleThreadedComments::default();
    let module = parse_js(
      cm.clone(),
      fm.clone(),
      &handler,
      EsVersion::EsNext,
      Syntax::Es(Default::default()),
      IsModule::Bool(true),
      Some(&comments),
    )
    .context("failed to parse code")?;
    let mut esc = compat(
      es_version,
      cm,
      fm,
      Config {
        targets: Some(env_targets),
        mode: None,
        // https://github.com/babel/babel/issues/16254
        bugfixes: true,
        ..Default::default()
      },
    );
    module.visit_with(&mut esc);
    Ok(DetectResult {
      features: esc.features,
      es_versions: esc
        .es_versions
        .into_iter()
        .map(|(key, value)| (format!("{:?}", key), value))
        .collect::<std::collections::HashMap<String, bool>>(),
      details: esc.details,
    })
  })
  .map_err(|err| napi::Error::from_reason(format!("{:?}", err)))
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct LookupOptions {
  pub filename: String,
  pub details: Vec<Detail>,
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct LookupResult {
  pub ls: Option<Line>,
  pub le: Option<Line>,
  pub source: Option<String>
}

#[napi]
pub fn lookup(options: LookupOptions) -> Result<Vec<LookupResult>, napi::Error> {
  let mut file = File::open(options.filename).expect("TODO");
  let mut source_map_content = String::new();
  file.read_to_string(&mut source_map_content).expect("TODO");
  let smc = RawSourceMap::from_slice(source_map_content.as_bytes()).expect("TODO");
  
  let mut result: Vec<LookupResult> = vec![];
  let mut source: Option<String> = Default::default();
  for generated_loc in options.details {
    let line_lo = generated_loc.ls.l;
    let col_lo = generated_loc.ls.c;
    let line_hi = generated_loc.le.l;
    let col_hi = generated_loc.le.c;
    let original_loc_lo: Option<Line> = match smc.lookup_token(line_lo as u32, col_lo as u32) {
      Some(token) => {
        source = token.get_source().map(|f| f.to_string());
        let loc = Line {
          l: token.get_src_line() as i32,
          c: token.get_src_col() as i32,
        };
        Some(loc)
      },
      None => None
    };
    let original_loc_hi: Option<Line> = match smc.lookup_token(line_hi as u32, col_hi as u32) {
      Some(token) => {
        source = token.get_source().map(|f| f.to_string());
        let loc = Line {
          l: token.get_src_line() as i32,
          c: token.get_src_col() as i32,
        };
        Some(loc)
      },
      None => None
    };
    let original_loc = LookupResult {
      ls: original_loc_lo,
      le: original_loc_hi,
      source: source.clone(),
    };
    result.push(original_loc);
  }
  Ok(result)
}