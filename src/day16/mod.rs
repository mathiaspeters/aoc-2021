pub fn day16() {
    println!("Result 16-1: {}", part1());
    println!("Result 16-2: {}", part2());
}

pub fn part1() -> usize {
    let packet = Packet::new(raw_input());
    packet.version_sum()
}

pub fn part2() -> usize {
    let packet = Packet::new(raw_input());
    packet.value()
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: usize,
        type_id: usize,
        value: usize,
    },
    Operator {
        version: usize,
        type_id: usize,
        packets: Vec<Packet>,
    },
}

impl Packet {
    pub fn new(hex: &str) -> Self {
        let bits = to_binary(hex);
        Self::parse(&bits).0
    }

    fn parse(bits: &[u8]) -> (Self, Vec<u8>) {
        let version = to_int(&bits[0..3]);
        let type_id = to_int(&bits[3..6]);
        let output = if type_id == 4 {
            let mut bits_used = 6;
            let mut significant_bits = vec![];
            for chunk in bits[6..].chunks(5) {
                bits_used += 5;
                for i in 1..=4 {
                    significant_bits.push(chunk[i]);
                }
                if chunk[0] == 0 {
                    break;
                }
            }
            (
                Self::Literal {
                    version,
                    type_id,
                    value: to_int(&significant_bits),
                },
                bits[bits_used..].to_vec(),
            )
        } else {
            let length_type_id = bits[6];
            let (packets, remaining_bits) = if length_type_id == 0 {
                let sub_packet_length = to_int(&bits[7..22]);
                let mut remaining_bits = bits[22..22 + sub_packet_length].to_vec();
                let mut packets = vec![];
                while !remaining_bits.is_empty() {
                    let (packet, rem) = Self::parse(&remaining_bits);
                    packets.push(packet);
                    remaining_bits = rem;
                }
                (packets, bits[22 + sub_packet_length..].to_vec())
            } else {
                let sub_packet_count = to_int(&bits[7..18]);
                let mut remaining_bits = bits[18..].to_vec();
                let mut packets = Vec::with_capacity(sub_packet_count);
                for _ in 0..sub_packet_count {
                    let (packet, rem) = Self::parse(&remaining_bits);
                    packets.push(packet);
                    remaining_bits = rem;
                }
                (packets, remaining_bits)
            };
            (
                Self::Operator {
                    version,
                    type_id,
                    packets,
                },
                remaining_bits,
            )
        };
        output
    }

    pub fn version_sum(&self) -> usize {
        match self {
            Self::Literal { version, .. } => *version,
            Self::Operator {
                version, packets, ..
            } => *version + packets.iter().map(|p| p.version_sum()).sum::<usize>(),
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Self::Literal { value, .. } => *value,
            Self::Operator {
                type_id, packets, ..
            } => match type_id {
                0 => packets.iter().map(|p| p.value()).sum(),
                1 => packets.iter().map(|p| p.value()).product(),
                2 => packets.iter().map(|p| p.value()).min().unwrap(),
                3 => packets.iter().map(|p| p.value()).max().unwrap(),
                5 => {
                    if packets[0].value() > packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].value() < packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].value() == packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!(),
            },
        }
    }
}

fn to_int(bits: &[u8]) -> usize {
    let mut multiplier = 1;
    let mut result = 0;
    bits.iter().rev().for_each(|b| {
        result += *b as usize * multiplier;
        multiplier *= 2;
    });
    result
}

fn to_binary(s: &str) -> Vec<u8> {
    s.chars()
        .flat_map(|c| match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => panic!(),
        })
        .collect()
}

#[cfg(not(test))]
fn raw_input() -> &'static str {
    include_str!("input")
}

#[cfg(test)]
fn raw_input() -> &'static str {
    include_str!("testinput")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(6, Packet::new("D2FE28").version_sum());
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(16, Packet::new("8A004A801A8002F478").version_sum());
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(12, Packet::new("620080001611562C8802118E34").version_sum());
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(
            23,
            Packet::new("C0015000016115A2E0802F182340").version_sum()
        );
    }

    #[test]
    fn test_part1_5() {
        assert_eq!(
            31,
            Packet::new("A0016C880162017C3686B18A3D4780").version_sum()
        );
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(3, Packet::new("C200B40A82").value());
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(54, Packet::new("04005AC33890").value());
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(7, Packet::new("880086C3E88112").value());
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(9, Packet::new("CE00C43D881120").value());
    }

    #[test]
    fn test_part2_5() {
        assert_eq!(1, Packet::new("D8005AC2A8F0").value());
    }

    #[test]
    fn test_part2_6() {
        assert_eq!(0, Packet::new("F600BC2D8F").value());
    }

    #[test]
    fn test_part2_7() {
        assert_eq!(0, Packet::new("9C005AC2F8F0").value());
    }

    #[test]
    fn test_part2_8() {
        assert_eq!(1, Packet::new("9C0141080250320F1802104A08").value());
    }
}
