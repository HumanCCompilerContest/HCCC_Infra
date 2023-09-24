//! This crate is the server that judges submissions.  
//! It monitors the database tables, retrieves Pending submissions,
//! judges them using `test_runner`, and stores the results in the database.

use futures::future;
use judge_server::database::RepositoryProvider;
use judge_server::entities::{JudgeResult, Problem, Submit, Testcase};
use judge_server::repositories::{Problems, Submits, Testcases};
use tokio::process::Command;
use tokio::time::{sleep, Duration};

/// Judge the submission using `test_runner`.
async fn judge(
    submit: &Submit,
    problem: &Problem,
    testcase: &Vec<Testcase>,
) -> (JudgeResult, Option<String>, i32) {
    const CONTAINER_NAME: &str = "ghcr.io/humanccompilercontest/hccc_infra:test_runner-develop";
    if submit.is_ce {
        if problem.is_wrong_code && submit.error_line_number == problem.error_line_number {
            return (JudgeResult::AC, None, submit.id());
        } else {
            return (
                JudgeResult::WC,
                Some("Wrong Compile Error".to_string()),
                submit.id(),
            );
        }
    }

    let result = Command::new("bash")
        .arg("-c")
        .arg(dbg!(format!(
            "docker run --rm --memory=128M --cpus=\"0.05\" {} {} {} {}",
            CONTAINER_NAME,
            base64::encode(&submit.asm),
            serde_json::to_string(&problem.test_target).expect("getting renamed name failed"),
            base64::encode(serde_json::to_string(&testcase).expect("serialization failed")),
        )))
        .output()
        .await;

    if let Ok(result) = result {
        let judge_result = match result.status.code().unwrap_or(7) {
            0 => JudgeResult::AC,
            1 => JudgeResult::WA,
            2 => JudgeResult::WC,
            3 => JudgeResult::AE,
            4 => JudgeResult::LE,
            5 => JudgeResult::RE,
            6 => JudgeResult::TLE,
            _ => JudgeResult::SystemError,
        };

        // submit Compile Error to correct code
        if problem.is_wrong_code && !submit.is_ce {
            match judge_result {
                JudgeResult::AC => return (JudgeResult::WA, None, submit.id()),
                _ => (),
            }
        }

        let error_message = match judge_result {
            JudgeResult::AC => None,
            JudgeResult::SystemError => {
                tracing::debug!(
                    "SystemError: {}",
                    std::str::from_utf8(&result.stderr).unwrap()
                );
                Some("System error: Please contact the competition management.".to_string())
            }
            // get error message from stderr
            _ => std::str::from_utf8(&result.stderr)
                .map(std::string::ToString::to_string)
                .ok(),
        };

        (judge_result, error_message, submit.id())
    } else {
        (
            JudgeResult::SystemError,
            Some("Unknown Result.".to_string()),
            submit.id(),
        )
    }
}

/// Main function.
#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "judge_server=debug");
    }
    tracing_subscriber::fmt::init();

    let repo = RepositoryProvider::new().await;
    let repo_submit = repo.submit();
    let problems = repo.problem().get_all_problems().await;
    let testcases = repo
        .testcase()
        .get_all_testcases(problems.len() as u32)
        .await;

    loop {
        let submits = repo_submit.get_pending_submits().await;
        let works: Vec<_> = submits
            .iter()
            .map(|submit| {
                judge(
                    submit,
                    &problems[submit.problem_id() as usize],
                    &testcases[submit.problem_id() as usize],
                )
            })
            .collect();
        let rets = future::join_all(works).await;

        for ret in rets {
            let (judge_result, error_message, submit_id) = ret;
            repo_submit
                .store_result(
                    judge_result,
                    error_message.unwrap_or(String::new()),
                    submit_id,
                )
                .await;
        }
        sleep(Duration::from_millis(5000)).await;
    }
}
