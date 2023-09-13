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
/// * `is_ce` - Is compile error submission or not.
/// * `asm` - Submitted assembly that encoded by base64.
/// * `testcase_path` - File path of json format testcase.
struct CmdOption {
    is_ce: bool,
    asm: String,
    testcase_path: String,
}

/// Create `CmdOption` used by clap crate.
fn get_arg() -> Result<CmdOption, Box<dyn std::error::Error>> {
    let app = clap::app_from_crate!()
        .arg(arg!(<problem_number> "number for testcase"))
        .arg(arg!(<is_ce> "is compile error"))
        .arg(arg!(<asm> "assembly"))
        .get_matches();

    let problem_num = app.value_of("problem_number").unwrap();
    let is_ce = match app.value_of("is_ce") {
        Some(a) => matches!(a, "true"),
        None => panic!("please specify target ELF file."),
    };
    let asm = match app.value_of("asm") {
        Some(a) => String::from_utf8(base64::decode(a)?)?,
        None => panic!("please input assembly."),
    };

    Ok(CmdOption {
        is_ce,
        asm,
        testcase_path: format!("/work/testcase/case{problem_num}.json"),
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
    let cmd = get_arg().unwrap();
    let testcase_str = std::fs::read_to_string(&cmd.testcase_path).unwrap_or_else(|_| {
        eprintln!("Failed to read file: {}", &cmd.testcase_path);
        std::process::exit(ExitCode::SystemError as i32);
    });
    let testcases: Testcases = serde_json::from_str(&testcase_str).unwrap();

    if cmd.is_ce && !testcases.is_wrong_code {
        eprintln!("wrong compile error!");
        std::process::exit(ExitCode::WC as i32);
    }

    if cmd.is_ce && testcases.is_wrong_code {
        std::process::exit(ExitCode::AC as i32);
    }

    let mut file = File::create("./submit.s")?;
    writeln!(file, "{}", cmd.asm)?;
    file.flush()?;

    create_elf().await;

    match testcases.judge_target {
        TestTarget::NoTestCase => run_test::just_exec().await,
        TestTarget::ExitCode | TestTarget::StdOut => {
            run_test::with_testcase(testcases).await;
        }
    }

    Ok(())
}
