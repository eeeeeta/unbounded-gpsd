//! Types employed in the GPSD API.
use chrono::*;

fn serde_true() -> bool { true }
fn serde_false() -> bool { false }

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged, deny_unknown_fields)]
/// A time-position-velocity (TPV) report.
///
/// The API here splits the TPV object that GPSD sends into various variants, in
/// a bid to classify common responses so that you don't have to do this
/// yourself. See the variant documentation for details.
///
/// Basically, the aim here is to reduce the amount of Option unwrapping
/// you have to do, as gpsd specifies that all these fields are optional.
///
/// The field documentation is exactly the same across variants; it may be omitted
/// for brevity.
///
/// # Error estimates
///
/// Fields ending `_err` denote error estimates. These are given in the units of
/// their respective fields: for example, `alt_err` is the altitude error, given
/// in meters. All errors are delivered with 95% confidence.
pub enum TpvResponse {
    /// 3D GPS fix, with speed and climb data.
    Fix3D {
        /// Name of originating device.
        device: Option<String>,
        /// Timestamp.
        time: DateTime<Utc>,
        /// Fix type: 0 = unknown, 1 = no fix, 2 = 2D fix, 3 = 3D fix.
        mode: u8,
        /// Estimated timestamp error (seconds, 95% confidence).
        #[serde(rename = "ept")]
        time_err: f64,
        /// Latitude in degrees: +/- signifies North/South. Present when mode is 2 or 3.
        lat: f64,
        #[serde(rename = "epy")]
        lat_err: Option<f64>,
        /// Longitude in degrees: +/- signifies East/West. Present when mode is 2 or 3.
        lon: f64,
        #[serde(rename = "epx")]
        lon_err: Option<f64>,
        /// Altitude in meters. Present if mode is 3.
        alt: f64,
        #[serde(rename = "epv")]
        alt_err: Option<f64>,
        /// Course over ground, degrees from true north.
        track: Option<f64>,
        #[serde(rename = "epd")]
        track_err: Option<f64>,
        /// Speed over ground, meters per second.
        speed: f64,
        #[serde(rename = "eps")]
        speed_err: Option<f64>,
        /// Climb (positive) or sink (negative) rate, meters per second.
        climb: f64,
        #[serde(rename = "epc")]
        climb_err: Option<f64>
    },
    /// 2D GPS fix, with speed data.
    Fix2D {
        /// Name of originating device.
        device: Option<String>,
        /// Timestamp.
        time: DateTime<Utc>,
        /// Fix type: 0 = unknown, 1 = no fix, 2 = 2D fix, 3 = 3D fix.
        mode: u8,
        /// Estimated timestamp error (seconds, 95% confidence).
        #[serde(rename = "ept")]
        time_err: f64,
        /// Latitude in degrees: +/- signifies North/South. Present when mode is 2 or 3.
        lat: f64,
        #[serde(rename = "epy")]
        lat_err: Option<f64>,
        /// Longitude in degrees: +/- signifies East/West. Present when mode is 2 or 3.
        lon: f64,
        #[serde(rename = "epx")]
        lon_err: Option<f64>,
        /// Course over ground, degrees from true north.
        track: Option<f64>,
        #[serde(rename = "epd")]
        track_err: Option<f64>,
        /// Speed over ground, meters per second.
        speed: f64,
        #[serde(rename = "eps")]
        speed_err: Option<f64>,
    },
    /// Fix with lat/lon, and an unknown smattering of fields.
    /// You'll get this variant if a fix is obtained (lat/lon available), but GPSD
    /// otherwise sent data that doesn't exactly fit into any of the categories above.
    ///
    /// If you are getting this variant, we'd greatly appreciate it if you filed an issue,
    /// so we can see what sort of strange data your GPSD is sending!
    LatLonOnly {
        /// Name of originating device.
        device: Option<String>,
        /// Timestamp.
        time: DateTime<Utc>,
        /// Fix type: 0 = unknown, 1 = no fix, 2 = 2D fix, 3 = 3D fix.
        mode: u8,
        /// Estimated timestamp error (seconds, 95% confidence).
        #[serde(rename = "ept")]
        time_err: f64,
        /// Latitude in degrees: +/- signifies North/South. Present when mode is 2 or 3.
        lat: f64,
        #[serde(rename = "epy")]
        lat_err: Option<f64>,
        /// Longitude in degrees: +/- signifies East/West. Present when mode is 2 or 3.
        lon: f64,
        #[serde(rename = "epx")]
        lon_err: Option<f64>,
        /// Altitude in meters. Present if mode is 3.
        alt: Option<f64>,
        #[serde(rename = "epv")]
        alt_err: Option<f64>,
        /// Course over ground, degrees from true north.
        track: Option<f64>,
        #[serde(rename = "epd")]
        track_err: Option<f64>,
        /// Speed over ground, meters per second.
        speed: Option<f64>,
        #[serde(rename = "eps")]
        speed_err: Option<f64>,
        /// Climb (positive) or sink (negative) rate, meters per second.
        climb: Option<f64>,
        #[serde(rename = "epc")]
        climb_err: Option<f64>,
    },
    /// No fix.
    NoFix {
        /// Name of originating device.
        device: Option<String>,
        /// Timestamp.
        time: DateTime<Utc>,
        /// Fix type: 0 = unknown, 1 = no fix, 2 = 2D fix, 3 = 3D fix.
        mode: u8
    },
    /// Possibly no useful data whatsoever.
    Nothing {
        /// Name of originating device.
        device: Option<String>,
        /// Timestamp.
        time: Option<DateTime<Utc>>,
        /// Fix type: 0 = unknown, 1 = no fix, 2 = 2D fix, 3 = 3D fix.
        mode: Option<u8>
    },
    /// Something else! You'll get this variant if GPSD sent data that doesn't
    /// exactly fit into any of the categories above.
    ///
    /// If you are getting this variant, we'd greatly appreciate it if you filed an issue,
    /// so we can see what sort of strange data your GPSD is sending!
    Dustbin {
        device: Option<String>,
        time: Option<DateTime<Utc>>,
        mode: Option<u8>,
        #[serde(rename = "ept")]
        time_err: Option<f64>,
        lat: Option<f64>,
        #[serde(rename = "epy")]
        lat_err: Option<f64>,
        lon: Option<f64>,
        #[serde(rename = "epx")]
        lon_err: Option<f64>,
        alt: Option<f64>,
        #[serde(rename = "epv")]
        alt_err: Option<f64>,
        track: Option<f64>,
        #[serde(rename = "epd")]
        track_err: Option<f64>,
        speed: Option<f64>,
        #[serde(rename = "eps")]
        speed_err: Option<f64>,
        climb: Option<f64>,
        #[serde(rename = "epc")]
        climb_err: Option<f64>,
    },
}
/// A single satellite.
#[derive(Serialize, Deserialize, Debug)]
pub struct SatelliteObject {
    #[serde(rename = "PRN")]
    /// PRN ID of the satellite. 1-63 are GNSS satellites, 64-96 are GLONASS
    /// satellites, 100-164 are SBAS satellites
    pub prn: u16,
    #[serde(rename = "az")]
    /// Azimuth, degrees from true north.
    pub azimuth: u32,
    #[serde(rename = "el")]
    /// Elevation in degrees.
    pub elevation: u32,
    #[serde(rename = "ss")]
    /// Signal strength in dB.
    pub signal_strength: u32,
    /// Used in current solution? (SBAS/WAAS/EGNOS satellites may be flagged
    /// used if the solution has corrections from them, but not all drivers make
    /// this information available.)
    pub used: bool
}
#[derive(Serialize, Deserialize, Debug)]
/// A sky view report (SKY) of GPS satellite positions.
///
/// If there is no GPS device available, or no skyview has been reported yet,
/// all fields will be blank.
///
/// # Dilutions of precision
///
/// Fields ending `dop` denote dilutions of precision. These are dimensionless
/// factors that should be multiplied by a base UERE to get an error estimate.
///
/// Many devices compute dilution of precision factors but do not include them
/// in their reports. Many that do report DOPs report only HDOP, two-dimensional
/// circular error. gpsd always passes through whatever the device actually
/// reports, then attempts to fill in other DOPs by calculating the appropriate
/// determinants in a covariance matrix based on the satellite view. DOPs may be
/// missing if some of these determinants are singular. It can even happen that
/// the device reports an error estimate in meters when the corresponding DOP is
/// unavailable; some devices use more sophisticated error modeling than the
/// covariance calculation.
pub struct SkyResponse {
    /// Name of originating device.
    pub device: Option<String>,
    /// Timestamp.
    pub time: Option<DateTime<Utc>>,
    /// Longitudinal d.o.p.
    pub xdop: Option<f32>,
    /// Latitutinal d.o.p.
    pub ydop: Option<f32>,
    /// Altitude d.o.p.
    pub vdop: Option<f32>,
    /// Time d.o.p.
    pub tdop: Option<f32>,
    /// Horizontal d.o.p.
    pub hdop: Option<f32>,
    /// Spherical d.o.p.
    pub pdop: Option<f32>,
    /// Hyperspherical d.o.p.
    pub gdop: Option<f32>,
    /// Satellites in skyview.
    pub satellites: Vec<SatelliteObject>
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// Information about a device known to gpsd.
///
/// The API splits the DEVICE object into three variants:
/// - `ActiveSeenPackets`: the device is active, and we've seen packets from it.
/// - `Active`: the device is active, but we haven't seen any packets.
/// - `Inactive`: the device is inactive.
///
/// Basically, the aim here is to reduce the amount of Option unwrapping
/// you have to do, as gpsd specifies that all these fields are optional.
pub enum DeviceObject {
    ActiveSeenPackets {
        /// Name the device for which the control bits are being reported, or
        /// for which they are to be applied. This attribute may be omitted only
        /// when there is exactly one subscribed channel
        path: Option<String>,
        /// Time the device was activated as an ISO8601 timestamp. If the device
        /// is inactive this attribute is absent.
        activated: DateTime<Utc>,
        /// Bit vector of property flags. Currently defined flags are: describe
        /// packet types seen so far (GPS, RTCM2, RTCM3, AIS). Won't be reported
        /// if empty, e.g. before gpsd has seen identifiable packets from the
        /// device.
        ///
        /// # Flags
        ///
        /// - 0x01: GPS data seen
        /// - 0x02: RTCM2 data seen
        /// - 0x04: RTCM3 data seen
        /// - 0x08: AIS data seen
        ///
        /// Yes, I know manual bitflags suck. I'll fix it one day if you bug me.
        flags: u8,
        /// GPSD's name for the device driver type. Won't be reported before
        /// gpsd has seen identifiable packets from the device.
        driver: String,
        /// Whatever version information the device returned.
        subtype: Option<String>,
        /// Device speed in bits per second.
        bps: Option<u32>,
        /// N, O or E for no parity, odd, or even.
        parity: Option<String>,
        /// Stop bits (1 or 2).
        stopbits: Option<String>,
        /// 0 means NMEA mode and 1 means alternate mode (binary if it has one,
        /// for SiRF and Evermore chipsets in particular). Attempting to set
        /// this mode on a non-GPS device will yield an error.
        native: Option<u8>,
        /// Device cycle time in seconds.
        cycle: Option<f32>,
        /// Device minimum cycle time in seconds. Reported from ?DEVICE when
        /// (and only when) the rate is switchable. It is read-only and not
        /// settable.
        minicycle: Option<f32>
    },
    Active {
        path: Option<String>,
        activated: DateTime<Utc>,
        subtype: Option<String>,
        bps: Option<u32>,
        parity: Option<String>,
        stopbits: Option<String>,
        native: Option<u8>,
        cycle: Option<f32>,
        minicycle: Option<f32>
    },
    Inactive {
        path: Option<String>
    }
}
#[derive(Serialize, Deserialize, Debug)]
/// Information about watcher mode parameters.
pub struct WatchObject {
    #[serde(default = "serde_true")]
    /// Enable (true) or disable (false) watcher mode. Default is true.
    pub enable: bool,
    #[serde(default = "serde_false")]
    /// Enable (true) or disable (false) dumping of JSON reports. Default is
    /// false.
    pub json: bool,
    #[serde(default = "serde_false")]
    /// Enable (true) or disable (false) dumping of binary packets as
    /// pseudo-NMEA. Default is false.
    pub nmea: bool,
    /// Controls 'raw' mode. When this attribute is set to 1 for a channel, gpsd
    /// reports the unprocessed NMEA or AIVDM data stream from whatever device
    /// is attached. Binary GPS packets are hex-dumped. RTCM2 and RTCM3 packets
    /// are not dumped in raw mode. When this attribute is set to 2 for a
    /// channel that processes binary data, gpsd reports the received data
    /// verbatim without hex-dumping.
    pub raw: Option<u32>,
    #[serde(default = "serde_false")]
    /// If true, apply scaling divisors to output before dumping; default is
    /// false.
    pub scaled: bool,
    #[serde(default = "serde_false")]
    /// If true, aggregate AIS type24 sentence parts. If false, report each part
    /// as a separate JSON object, leaving the client to match MMSIs and
    /// aggregate. Default is false. Applies only to AIS reports.
    pub split24: bool,
    #[serde(default = "serde_false")]
    /// If true, emit the TOFF JSON message on each cycle and a PPS JSON message
    /// when the device issues 1PPS. Default is false.
    pub pps: bool,
    /// If present, enable watching only of the specified device rather than all
    /// devices. Useful with raw and NMEA modes in which device responses aren't
    /// tagged. Has no effect when used with enable:false.
    pub device: Option<String>,
    /// URL of the remote daemon reporting the watch set. If empty, this is a
    /// WATCH response from the local daemon.
    pub remote: Option<String>
}
impl Default for WatchObject {
    fn default() -> Self {
        Self {
            enable: true,
            json: false,
            nmea: false,
            raw: None,
            scaled: false,
            split24: false,
            pps: false,
            device: None,
            remote: None
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "class")]
/// A response from GPSD.
///
/// For single-struct variants, the documentation on the struct usually has
/// more information.
pub enum Response {
    #[serde(rename = "TPV")]
    Tpv(TpvResponse),
    #[serde(rename = "SKY")]
    Sky(SkyResponse),
    #[serde(rename = "POLL")]
    /// Data from the last-seen fixes on all active GPS devices.
    Poll {
        time: DateTime<Utc>,
        /// Count of active devices.
        active: u32,
        tpv: Vec<TpvResponse>,
        sky: Vec<SkyResponse>
    },
    #[serde(rename = "DEVICE")]
    Device(DeviceObject),
    #[serde(rename = "DEVICES")]
    Devices {
        devices: Vec<DeviceObject>,
        remote: Option<String>
    },
    #[serde(rename = "WATCH")]
    Watch(WatchObject),
    #[serde(rename = "VERSION")]
    Version {
        release: String,
        rev: String,
        proto_major: u32,
        proto_minor: u32,
        remote: Option<String>
    },
    #[serde(rename = "ERROR")]
    Error {
        message: String
    }
}
