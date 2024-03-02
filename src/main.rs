use anyhow::{bail, Context};
use clap::Parser;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::{
    collections::VecDeque,
    io::{self, BufRead, BufReader},
    time::Duration,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "y")]
    prompt: String,
    #[arg(short, long, default_value = "500ms")]
    interval: humantime::Duration,
    #[arg(raw = true)]
    command: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 1000,
            cols: 1000,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("failed to open pty")?;

    let mut cmdargs: VecDeque<_> = args.command.into();

    let Some(command) = cmdargs.pop_front() else {
        bail!("must provide a command")
    };

    let mut cmd = CommandBuilder::new(command);

    if let Ok(cwd) = std::env::current_dir() {
        println!("{cwd:?}");
        cmd.cwd(cwd);
    }

    cmd.env("TERM", "xterm-256color");
    cmd.env("COLORTERM", "truecolor");
    cmd.env("ANSICON", "1");
    cmd.env("ConEmuANSI", "ON");
    cmd.env("LANG", "en_US.UTF-8");
    cmd.env("LC_ALL", "en_US.UTF-8");

    cmd.args(cmdargs);

    let mut child = pair.slave.spawn_command(cmd)?;
    let master = pair.master;
    let mut writer = master.take_writer()?;

    let mut reader = BufReader::new(master.try_clone_reader()?);
    let mut line = String::with_capacity(1000);

    let prompt = args.prompt;
    let interval: Duration = args.interval.into();

    std::thread::spawn(move || loop {
        writeln!(&mut writer, "{prompt}").expect(&format!("failed to write '{prompt}' to command"));
        std::thread::sleep(interval);
    });

    std::thread::spawn(move || loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                print!("{line}");
                line.clear();
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::WouldBlock {
                    eprintln!("Error reading from PTY: {:?}", e);
                    break;
                }
            }
        }
    });

    child.wait()?;

    Ok(())
}
