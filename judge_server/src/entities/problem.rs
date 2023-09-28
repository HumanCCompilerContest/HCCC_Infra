use postgres_types::FromSql;
use serde::{Deserialize, Serialize};

/// Answer output
#[derive(FromSql, Serialize, Deserialize, Debug)]
#[postgres(name = "testtarget")]
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

/// Architecture
#[derive(FromSql, Serialize, Deserialize, Debug)]
pub enum Arch {
    /// x86_64
    #[serde(rename = "x8664")]
    X8664,
    /// RISC-V.
    #[serde(rename = "riscv")]
    RiscV,
}

/// Problem data for judge submits.
#[derive(FromSql)]
pub struct Problem {
    /// Submit id.
    _id: i32,
    /// Target Architecture.
    pub arch: Arch,
    /// Test target
    pub test_target: TestTarget,
    /// Is wrong code or not.
    pub is_wrong_code: bool,
    /// Line number where the first syntax discrepancy occurs.
    pub error_line_number: Option<i32>,
}

impl Problem {
    #[must_use]
    pub fn new(
        id: i32,
        arch: Arch,
        test_target: TestTarget,
        is_wrong_code: bool,
        error_line_number: Option<i32>,
    ) -> Self {
        Problem {
            _id: id,
            arch,
            test_target,
            is_wrong_code,
            error_line_number,
        }
    }
}
