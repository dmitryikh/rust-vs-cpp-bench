use std::env;
use std::process::{Command,Stdio,ExitStatus};
use std::time::Instant;
use std::fs::File;
use std::io::{Read,Write,Result};
use std::fs::OpenOptions;

// measure and return time elapsed in `func` in seconds
fn measure_runtime<F: Fn()>(func: F) -> f64 {
    let start = Instant::now();
    func();
    let elapsed = start.elapsed();
    (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0)
}

// run external program `prog_name` with optional `args`, pass file with name `stdin_fname` to
// program's stdin and save program's stdout to file with name `stdout_fname`. Return program's
// Result.
fn run_command_io( prog_name: &str, args: Option<&[&str]>
                 , stdin_fname: Option<&str>
                 , stdout_fname: Option<&str>
                 ) -> Result<ExitStatus> {
    let mut comm = Command::new(prog_name);
    match args {
        Some(args_) => { comm.args(args_); }
        None          => { }
    };
    match stdin_fname {
        Some(fname) => { let file = File::open(fname).unwrap();
                           comm.stdin(file);
                       }
        None          => { }
    };
    match stdout_fname {
        Some(fname) => { let file = OpenOptions::new().write(true).create(true).open(fname).unwrap();
                         comm.stdout(file);
                       }
        None          => { comm.stdout(Stdio::null()); }
    };
    let mut child = comm.spawn().expect("Can't run command");
    child.wait()
}

fn main() {
    // 1st argument - name of the measured program
    // 2nd argument - name of the file to pass to the program's stdin
    // 3rd argument(optional) - name of the file to save pragrams' stdout
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 || args.len() > 3 {
        panic!("You should provide 2 or 3 arguments");
    }
    let stdout = if args.len() == 3 {
        Some(&args[2] as &str)
    } else {
        None
    };
    let sec = measure_runtime(|| { run_command_io(&args[0], None, Some(&args[1]), stdout).unwrap(); }); 
    println!("{}", sec);
}
