use postgres_types::FromSql;
use serde::Serialize;

/// Testcase
#[derive(FromSql, Serialize)]
pub struct Testcase {
    id: i32,
    /// Problem ID.
    problem_id: i32,
    /// Input of testcase.
    input: Option<String>,
    /// Expect result.
    expect: Option<String>,
}

impl Testcase {
    #[must_use]
    pub fn new(id: i32, problem_id: i32, input: Option<String>, expect: Option<String>) -> Self {
        Testcase {
            id,
            problem_id,
            input,
            expect,
        }
    }
}
