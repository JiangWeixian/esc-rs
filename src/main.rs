#![feature(let_chains)]
mod esc;
mod nullish_coalescing;
mod object_rest_spread;
mod optional_catch_binding;
mod optional_chaining;

use anyhow::{anyhow, Context};
use esc::{compat, compat_by_env};
use preset_env_base::query::Query;
use std::panic::{catch_unwind, AssertUnwindSafe};
use swc_compiler_base::{parse_js, IsModule};
use swc_core::common::errors::Handler;
use swc_core::common::Mark;
use swc_core::common::{sync::Lrc, FileName, Globals, SourceMap, GLOBALS};
use swc_core::ecma::ast::EsVersion;
use swc_core::ecma::transforms::base::helpers::HELPERS;
use swc_core::ecma::transforms::base::{helpers, Assumptions};
use swc_core::ecma::visit::{FoldWith, VisitMutWith};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_ecma_preset_env::{Config, Targets, Version};
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

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("input.js".into()),
        "const a = 1 ?? 2".into(),
    );

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let assumptions = Assumptions::default();

    let targets: Targets = Targets::Query(Query::Single(String::from("Chrome > 63")));

    let _ = try_with(cm.clone(), false, |handler| {
        let module = parse_js(
            cm,
            fm,
            &handler,
            EsVersion::EsNext,
            Syntax::Es(Default::default()),
            IsModule::Bool(true),
            None,
        )
        .context("failed to parse code")?;
        let mut esc = compat_by_env(Config {
            targets: Some(targets),
            mode: None,
            ..Default::default()
        });
        module.fold_with(&mut esc);
        Ok(())
    });

    // let _ = try_with(cm.clone(), false, |handler| {
    //     let module = parse_js(
    //         cm,
    //         fm,
    //         &handler,
    //         EsVersion::EsNext,
    //         Syntax::Es(Default::default()),
    //         IsModule::Bool(true),
    //         None,
    //     )
    //     .context("failed to parse code")?;
    //     let mut esc = compat(
    //         Some(EsVersion::Es5),
    //         assumptions,
    //         Mark::new(),
    //         Mark::new(),
    //         None,
    //         true,
    //     );
    //     module.fold_with(&mut esc);
    //     Ok(())
    // });
    // let program = parser.parse_program().expect("should work");
    // let helpers = Default::default();
    // let globals = Default::default();
    // HELPERS.set(&helpers, || {
    //     GLOBALS.set(&globals, || {
    //         let mut esc = compat(
    //             Some(EsVersion::Es5),
    //             assumptions,
    //             Mark::new(),
    //             Mark::new(),
    //             None,
    //             true,
    //         );
    //         program.fold_with(&mut esc);
    //     });
    // });
}
