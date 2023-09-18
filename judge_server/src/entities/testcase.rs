use postgres_types::FromSql;

/// Testcase
#[derive(FromSql)]
pub struct Testcase {
    id: i32,
    /// Problem ID.
    problem_id: i32,
    /// Input of testcase.
    input: String,
    /// Expect result.
    expect: String,
}

impl Testcase {
    pub fn new(id: i32, problem_id: i32, input: String, expect: String) -> Self {
        Testcase {
            id,
            problem_id,
            input,
            expect,
        }
    }
}
