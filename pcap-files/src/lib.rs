pub(crate) mod constants;
pub(crate) mod pcap;
pub(crate) mod pcapng;

pub use pcap::Pcap;
pub use pcap::PcapPacketRecord;
pub use pcap::PcapMagicNumber;
pub use pcapng::PcapNG;
pub use constants::{ETHERNET_HEADER_LENGTH_BYTES, IP_HEADER_LENGTH_BYTES, UDP_HEADER_LENGTH_BYTES};

use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum PcapError {
    #[error("Error parsing .pcap file")]
    ParsePcapError,
    #[error("Error parsing .pcapng file")]
    ParsePcapNgError,
}
