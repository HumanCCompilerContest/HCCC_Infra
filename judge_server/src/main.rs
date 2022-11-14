use tokio::time::{sleep, Duration};
use tokio::process::Command;
use judge_server::database::new_repo;
use judge_server::entities::JudgeResult;
use judge_server::repositories::submit::Submits;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "judge_server=debug")
    }
    tracing_subscriber::fmt::init();

    let repo = new_repo().await;
    let repo_submit = repo.submit();
    loop {
        let submit = match repo_submit.get_pendding_submit().await {
            Some(s) => s,
            None => {
                continue;
            },
        };

        let result = Command::new("bash")
            .arg("-c")
            .arg(format!(
                "sudo docker run --memory=128M --cpus=\".05\" judge_system /work/judge_system {}",
                base64::encode(&submit.asm)
            ))
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
            repo_submit.store_result(JudgeResult::SystemError, submit.id()).await;
        }
        sleep(Duration::from_millis(10)).await;
    }
}
