pub(crate) mod pcap;
pub(crate) mod pcapng;

pub use pcap::Pcap;
pub use pcap::PcapPacketRecord;
pub use pcap::PcapMagicNumber;
// pub use pcap::PcapFileHeader;
pub use pcapng::PcapNG;

use thiserror::Error;

#[derive(Clone, Debug)]

pub enum Error {
    #[error("Error parsing .pcap file")]
    ParsePcapError,
    #[error("Error parsing .pcapng file")]
    ParsePcapNgError,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
