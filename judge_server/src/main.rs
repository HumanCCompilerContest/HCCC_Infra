use futures::future;
use judge_server::database::new_repo;
use judge_server::entities::{JudgeResult, Submit};
use judge_server::repositories::submit::Submits;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

async fn judge(submit: &Submit) -> (JudgeResult, i32) {
    let result = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "sudo docker run --rm --memory=128M --cpus=\"0.05\" ghcr.io/alignof/hccc_infra:judge_system-develop {} {} {}",
            submit.problem_id,
            submit.is_ce,
            base64::encode(&submit.asm)
        ))
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

        (judge_result, submit.id())
    } else {
        (JudgeResult::SystemError, submit.id())
    }
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "judge_server=debug")
    }
    tracing_subscriber::fmt::init();

    let repo = new_repo().await;
    let repo_submit = repo.submit();
    loop {
        let submits = repo_submit.get_pending_submits().await;
        let works: Vec<_> = submits.iter().map(|submit| judge(submit)).collect();
        let rets = future::join_all(works).await;

        for ret in rets {
            let (judge_result, submit_id) = ret;
            repo_submit.store_result(judge_result, submit_id).await;
        }
        sleep(Duration::from_millis(5000)).await;
    }
}
