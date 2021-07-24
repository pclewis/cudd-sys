// build.rs
use std::process::Command;
use std::env;
use std::fs;
use std::io::{BufReader,BufWriter,BufRead,Write};
use std::fs::File;
use std::path::{Path,PathBuf};

const PACKAGE_URL: &str = "https://github.com/ivmai/cudd/archive/refs/tags/cudd-2.5.1.tar.gz";
const PACKAGE_MD5: &str = "42283a52ff815c5ca8231b6927318fbf";

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
        FetchError::IOError( err )
    }
}

/// Run a command and return (stdout, stderr) if exit status is success.
fn run_command( cmd: &mut Command ) -> Result<(String, String), FetchError>
{
    let output = cmd.output()?;

    return if output.status.success() {
        Ok((String::from_utf8(output.stdout).unwrap(),
            String::from_utf8(output.stderr).unwrap()))
    } else {
        println!("Command {:?} exited with status {}", cmd, output.status);
        Err(FetchError::CommandError(output.status))
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
                    run_command( &mut Command::new("curl").args(&["-L", url, "-o", target_path_str]) )?;
                } else {
                    return Err( FetchError::IOError(e) );
                }
            }
        }

        run_command( &mut Command::new("md5sum").arg(target_path_str) ).or_else(|_|
            run_command( &mut Command::new("md5").arg(target_path_str) ))
    };

    Ok(
        (target_path,
        match md5_result {
            Err( _ ) => MD5Status::Unknown,
            Ok( (out,_) ) => {
                // md5sum outputs "<md5> <file>", md5 on OSX outputs "MD5 (<file>) = <md5>"
                if out.contains(md5) { MD5Status::Match }
                else { MD5Status::Mismatch }
            }
        }))

}

fn replace_lines( path: &Path, replacements: Vec<(&str,&str)> ) -> Result< u32, std::io::Error >
{
    let mut lines_replaced = 0;
    let new_path = path.with_extension(".new");

    {
        let f_in = File::open(&path)?;
        let f_out = File::create(&new_path)?;
        let reader = BufReader::new(&f_in);
        let mut writer = BufWriter::new(&f_out);

        'read: for line in reader.lines() {
            let line = line.unwrap();
            for &(original, replacement) in &replacements {
                if line.starts_with(&original) {
                    writeln!(writer, "{}", replacement)?;
                    lines_replaced += 1;
                    continue 'read;
                }
            }
            writeln!(writer, "{}", line)?;
        }
    }

    fs::remove_file(&path)?;
    fs::rename(&new_path, &path)?;

    Ok( lines_replaced )
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
    if !untarred_path.exists() { // Create the destination directory.
        std::fs::create_dir_all(untarred_path.clone()).unwrap();
    }
        
    // untar package, ignoring the name of the top level folder, dumping into untarred_path instead.
    let untarred_path_str = untarred_path.clone().into_os_string().into_string().unwrap();    
    run_command( Command::new("tar").args(&["xf", tar_path.to_str().unwrap(), "--strip-components=1", "-C", &untarred_path_str]) ).unwrap();

    // patch Makefile
    let lines_replaced = replace_lines(
        &untarred_path.join("Makefile"),
        vec![// need PIC so our rust lib can be dynamically linked
            ("ICFLAGS\t= -g -O3",
             "ICFLAGS\t= -g -O3 -fPIC"),
            // remove "nanotrav" from DIRS, it doesn't compile on OSX
            ("DIRS\t= $(BDIRS)", // (matches prefix)
             "DIRS\t= $(BDIRS)")]).unwrap();
    if lines_replaced != 2 {
        panic!("Replaced {} lines in Makefile, expected 1", lines_replaced);
    }

    // build libraries (execute make)
    run_command( Command::new("make").current_dir(&untarred_path) ).unwrap();

    // Move all libs to output directory so we don't have to specify each directory
    run_command( Command::new("sh").current_dir(&out_dir).args(&["-c", "cp cudd-*/*/*.a ."]) ).unwrap();

    println!("cargo:rustc-flags=-L {}", out_dir);
    println!("cargo:rustc-flags=-l static=cudd -l static=mtr -l static=util -l static=epd -l static=st");
}
