//! This crate is a test runner
//! that retrieves submissions from the database and runs the testcases.  
//! Testcases must be provided by json file for now.

mod run_test;

use crate::run_test::{TestTarget, Testcases};
use clap::arg;
use std::fs::File;
use std::io::Write;
use tokio::process::Command;

/// Judge status (also serves as exit code).
/// This test runner returns the result via exit code.
/// `ExitCode` must be consistent with `judge_server::entities::JudgeResult`.
#[allow(dead_code)]
pub enum ExitCode {
    /// ACcepted
    AC = 0,
    /// Wrong Answer
    WA,
    /// Wrong Compile Error
    WC,
    /// Assembly Error
    AE,
    /// Linker Error
    LE,
    /// Runtime Error
    RE,
    /// Time Limit Exceeded
    TLE,
    /// the submit is pending.
    Pending,
    /// The judge failed due to system error.
    SystemError,
}

/// Command line option of the test runner.
struct CmdOption {
    /// Submitted assembly that encoded by base64.
    asm: String,
    /// Test target (exit code or stdout).
    test_target: TestTarget,
    /// Test cases that decoded by base64.
    testcases: String,
}

/// Create `CmdOption` used by clap crate.
fn get_arg() -> Result<CmdOption, Box<dyn std::error::Error>> {
    let app = clap::app_from_crate!()
        .arg(arg!(<asm> "assembly"))
        .arg(arg!(<test_target> "test cases target"))
        .arg(arg!(<testcases> "testcases json"))
        .get_matches();

    let asm = match app.value_of("asm") {
        Some(a) => String::from_utf8(base64::decode(a)?)?,
        None => panic!("please input assembly."),
    };
    let test_target = match app
        .value_of("test_target")
        .expect("please specify target for test cases.")
    {
        "exitcode" => TestTarget::ExitCode,
        "stdout" => TestTarget::StdOut,
        "none" => TestTarget::NoTestCase,
        _ => panic!("test_target is invalid value."),
    };
    let testcases = match app.value_of("testcases") {
        Some(t) => String::from_utf8(base64::decode(t)?)?,
        None => panic!("please specify target ELF file."),
    };

    Ok(CmdOption {
        asm,
        test_target,
        testcases,
    })
}

/// Create `test_target` elf file to judge submission correctness.
/// When failed to generate execute file, exit with `ExitCode`.
async fn create_elf() {
    // assemble
    let asm_result = Command::new("bash")
        .arg("-c")
        .arg("as submit.s -o tmp.o")
        .output()
        .await
        .unwrap_or_else(|_| {
            std::process::exit(ExitCode::AE as i32);
        });
    let exit_code_asm = asm_result.status.code().unwrap_or(-1);

    if exit_code_asm != 0 {
        eprintln!("{}", std::str::from_utf8(&asm_result.stderr).unwrap());
        std::process::exit(ExitCode::AE as i32);
    }

    // link
    let link_result = Command::new("bash")
        .arg("-c")
        .arg("gcc -v -static -no-pie tmp.o -o test_target")
        .output()
        .await
        .unwrap_or_else(|_| {
            std::process::exit(ExitCode::LE as i32);
        });
    let exit_code_link = link_result.status.code().unwrap_or(-1);

    if exit_code_link != 0 {
        eprintln!("{}", std::str::from_utf8(&link_result.stderr).unwrap());
        std::process::exit(ExitCode::LE as i32);
    }
}

/// Main function.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = get_arg().expect("parsing arguments failed.");
    let testcases: Testcases = Testcases::new(cmd.test_target, cmd.testcases);

    let mut file = File::create("./submit.s")?;
    writeln!(file, "{}", cmd.asm)?;
    file.flush()?;

    create_elf().await;

    match testcases.test_target {
        TestTarget::NoTestCase => run_test::just_exec().await,
        TestTarget::ExitCode | TestTarget::StdOut => {
            run_test::with_testcase(testcases).await;
        }
    }

    Ok(())
}
