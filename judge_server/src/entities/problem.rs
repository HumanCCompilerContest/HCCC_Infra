use postgres_types::FromSql;
use serde::Deserialize;

/// Answer output
#[derive(FromSql, Deserialize, Debug)]
pub enum TestTarget {
    /// Exit status of `$ bash -c ./test_target`.
    #[serde(rename = "exitcode")]
    ExitCode,
    /// Output from stdout.
    #[serde(rename = "stdout")]
    StdOut,
    /// No test case.
    #[serde(rename = "none")]
    NoTestCase,
}

/// Problem data for judge submits.
#[derive(FromSql)]
pub struct Problem {
    /// Submit id.
    _id: i32,
    /// Test target
    pub test_target: TestTarget,
    /// Is wrong code or not.
    pub is_wrong_code: bool,
}

impl Problem {
    pub fn new(id: i32, test_target: TestTarget, is_wrong_code: bool) -> Self {
        Problem {
            _id: id,
            test_target,
            is_wrong_code,
        }
    }
}
