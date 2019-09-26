#[macro_use]
extern crate clap;
extern crate fern;

extern crate log;
extern crate secsp_analysis;
extern crate secsp_syntax;

use std::path::PathBuf;

use clap::App;
use clap::Arg;

use secsp_analysis::{AnalysisDatabase, AnalysisHost};

mod utils;

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("input")
                .help("List of source files to process")
                .multiple(true)
                .index(1),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Turn up logging verbosity"),
        )
        .get_matches();

    let log_level = match matches.occurrences_of("verbosity") {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    setup_logger(log_level).unwrap();

    let input_files = matches
        .values_of_lossy("input")
        .unwrap_or_else(|| vec![])
        .iter()
        .map(PathBuf::from)
        .collect();

    let analysis_db = AnalysisDatabase::from_files(input_files)
        .unwrap_or_else(|e| panic!("Unable to read input files: {}", e));
    let analysis_host = AnalysisHost::new(analysis_db);

    let analysis = analysis_host.analysis();
    let source_root = analysis.source_root().expect("no source root defined");

    for id in source_root.0 {
        let source = analysis.source_file(id).expect("couldn't parse");
        println!("{}", utils::ast_to_string(&source.tree()));
    }
}
