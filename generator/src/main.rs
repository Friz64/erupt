mod codegen;
mod comment_gen;
mod declaration;
mod header;
mod items;
mod loaders;
mod name;
mod origin;
mod source;

use log::LevelFilter;
pub use roxmltree::Node as XmlNode;
use source::Source;
use std::{env, process::Command, time::Instant};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    pretty_env_logger::formatted_builder()
        .filter(Some("generator"), LevelFilter::Trace)
        .init();

    log::info!("Collecting source...");
    let start = Instant::now();
    let source = Source::collect();
    log::info!("Source collection finished in {:.2?}", start.elapsed());

    log::info!("Generating code...");
    let start = Instant::now();
    codegen::generate(&source);
    log::info!("Code generation finished in {:.2?}", start.elapsed());

    log::info!("Formatting output...");
    let start = Instant::now();
    let cmd = Command::new("cargo")
        .current_dir("erupt/")
        .args(&["+nightly", "fmt"])
        .status()
        .expect("Failed to run rustfmt");
    log::info!("Output formatting finished in {:.2?}", start.elapsed());
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
    log::info!("Output checking finished in {:.2?}", start.elapsed());
    if !cmd.success() {
        log::error!("Output checking failed");
        return;
    }

    log::info!("Generating documentation...");
    let start = Instant::now();
    let cmd = Command::new("cargo")
        .args(&["+nightly", "doc", "-p", "erupt"])
        .status()
        .expect("Failed to run rustdoc");
    log::info!(
        "Documentation generation finished in {:.2?}",
        start.elapsed()
    );

    if !cmd.success() {
        log::error!("Documentation generation failed");
        return;
    }
}
