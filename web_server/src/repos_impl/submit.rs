use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::Submit;
use crate::repositories::Submits;

pub struct SubmitImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Submits for SubmitImpl<'a> {
    async fn find(&self, id: i32) -> Option<Submit> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM submits WHERE id = $1", &[&id])
            .await
            .unwrap();

        row.map(|r| r.into())
    }

    async fn list(&self) -> Vec<Submit> {
        let conn = self.pool.get().await.unwrap();
        let rows = conn
            .query("SELECT * FROM submits ORDER BY time DESC", &[])
            .await
            .unwrap();

        rows.into_iter()
            .map(|r| r.into())
            .collect()
    }
    
    async fn store(&self, entity: &Submit) {
        let conn = self.pool.get().await.unwrap();
        dbg!(&entity.result);
        conn.execute(
            "INSERT INTO submits (user_id, time, asem, result) VALUES ($1, $2, $3, $4)",
            &[&entity.user_id, &entity.time, &entity.asem, &entity.result],
        )
        .await
        //.ok();
        .unwrap();
    }
}

impl From<Row> for Submit {
    fn from(r: Row) -> Self {
        Submit::new(
            r.get("id"),
            r.get("user_id"),
            r.get("time"),
            r.get("asem"),
            r.get("result"),
        )
    }
}


