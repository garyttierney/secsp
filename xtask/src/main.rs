use std::env;
use std::process::Command;

use std::time::Duration;
use wait_timeout::ChildExt;

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);

    match task.as_ref().map(|it| it.as_str()) {
        Some("fuzz") => fuzz()?,
        _ => unimplemented!(),
    };

    Ok(())
}

fn fuzz() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let install_cmd = Command::new(&cargo)
        .args(&["install", "afl", "--force"])
        .status()?;

    if !install_cmd.success() {
        return Err("Unable to install cargo-afl".into());
    }

    let build_cmd = Command::new(&cargo)
        .env("RUSTFLAGS", "-Clink-arg=-fuse-ld=gold")
        .args(&[
            "afl",
            "build",
            "--manifest-path=packages/secsp-fuzzer/Cargo.toml",
        ])
        .status()?;

    if !build_cmd.success() {
        return Err("Unable to build with AFL instrumentation".into());
    }

    let mut fuzz_cmd = Command::new(&cargo)
        .args(&[
            "afl",
            "fuzz",
            "-i",
            "documentation/examples",
            "-o",
            "fuzzer-output/",
            "--",
            "packages/secsp-fuzzer/target/debug/secsp-fuzzer",
        ])
        .spawn()?;

    let duration = Duration::from_secs(60);

    if fuzz_cmd.wait_timeout(duration)?.is_none() {
        fuzz_cmd.kill()?;
        fuzz_cmd.wait()?;
    }

    Ok(())
}
