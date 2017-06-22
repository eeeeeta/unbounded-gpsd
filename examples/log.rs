extern crate gpsd;

use gpsd::*;

fn main() {
    let mut conn = GpsdConnection::new("127.0.0.1:2947").unwrap();
    conn.watch(true).unwrap();
    loop {
        let resp = conn.get_response();
        if let Err(resp) = resp {
            println!("{:?}", resp);
        }
    }
}
