use super::*;
use std::process::Command;
use super::errors::*;
use serde_json::error::Category;
use std::thread;
use std::time::Duration;
#[test]
fn gpsfake_basic() {
    let mut cmd = Command::new("gpsfake")
        .args(&["-1", "stockholm_walk.nmea"])
        .spawn()
        .unwrap();
    thread::sleep(Duration::from_millis(1000));
    let mut conn = GpsdConnection::new("127.0.0.1:2947").unwrap();
    conn.set_read_timeout(Some(Duration::from_millis(1000))).unwrap();
    conn.watch(true).unwrap();
    loop {
        if cmd.try_wait().unwrap().is_some() {
            return;
        }
        let resp = conn.get_response();
        if let Err(e) = resp {
            if let &ErrorKind::DeserFailed(_, ref e) = e.kind() {
                if let Category::Eof = e.classify() {
                    continue;
                }
            }
            if let &ErrorKind::Io(..) = e.kind() {
                return;
            }
            panic!("error: {:?}", e);
        }
    }
}
#[test]
fn gpsfake_poll() {
    Command::new("gpsfake")
        .args(&["-1", "stockholm_walk.nmea"])
        .spawn()
        .unwrap();
    thread::sleep(Duration::from_millis(1000));
    let mut conn = GpsdConnection::new("127.0.0.1:2947").unwrap();
    conn.set_read_timeout(Some(Duration::from_millis(1000))).unwrap();
    thread::sleep(Duration::from_millis(1000));
    conn.poll().unwrap();
    let resp = conn.get_response();
    if let Err(e) = resp {
        if let &ErrorKind::DeserFailed(_, ref e) = e.kind() {
            if let Category::Eof = e.classify() {
                return;
            }
        }
        if let &ErrorKind::Io(..) = e.kind() {
            return;
        }
        panic!("error: {:?}", e);
    }
}
