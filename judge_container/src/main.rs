mod run_test;

use crate::run_test::Testcases;
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

pub enum ExeOption {
    JustRun,
    ExitCode,
    Output,
}

struct CmdOption {
    exe_option: ExeOption,
    is_ce: bool,
    asm: String,
    problem_path: String,
}

fn get_arg() -> Result<CmdOption, Box<dyn std::error::Error>> {
    let app = clap::app_from_crate!()
        .arg(arg!(--justrun <problem_number> ... "just run").required(false))
        .arg(arg!(--exitcode <problem_number> ... "check exitcode").required(false))
        .arg(arg!(--output <problem_number> "check stdout").required(false))
        .arg(arg!(<is_ce> "is compile error"))
        .arg(arg!(<asm> "assembly"))
        .get_matches();

    let flag_map = || {
        (
            app.is_present("justrun"),
            app.is_present("exitcode"),
            app.is_present("output"),
        )
    };
    let (exe_option, problem_num) = match flag_map() {
        (true, _, _) => (
            ExeOption::JustRun,
            app.value_of("justrun").expect("missing problem_num"),
        ),
        (_, true, _) => (
            ExeOption::ExitCode,
            app.value_of("exitcode").expect("missing problem_num"),
        ),
        (_, _, true) => (
            ExeOption::Output,
            app.value_of("output").expect("missing problem_num"),
        ),
        _ => (
            ExeOption::JustRun,
            app.value_of("justrun").expect("missing problem_num"),
        ),
    };

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
        exe_option,
        is_ce,
        asm,
        problem_path: format!("./problem/case{}.json", problem_num),
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
    let testcases: Testcases = serde_json::from_str(&cmd.problem_path).unwrap();

    if cmd.is_ce && !testcases.is_wrong_code {
        eprintln!("wrong compile error!");
        std::process::exit(ExitCode::WC as i32);
    }

    let mut file = File::create("./submit.s")?;
    writeln!(file, "{}", dbg!(cmd.asm))?;
    file.flush()?;

    create_elf().await;

    match cmd.exe_option {
        ExeOption::JustRun => run_test::just_exec().await,
        ExeOption::ExitCode => {
            run_test::with_testcase(testcases, cmd.exe_option).await;
        }
        ExeOption::Output => {
            run_test::with_testcase(testcases, cmd.exe_option).await;
        }
    }

    Ok(())
}
