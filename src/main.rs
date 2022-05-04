//
//    0                   1                   2                   3
//    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |Version|  IHL  |Type of Service|          Total Length         |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |         Identification        |Flags|      Fragment Offset    |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |  Time to Live |    Protocol   |         Header Checksum       |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |                       Source Address                          |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |                    Destination Address                        |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |                    Options                    |    Padding    |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
//   Version: 4 bits
//   IHL:  4 bits, minimum value is 5
//   Type of Service: 8 bits
//   Total Length: 16 bits
//   Identification: 16 bits
//   Flags:  3 bits
//   Fragment Offset:  13 bits
//   Time to Live:  8 bits
//   Protocol:  8 bits
//   Header Checksum:  16 bits
//   Source Address:  32 bits
//   Destination Address:  32 bits
//   Options:  variable
//   Padding:  variable

use std::net::Ipv4Addr;

// wanna xhibit this and nest the bitflags struct in the Ipv4Header struct but SO says no

struct Ipv4Flags {
    reserved: 0,
    dont_fragment: u8,
    more_fragments: u8
}

// setting u8 for 4bits or 3bits seems wasteful would like to shrink that if possible.
struct Ipv4Header {
    version: u8,
    internet_header_length: u8,
    differentiated_services_code_point: u8,
    explicit_congestion_notification: u8,
    // total_length: u16,
    identification: u16,
    flags: BitlFags,
    // fragment_offset: u16,
    time_to_live: u8,
    protocol: u8,
    header_checksum: u16,
    source_address: Ipv4Addr,
    destination_addres: Ipv4Addr,
}

impl Ipv4Header {
    fn new(src: Ipv4Addr, dst: Ipv4Addr) -> Self {
        Ipv4Header {
            version: 4,
            internet_header_length: 0,
            differentiated_services_code_point: 0,
            explicit_congestion_notification: 0,
            identification: 0,
            flags: ,
            time_to_live: 0,
            protocol: 4,
            header_checksum: 'something',
            source_address: src,
            destination_addres: dst
        }
    }
}


fn main() {
    println!("Do the PING PONG");
}
