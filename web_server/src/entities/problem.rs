use serde::Serialize;

#[derive(Serialize)]
pub struct ProblemObject {
    id: i32,
    title: String,
    statement: String,
	code: String,
	input_desc: String,
	output_desc: String,
	score: i32,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Problem {
    status: String,
    problem: ProblemObject,
    errorMessage: Option<String>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct AllProblems {
    status: String,
    problems: Vec<ProblemObject>,
    errorMessage: Option<String>,
}

impl ProblemObject {
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

    pub fn dummy() -> Self{
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
    pub fn new(
        status: String,
        id: i32,
        title: String,
        statement: String,
        code: String,
        input_desc: String,
        output_desc: String,
        score: i32,
        error_message: Option<String>
    ) -> Self {
        Problem {
            status,
            problem: ProblemObject::new(id, title, statement, code, input_desc, output_desc, score,),
            errorMessage: error_message,
        }
    }

    pub fn error() -> Self {
        Problem {
            status: "ng".to_string(),
            problem: ProblemObject::dummy(),
            errorMessage: Some("problem not found".to_string()),
        }
    }
}


impl AllProblems {
    pub fn new(
        status: String,
        problems: Vec<ProblemObject>,
        error_message: Option<String>
    ) -> Self {
        AllProblems {
            status,
            problems,
            errorMessage: error_message,
        }
    }
}
