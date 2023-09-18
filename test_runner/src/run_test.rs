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
#[derive(Deserialize)]
pub enum TestTarget {
    /// Exit status of `$ bash -c ./test_target`.
    #[serde(rename = "exitcode")]
    ExitCode,
    /// Output from stdout.
    #[serde(rename = "stdout")]
    StdOut,
    /// No test case.
    #[serde(rename = "none")]
    NoTestCase,
}

/// Testcase deserialized from json file.
#[derive(Deserialize)]
pub struct Testcases {
    /// Test target (exit code or stdout).
    pub test_target: TestTarget,
    /// Testcases.
    tests: Vec<Testcase>,
}

impl Testcases {
    pub fn new(test_target: TestTarget, testcases: String) -> Self {
        Testcases {
            test_target,
            tests: serde_json::from_str(&testcases).unwrap(),
        }
    }
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
        std::process::exit(ExitCode::TLE as i32);
    })
    .unwrap_or_else(|_| {
        std::process::exit(ExitCode::RE as i32);
    });

    if output.status.code().unwrap() != 0 {
        eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
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
            std::process::exit(ExitCode::TLE as i32);
        })
        .unwrap_or_else(|_| {
            std::process::exit(ExitCode::RE as i32);
        });

        match testcases.test_target {
            TestTarget::ExitCode => {
                if !output.stderr.is_empty() {
                    eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
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
                    eprintln!("{}", std::str::from_utf8(&output.stderr).unwrap());
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
