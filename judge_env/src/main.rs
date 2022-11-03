use std::fs::File;
use tokio::process::Command;
use std::io::Write;
use std::time::Duration;

#[allow(dead_code)]
enum ExitCode {
    AC = 0,
    WA,
    AE,
    LE,
    RE,
    TLE,
    SystemError,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const TLE_MSEC: u64 = 2000;
    let args: Vec<String> = std::env::args().collect();
    let mut file = File::create("./submit.s")?;
    writeln!(file, "{}", args[1])?;
    file.flush()?;

    let object = Command::new("bash")
        .arg("-c")
        .arg("as submit.s -o tmp.o")
        .output()
        .await;

    if let Err(_) = object {
        eprintln!("Assembling Error");
        std::process::exit(ExitCode::AE as i32);
    }

    let elf = Command::new("bash")
        .arg("-c")
        .arg("gcc -v -static -no-pie tmp.o -o test_target")
        .output()
        .await;

    if let Err(_) = elf {
        eprintln!("Linking Error");
        std::process::exit(ExitCode::LE as i32);
    }

    let test = tokio::time::timeout(
        Duration::from_millis(TLE_MSEC as u64),
        Command::new("bash")
            .kill_on_drop(true)
            .arg("-c")
            .arg("./test_target")
            .output()
    )
    .await
    .unwrap_or_else(|_| {
        eprintln!("Time Limit Exceeded");
        std::process::exit(ExitCode::TLE as i32);
    });

    let output = match test {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Runtime Error");
            std::process::exit(ExitCode::RE as i32);
        },
    };

    println!("{:?}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
