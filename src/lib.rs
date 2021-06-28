//! A small crate to interface with gpsd, based on the server JSON protocol.
//!
//! This crate uses the [log](https://crates.io/crates/log) crate for debug logging.
//! Logs will only appear if the logging apparatus is correctly configured. As such,
//! if you're filing an issue, we would appreciate it if you did this and gave us the
//! relevant logs!

extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate chrono;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;

use std::net::{ToSocketAddrs, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::time::Duration;

pub mod errors {
    //! Error handling, using error-chain.

    error_chain! {
        types {
            Error, ErrorKind, ResultExt, GpsdResult;
        }
        foreign_links {
            Io(::std::io::Error);
            Serde(::serde_json::Error);
        }
        errors {
            DeserFailed(s: String, e: ::serde_json::Error) {
                display("failed to deserialize text '{}': {}", s, e)
            }
            GpsdFailed(s: String) {
                display("gpsd connection closed")
            }
        }
    }
}
pub use errors::GpsdResult;
pub mod types;
#[cfg(test)]
pub mod tests;
use types::*;

/// A connection to gpsd.
pub struct GpsdConnection {
    raw_data: bool,
    inner: BufReader<TcpStream>
}
impl GpsdConnection {
    /// Make a new connection to a given address.
    pub fn new<A: ToSocketAddrs>(addr: A) -> GpsdResult<Self> {
        let stream = TcpStream::connect(addr)?;
        let inner = BufReader::new(stream);
        Ok(Self { inner, raw_data: false })
    }
    /// Enable or disable watcher mode.
    fn _watch(&mut self, watch: bool, json: bool, raw: u8) -> GpsdResult<()> {
        let stream = self.inner.get_mut();
        let watch_data = json!({
            "class": "WATCH",
            "enable": watch,
            "json": json,
            "raw": raw,
        });
        self.raw_data = raw > 0;
        let msg = format!("?WATCH={}\n", watch_data.to_string());
        stream.write_all(msg.as_bytes())?;
        Ok(())
    }
    /// Enable or disable watcher mode.
    pub fn watch(&mut self, watch: bool) -> GpsdResult<()> {
        self._watch(watch, true, 0)
    }
    /// Enable RAW mode. In RAW mode, gpsd sends raw data from the GPS device, depending on the value of `raw`:
    ///
    /// When this attribute is set to 1 for a channel, gpsd reports the unprocessed NMEA or
    /// AIVDM data stream from whatever device is attached. Binary GPS packets are hex-dumped.
    /// RTCM2 and RTCM3 packets are not dumped in raw mode. When this attribute is set to 2 for a channel that
    /// processes binary data, gpsd reports the received data verbatim without hex-dumping.
    pub fn watch_raw(&mut self, watch: bool, json: bool, raw: u8) -> GpsdResult<()> {
        self._watch(watch, json, raw)
    }
    /// The POLL command requests data from the last-seen fixes on all active
    /// GPS devices. Devices must previously have been activated by ?WATCH to be
    /// pollable.
    pub fn poll(&mut self) -> GpsdResult<()> {
        let stream = self.inner.get_mut();
        stream.write_all("?POLL;\n".as_bytes())?;
        Ok(())
    }
    /// Ask for the server's version (triggers a Response::Version).
    pub fn version(&mut self) -> GpsdResult<()> {
        let stream = self.inner.get_mut();
        stream.write_all("?VERSION;\n".as_bytes())?;
        Ok(())
    }
    /// Ask for the server's devices (triggers a Response::Devices)
    pub fn devices(&mut self) -> GpsdResult<()> {
        let stream = self.inner.get_mut();
        stream.write_all("?DEVICES;\n".as_bytes())?;
        Ok(())
    }
    /// Sets the read timeout for `get_response`.
    ///
    /// A value of `None` implies that the read will never block.
    pub fn set_read_timeout(&mut self, dur: Option<Duration>) -> GpsdResult<()> {
        self.inner.get_ref().set_read_timeout(dur)?;
        Ok(())
    }
    /// Polls for responses from GPSD, blocking if necessary.
    ///
    /// Ideally, you run this in a loop somewhere to process messages.
    pub fn get_response(&mut self) -> GpsdResult<Response> {
        loop {
            let mut buf = String::new();
            let read_result = self.inner.read_line(&mut buf);

            if let Ok(size) = read_result {
                if size == 0 {
                    bail!(errors::ErrorKind::GpsdFailed(String::from("Gpsd Connection Closed")));
                }
            }

            if buf == "" {
                debug!("empty line received from GPSD");
                continue;
            }
            debug!("raw GPSD data: {}", buf);
            let data = serde_json::from_str(&buf);
            debug!("serde output: {:?}", data);
            match data {
                Err(e) => {
                    if self.raw_data {
                        return Ok(Response::Raw(buf))
                    } else {
                        debug!("deserializing response failed: {:?}", e);
                        bail!(errors::ErrorKind::DeserFailed(buf, e));
                    }
                },
                Ok(x) => return Ok(x)
            }
        }
    }
}
