use crate::entities::Submit;
use crate::repositories::Submits;
use crate::views::Home;

pub async fn create_submit(repo: &impl Submits, asem: &str) {
    let new_submit = Submit::create(asem);
    repo.store(&new_submit).await;
}

pub async fn list_submit(repo: &impl Submits) -> Home {
    let submits = repo.list()
        .await
        .into_iter()
        .map(|x| x.into())
        .collect();

    Home {
        submits,
    }
}


