mod aliases;
mod basetypes;
mod codegen;
mod comment_gen;
mod constants;
mod declaration;
mod defines;
mod enums;
mod eval;
mod functions;
mod handles;
mod header;
mod loaders;
mod name;
mod origin;
mod source;
mod structures;

use source::Source;
use std::{env, process::Command, time::Instant};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "generator");
    pretty_env_logger::init();

    log::info!("Collecting source...");
    let start = Instant::now();
    let source = Source::collect();
    log::info!("Source collection finished in {:?}", start.elapsed());

    log::info!("Generating code...");
    let start = Instant::now();
    codegen::generate(&source);
    log::info!("Code generation finished in {:?}", start.elapsed());

    log::info!("Formatting output...");
    let start = Instant::now();
    let cmd = Command::new("cargo")
        .args(&["fmt", "-p", "erupt"])
        .status()
        .expect("Failed to run rustfmt");
    log::info!("Output formatting finished in {:?}", start.elapsed());
    if !cmd.success() {
        log::error!("Output formatting failed");
        return;
    }

    log::info!("Checking output...");
    let start = Instant::now();
    let cmd = Command::new("cargo")
        .args(&["check", "-p", "erupt"])
        .status()
        .expect("Failed to run cargo check");
    log::info!("Output checking finished in {:?}", start.elapsed());
    if !cmd.success() {
        log::error!("Output checking failed");
        return;
    }

    log::info!("Generating documentation...");
    let start = Instant::now();
    let cmd = Command::new("cargo")
        .args(&["doc", "-p", "erupt"])
        .status()
        .expect("Failed to run rustdoc");
    log::info!("Documentation generation finished in {:?}", start.elapsed());
    if !cmd.success() {
        log::error!("Documentation generation failed");
        return;
    }
}
