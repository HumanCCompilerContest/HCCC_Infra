use judge_server::database::new_repo;
use judge_server::entities::{Submit, JudgeResult};
use judge_server::repositories::submit::Submits;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

async fn judge(repo_submit: &dyn Submits, submit: Submit) {
    let result = Command::new("bash")
        .arg("-c")
        .arg(dbg!(format!(
            "sudo docker run --memory=128M --cpus=\"0.05\" ghcr.io/alignof/hccc_infra:judge_system-develop {}",
            base64::encode(&submit.asm)
        )))
        .output()
        .await;

    if let Ok(result) = result {
        let judge_result = match result.status.code().unwrap_or(6) {
            0 => JudgeResult::AC,
            1 => JudgeResult::WA,
            2 => JudgeResult::AE,
            3 => JudgeResult::LE,
            4 => JudgeResult::RE,
            5 => JudgeResult::TLE,
            _ => JudgeResult::SystemError,
        };

        repo_submit.store_result(judge_result, submit.id()).await;
    } else {
        repo_submit
            .store_result(JudgeResult::SystemError, submit.id())
            .await;
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
        for submit in submits {
            judge(&repo_submit, submit).await;
        }

        sleep(Duration::from_millis(5000)).await;
    }
}
