use futures::future;
use judge_server::database::new_repo;
use judge_server::entities::{JudgeResult, Submit};
use judge_server::repositories::submit::Submits;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

async fn judge(submit: &Submit) -> (JudgeResult, i32) {
    const TESTCASE_PATH: &str = "/home/ubuntu/HCCC_Infra/judge_container/testcase/";
    const CONTAINER_NAME: &str =
        "ghcr.io/humanccompilercontest/hccc_infra:judge_container-hccc_001";
    let result = Command::new("bash")
        .arg("-c")
        .arg(dbg!(format!(
            "sudo docker run --rm --memory=128M --cpus=\"0.05\" -v '{}:/work/testcase/' {} {} {} {}",
            TESTCASE_PATH,
            CONTAINER_NAME,
            submit.problem_id,
            dbg!(submit.is_ce),
            base64::encode(&submit.asm)
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

    /*
    let testcase_path = Command::new("bash")
        .arg("-c")
        .arg("realpath ../judge_container/testcase/")
        .output()
        .await
        .unwrap()
        .stdout;
    let testcase_path = String::from_utf8_lossy(&testcase_path);
    */

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
