// build.rs
use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use MD5Status::{Mismatch, Unknown};

const PACKAGE_URL: &str = "https://github.com/ivmai/cudd/archive/refs/tags/cudd-3.0.0.tar.gz";
const PACKAGE_MD5: &str = "edca9c69528256ca8ae37be9cedef73f";

#[derive(Debug)]
enum FetchError {
    CommandError(std::process::ExitStatus),
    IOError(std::io::Error),
    PathExists,
}

enum MD5Status {
    Match,
    Mismatch,
    Unknown,
}

impl From<std::io::Error> for FetchError {
    fn from(err: std::io::Error) -> FetchError {
        FetchError::IOError(err)
    }
}

/// Run a command and return (stdout, stderr) if exit status is success.
fn run_command(cmd: &mut Command) -> Result<(String, String), FetchError> {
    let output = cmd.output()?;

    return if output.status.success() {
        Ok((
            String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap(),
        ))
    } else {
        println!("Command {:?} exited with status {}", cmd, output.status);
        Err(FetchError::CommandError(output.status))
    };
}

/// Fetch a file from a URL if it does not already exist in out_dir and verify its md5sum if possible.
fn fetch_package(out_dir: &str, url: &str, md5: &str) -> Result<(PathBuf, MD5Status), FetchError> {
    let out_path = Path::new(&out_dir);
    let target_path = out_path.join(Path::new(url).file_name().unwrap());
    let target_path_str = target_path.clone().into_os_string().into_string().unwrap();

    match target_path.metadata() {
        Err(error) if error.kind() == ErrorKind::NotFound => {
            // Path does not exist! Start download...
            println!("Downloading {} to {}", url, target_path_str);
            let mut command = Command::new("curl");
            command.args(&["-L", url, "-o", target_path_str.as_str()]);
            run_command(&mut command)?;
        }
        Ok(data) if data.is_file() => {
            println!("{} exists. Skipping download.", target_path_str);
        }
        Ok(_) => return Err(FetchError::PathExists),
        Err(error) => return Err(FetchError::IOError(error)),
    }

    // Now run md5 sum check:
    let mut command_1 = Command::new("md5sum");
    command_1.arg(target_path.clone());
    let mut command_2 = Command::new("md5");
    command_2.arg(target_path.clone());
    let md5_result = run_command(&mut command_1).or_else(|_| run_command(&mut command_2));

    let md5_status = match md5_result {
        Err(_) => MD5Status::Unknown,
        Ok((output, _)) if output.contains(md5) => MD5Status::Match,
        _ => MD5Status::Mismatch,
    };

    Ok((target_path, md5_status))
}

fn main() -> Result<(), String> {
    let out_dir = env::var("OUT_DIR")
        .map_err(|_| format!("Environmental variable `OUT_DIR` not defined."))?;

    let (tar_path, md5_status) = fetch_package(&out_dir, PACKAGE_URL, PACKAGE_MD5)
        .map_err(|e| format!("Error downloading CUDD package: {:?}.", e))?;
    let tar_path_str = tar_path.to_str().unwrap().to_string();

    match md5_status {
        Unknown => eprintln!("WARNING: MD5 not computed. Package validation skipped."),
        Mismatch => return Err(format!("CUDD package MD5 hash mismatch.")),
        _ => (),
    }

    // Get cudd.tar.gz path without extensions.
    let cudd_path = tar_path.with_extension("").with_extension("");
    let cudd_path_str = cudd_path.clone().into_os_string().into_string().unwrap();

    if !cudd_path.exists() {
        // Create the destination directory.
        std::fs::create_dir_all(cudd_path.clone())
            .map_err(|e| format!("Cannot create CUDD director: {:?}", e))?;
    }

    // un-tar package, ignoring the name of the top level folder, dumping into cudd_path instead.
    let mut tar_command = Command::new("tar");
    tar_command.args(&["xf", &tar_path_str, "--strip-components=1", "-C", &cudd_path_str]);
    run_command(&mut tar_command)
        .map_err(|e| format!("Error decompressing CUDD: {:?}", e))?;

    let build_output = autotools::build(cudd_path);
    eprintln!("Output: {}", build_output.display());
    println!("cargo:rustc-link-search=native={}/lib", build_output.display());
    println!("cargo:rustc-link-lib=static=cudd");

    Ok(())
}
