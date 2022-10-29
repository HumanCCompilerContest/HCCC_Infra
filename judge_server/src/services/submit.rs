use crate::repositories::Submits;
use crate::entities::JudgeResult;

pub async fn accept_submit(repo: &impl Submits, user_id: u32, submit_id: u32) -> JudgeResult {
    let submit = repo.get_submit(user_id, submit_id)
        .await
        .unwrap();

    println!("{}", submit.asem);

    JudgeResult::AC
}

