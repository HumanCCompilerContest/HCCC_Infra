//! Module for running an elf created by submittion
//! and judge its correctness with testcases.

use crate::ExitCode;
use serde::Deserialize;
use std::time::Duration;
use tokio::process::Command;

/// The test runner times out at 2000ms.
const TLE_SEC: u64 = 2;

/// Testcase
#[derive(Deserialize)]
struct Testcase {
    _id: i32,
    input: String,
    expect: String,
}

/// Answer output
/// * `ExitCode` - Exit status of `$ bash -c ./test_target`.
/// * `StdOut` - Output from stdout.
/// * `NoTestCase` - No test case.
#[derive(Deserialize)]
pub enum TestTarget {
    #[serde(rename = "exitcode")]
    ExitCode,
    #[serde(rename = "stdout")]
    StdOut,
    #[serde(rename = "none")]
    NoTestCase,
}

/// Testcase deserialized from json file.
/// * `judge_target` - Answer output to.
/// * `is_wrong_code` - Is probelm ivalid code.
/// * `tests` - Testcases.
#[derive(Deserialize)]
pub struct Testcases {
    pub judge_target: TestTarget,
    pub is_wrong_code: bool,
    tests: Vec<Testcase>,
}

/// Just exec `test_target`.
/// If the file exited successfully, it will be `AC`.
pub async fn just_exec() {
    // exec
    let output = tokio::time::timeout(
        Duration::from_secs(TLE_SEC),
        Command::new("bash")
            .kill_on_drop(true)
            .arg("-c")
            .arg("./test_target")
            .output(),
    )
    .await
    .unwrap_or_else(|_| {
        eprintln!("Time Limit Exceeded");
        std::process::exit(ExitCode::TLE as i32);
    })
    .unwrap_or_else(|_| {
        eprintln!("Runtime Error");
        std::process::exit(ExitCode::RE as i32);
    });

    if output.status.code().unwrap() != 0 {
        eprintln!("Runtime Error");
        std::process::exit(ExitCode::RE as i32);
    }

    // WA when submit to wrong code
    std::process::exit(ExitCode::WA as i32);
}

/// Judge with testcases.
pub async fn with_testcase(testcases: Testcases) {
    for case in testcases.tests {
        // exec and test
        let output = tokio::time::timeout(
            Duration::from_secs(TLE_SEC),
            Command::new("bash")
                .kill_on_drop(true)
                .arg("-c")
                .arg(format!("echo {} | ./test_target 2>&1", case.input))
                .output(),
        )
        .await
        .unwrap_or_else(|_| {
            eprintln!("Time Limit Exceeded");
            std::process::exit(ExitCode::TLE as i32);
        })
        .unwrap_or_else(|_| {
            eprintln!("Runtime Error");
            std::process::exit(ExitCode::RE as i32);
        });

        match testcases.judge_target {
            TestTarget::ExitCode => {
                if !output.stderr.is_empty() {
                    eprintln!("Runtime Error");
                    std::process::exit(ExitCode::RE as i32);
                }

                let exit_status = output.status.code().unwrap();
                let expect: i32 = case.expect.parse().unwrap();
                if exit_status != expect {
                    eprintln!("output: {exit_status:?}");
                    std::process::exit(ExitCode::WA as i32);
                }
            }
            TestTarget::StdOut => {
                if output.status.code().unwrap() != 0 {
                    eprintln!("Runtime Error");
                    std::process::exit(ExitCode::RE as i32);
                }

                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout != case.expect {
                    eprintln!("output: {stdout:?}");
                    std::process::exit(ExitCode::WA as i32);
                }
            }
            TestTarget::NoTestCase => panic!("invalid ExeOption"),
        }
    }
}
