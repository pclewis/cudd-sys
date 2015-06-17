// build.rs
use std::process::Command;
use std::env;
use std::fs;
//use std::fs::File;
use std::path::{Path,PathBuf};

const PACKAGE_URL:&'static str = "ftp://vlsi.colorado.edu/pub/cudd-2.5.1.tar.gz";
const PACKAGE_MD5:&'static str = "e2a514c2d309feab6b697195b7615b8b";

#[derive(Debug)]
enum FetchError {
    CommandError( std::process::ExitStatus ),
    IOError( std::io::Error ),
    PathExists
}

enum MD5Status {
    Match,
    Mismatch,
    Unknown
}

impl From<std::io::Error> for FetchError {
    fn from(err: std::io::Error) -> FetchError {
        return FetchError::IOError( err );
    }
}

/// Run a command and return (stdout, stderr) if exit status is success.
fn run_command( cmd: &mut Command ) -> Result<(String, String), FetchError>
{
    let output = try!(cmd.output());

    if output.status.success() {
        return Ok( (String::from_utf8(output.stdout).unwrap(),
                    String::from_utf8(output.stderr).unwrap()) );
    } else {
        return Err( FetchError::CommandError( output.status ) );
    }
}

/// Fetch a file from a URL if it does not already exist in out_dir and verify its md5sum if possible.
fn fetch_package(out_dir: &str, url: &str, md5: &str) -> Result<(PathBuf, MD5Status), FetchError>
{
    let out_path        = Path::new(&out_dir);
    let target_path     = out_path.join( Path::new(url).file_name().unwrap() );

    let md5_result = {
        let target_path_str = target_path.to_str().unwrap();
        let meta = fs::metadata(&target_path);

        match meta {
            Ok(m) => {
                if m.is_file() { println!("{} already exists, skipping download", target_path_str) }
                else { return Err(FetchError::PathExists) }
            },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    println!("Downloading {} to {}", url, target_path_str);
                    try!(run_command( &mut Command::new("curl").args(&[url, "-o", target_path_str]) ));
                } else {
                    return Err( FetchError::IOError(e) );
                }
            }
        }

        run_command( &mut Command::new("md5sum").arg(target_path_str) ).or(
            run_command( &mut Command::new("md5").arg(target_path_str) ))
    };

    return Ok(
        (target_path,
        match md5_result {
            Err( _ ) => MD5Status::Unknown,
            Ok( (out,_) ) => {
                if out.starts_with(md5) { MD5Status::Match }
                else { MD5Status::Mismatch }
            }
        }));

}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let (tar_path, md5_status) = fetch_package( &out_dir, PACKAGE_URL, PACKAGE_MD5 ).unwrap();

    match md5_status {
        MD5Status::Match => (),
        MD5Status::Unknown => println!( "No md5 available, skipping package validation" ),
        MD5Status::Mismatch => panic!( "MD5 mismatch on package" )
    }

    let untarred_path = tar_path.with_extension("").with_extension(""); // kill .tar and .gz

    // untar package
    run_command( Command::new("tar").args(&["xf", tar_path.to_str().unwrap(), "-C", &out_dir]) ).unwrap();

    // Patch Makefile.. need  -fPIC in ICFLAGS
    // HACK HACK HACK HACK HACK
    run_command( Command::new("sed").args(&["-i", "s/ICFLAGS\t= -g -O3/ICFLAGS\t= -g -O3 -fPIC/",
                                            untarred_path.join("Makefile").to_str().unwrap()]) ).unwrap();

    // build libraries (execute make)
    run_command( Command::new("make").current_dir(&untarred_path) ).unwrap();

    // Move all libs to output directory so we don't have to specify each directory
    run_command( Command::new("sh").current_dir(&out_dir).args(&["-c", "cp cudd-*/*/*.a ."]) ).unwrap();

    println!("cargo:rustc-flags=-L {}", out_dir);
    println!("cargo:rustc-flags=-l static=cudd -l static=mtr -l static=util -l static=epd -l static=st");
}
