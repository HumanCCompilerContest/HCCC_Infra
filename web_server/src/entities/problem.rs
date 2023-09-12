use postgres_types::{FromSql, ToSql};
use serde::Serialize;

/// Problem data.
#[derive(Serialize, Debug, ToSql, FromSql)]
pub struct ProblemObject {
    /// Problem id.
    id: i32,
    /// Problem title.
    title: String,
    /// Problem statement.
    statement: String,
    /// C source code.
    code: String,
    /// Description of input.
    input_desc: String,
    /// Description of output.
    output_desc: String,
    /// Score.
    score: i32,
}

/// Api data for `/api/problem/:id`.
#[derive(Serialize, Debug, ToSql, FromSql)]
pub struct Problem {
    /// Getting problem successeed or not.
    /// * `ok` - successeed
    /// * `ng` - failed
    status: String,
    /// User selected problem.
    problem: ProblemObject,
    /// Error message.
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
}

/// Api data for `/api/problem/`.
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct AllProblems {
    /// Getting all problems successeed or not.
    /// * `ok` - successeed
    /// * `ng` - failed
    status: String,
    /// All problems.
    #[serde(rename = "items")]
    problems: Vec<ProblemObject>,
    /// Error message.
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
}

impl ProblemObject {
    /// Return new `ProblemObject`.
    pub fn new(
        id: i32,
        title: String,
        statement: String,
        code: String,
        input_desc: String,
        output_desc: String,
        score: i32,
    ) -> Self {
        ProblemObject {
            id,
            title,
            statement,
            code,
            input_desc,
            output_desc,
            score,
        }
    }

    /// Return dummy `ProblemObject`.
    pub fn dummy() -> Self {
        ProblemObject {
            id: 0,
            title: String::new(),
            statement: String::new(),
            code: String::new(),
            input_desc: String::new(),
            output_desc: String::new(),
            score: 0,
        }
    }
}

impl Problem {
    /// Return new `Problem`.
    pub fn new(
        id: i32,
        title: String,
        statement: String,
        code: String,
        input_desc: String,
        output_desc: String,
        score: i32,
    ) -> Self {
        Problem {
            status: "ok".to_string(),
            problem: ProblemObject::new(id, title, statement, code, input_desc, output_desc, score),
            error_message: None,
        }
    }

    /// Return error response.
    pub fn error(status: &str, msg: &str) -> Self {
        Problem {
            status: status.to_string(),
            problem: ProblemObject::dummy(),
            error_message: Some(msg.to_string()),
        }
    }

    /// Return problem object.
    pub fn get_object(self) -> ProblemObject {
        self.problem
    }
}

impl AllProblems {
    pub fn new(
        status: String,
        problems: Vec<ProblemObject>,
        error_message: Option<String>,
    ) -> Self {
        AllProblems {
            status,
            problems,
            error_message,
        }
    }

    pub fn error(status: &str, msg: &str) -> Self {
        AllProblems {
            status: status.to_string(),
            problems: Vec::new(),
            error_message: Some(msg.to_string()),
        }
    }
}
