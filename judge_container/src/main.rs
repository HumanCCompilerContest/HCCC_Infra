mod run_test;

use crate::run_test::{TestTarget, Testcases};
use clap::arg;
use std::fs::File;
use std::io::Write;
use tokio::process::Command;

#[allow(dead_code)]
pub enum ExitCode {
    AC = 0,
    WA,
    WC,
    AE,
    LE,
    RE,
    TLE,
    Pending,
    SystemError,
}

struct CmdOption {
    is_ce: bool,
    asm: String,
    problem_path: String,
}

fn get_arg() -> Result<CmdOption, Box<dyn std::error::Error>> {
    let app = clap::app_from_crate!()
        .arg(arg!(<problem_number> "number for testcase"))
        .arg(arg!(<is_ce> "is compile error"))
        .arg(arg!(<asm> "assembly"))
        .get_matches();

    let problem_num = app.value_of("problem_number").unwrap();
    let is_ce = match app.value_of("is_ce") {
        Some(a) => match a {
            "true" => true,
            _ => false,
        },
        None => panic!("please specify target ELF file."),
    };
    let asm = match app.value_of("asm") {
        Some(a) => String::from_utf8(base64::decode(a)?)?,
        None => panic!("please input assembly."),
    };

    Ok(CmdOption {
        is_ce,
        asm,
        problem_path: format!("/work/testcase/case{problem_num}.json"),
    })
}

async fn create_elf() {
    // assemble
    let exit_code_asm = Command::new("bash")
        .arg("-c")
        .arg("as submit.s -o tmp.o")
        .output()
        .await
        .unwrap_or_else(|_| {
            eprintln!("Assembling Error");
            std::process::exit(ExitCode::AE as i32);
        })
        .status
        .code()
        .unwrap_or(-1);

    if exit_code_asm != 0 {
        eprintln!("Assembling Error");
        std::process::exit(ExitCode::AE as i32);
    }

    // link
    let exit_code_link = Command::new("bash")
        .arg("-c")
        .arg("gcc -v -static -no-pie tmp.o -o test_target")
        .output()
        .await
        .unwrap_or_else(|_| {
            eprintln!("Linking Error");
            std::process::exit(ExitCode::LE as i32);
        })
        .status
        .code()
        .unwrap_or(-1);

    if exit_code_link != 0 {
        eprintln!("Linking Error");
        std::process::exit(ExitCode::LE as i32);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = get_arg().unwrap();
    let testcase_str = std::fs::read_to_string(&cmd.problem_path).unwrap_or_else(|_| {
        eprintln!("Failed to read file: {}", &cmd.problem_path);
        std::process::exit(ExitCode::SystemError as i32);
    });
    let testcases: Testcases = serde_json::from_str(&testcase_str).unwrap();

    if cmd.is_ce && !testcases.is_wrong_code {
        eprintln!("wrong compile error!");
        std::process::exit(ExitCode::WC as i32);
    }

    if cmd.is_ce && testcases.is_wrong_code {
        std::process::exit(ExitCode::AC as i32);
    }

    let mut file = File::create("./submit.s")?;
    writeln!(file, "{}", cmd.asm)?;
    file.flush()?;

    create_elf().await;

    match testcases.judge_target {
        TestTarget::NoTestCase => run_test::just_exec().await,
        TestTarget::ExitCode | TestTarget::StdOut => {
            run_test::with_testcase(testcases).await;
        }
    }

    Ok(())
}
