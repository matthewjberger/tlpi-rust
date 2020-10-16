use std::{env};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, read, write};
use anyhow::{Result, bail};

const BUF_SIZE: usize = 1024;

fn main() -> Result<()> {
    let mut buf = [0u8; BUF_SIZE];
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args[1] == "--help" {
        println!("Usage: {} old-file new-file", args[0]);
        return Ok(());
    }
    let input_fd = open(args[1].as_str(), OFlag::O_RDONLY, Mode::empty())?;

    let open_flags = OFlag::O_CREAT | OFlag::O_WRONLY | OFlag::O_TRUNC;
    let file_perms = Mode::S_IRUSR
        | Mode::S_IWUSR
        | Mode::S_IRGRP
        | Mode::S_IWGRP
        | Mode::S_IROTH
        | Mode::S_IWOTH;

    let output_fd = open(args[2].as_str(), open_flags, file_perms)?;
 loop {
        let num_read = match read(input_fd, &mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                bail!("Error reading old-file: {}", e);
            }
        };

        match write(output_fd, &buf[..num_read]) {
            Ok(n) => {
                if n != num_read {
                    bail!("Couldn't write the whole buffer.");
                }
            }
            Err(e) => {
                bail!("Error writing new-file: {}", e);
            }
        };
    }

    close(input_fd)?;
    close(output_fd)?;
    Ok(())
}