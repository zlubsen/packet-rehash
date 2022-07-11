use std::fs::File;
use std::io::{BufReader, Read};
use nom::IResult;
use crate::PcapError;

pub struct PcapNG {
    pub placeholder: usize,
}

impl TryFrom<File> for PcapNG {
    type Error = PcapError;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let mut buf = Vec::<u8>::new();
        let mut reader = BufReader::new(file);
        let _read = reader.read_to_end(&mut buf);

        match parse_pcapng_file(&buf) {
            Ok((_input, pcap)) => { Ok(pcap) }
            Err(_err) => {
                Err(PcapError::ParsePcapNgError) }
        }
    }
}

impl TryFrom<&[u8]> for PcapNG {
    type Error = PcapError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        match parse_pcapng_file(buf) {
            Ok((_input, pcap)) => { Ok(pcap) }
            Err(_err) => {
                Err(PcapError::ParsePcapNgError) }
        }
    }
}

fn parse_pcapng_file(_input: &[u8]) -> IResult<&[u8], PcapNG> {
    unimplemented!(".pcapng files not yet supported");
}