mod run_test;

use std::fs::File;
use std::io::Write;
use tokio::process::Command;
use clap::arg;

#[allow(dead_code)]
pub enum ExitCode {
    AC = 0,
    WA,
    AE,
    LE,
    RE,
    TLE,
    SystemError,
}

pub enum ExeOption {
    JustRun,
    ExitCode,
    Output,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = clap::app_from_crate!()
        .arg(arg!(--justrun ... "just run").required(false))
        .arg(arg!(--exitcode <testcase_number> ... "check exitcode").required(false))
        .arg(arg!(--output <testcase_number> "check stdout").required(false))
        .arg(arg!(<asm> "assembly"))
        .get_matches();
    let flag_map = | | {
        (
            app.is_present("justrun"),
            app.is_present("exitcode"),
            app.is_present("output"),
        )
    };
    let exe_option = match flag_map() {
        (true, _, _) => ExeOption::JustRun,
        (_, true, _) => ExeOption::ExitCode,
        (_, _, true) => ExeOption::Output,
        _ => ExeOption::JustRun,
    };

    let asm = match app.value_of("asm") {
        Some(a) => String::from_utf8(base64::decode(a)?)?,
        None => panic!("please specify target ELF file."),
    };
    let mut file = File::create("./submit.s")?;
    writeln!(file, "{}", dbg!(asm))?;
    file.flush()?;

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

    match exe_option {
        ExeOption::JustRun => run_test::just_exec().await,
        ExeOption::ExitCode => {
            let testcase_number = app.value_of("exitcode").unwrap();
            run_test::with_testcase(testcase_number, exe_option).await;
        },
        ExeOption::Output => {
            let testcase_number = app.value_of("output").unwrap();
            run_test::with_testcase(testcase_number, exe_option).await;
        },
    }

    Ok(())
}
