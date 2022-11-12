use crate::repositories::Submits;
use crate::entities::JudgeResult;

pub async fn accept_submit(repo: &impl Submits, user_id: i32, submit_id: i32) -> JudgeResult {
    let submit = repo.get_submit(user_id, submit_id)
        .await
        .unwrap();

    dbg!(submit.asm);

    JudgeResult::AC
}

