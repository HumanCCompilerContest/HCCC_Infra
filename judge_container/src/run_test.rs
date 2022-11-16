use crate::{ExeOption, ExitCode};
use serde::Deserialize;
use std::time::Duration;
use tokio::process::Command;

const TLE_SEC: u64 = 2;

#[derive(Deserialize)]
struct Testcase {
    _id: i32,
    input: String,
    expect: String,
}

pub async fn just_exec() {
    // exec
    let output = tokio::time::timeout(
        Duration::from_secs(TLE_SEC as u64),
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

    println!("{:?}", String::from_utf8_lossy(&output.stdout));
}

pub async fn with_testcase(testcase_number: &str, exe_option: ExeOption) {
    let testcase_str = std::fs::read_to_string(format!("./testcase/case{}.json", testcase_number))
        .unwrap_or_else(|_| {
            eprintln!("System Error");
            std::process::exit(ExitCode::SystemError as i32);
        });
    let testcases: Vec<Testcase> = serde_json::from_str(&testcase_str).unwrap();

    for case in testcases {
        // exec and test
        let output = tokio::time::timeout(
            Duration::from_secs(TLE_SEC as u64),
            Command::new("bash")
                .kill_on_drop(true)
                .arg("-c")
                .arg(format!("echo {} | ./test_target", case.input))
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

        match exe_option {
            ExeOption::ExitCode => {
                let exit_status = output.status.code().unwrap();
                let expect: i32 = case.expect.parse().unwrap();
                if exit_status != expect {
                    eprintln!("output: {:?}", exit_status);
                    std::process::exit(ExitCode::WA as i32);
                }
            }
            ExeOption::Output => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout != case.expect {
                    eprintln!("output: {:?}", stdout);
                    std::process::exit(ExitCode::WA as i32);
                }
            }
            _ => panic!("invalid ExeOption"),
        }
    }
}
