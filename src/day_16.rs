use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

pub(crate) struct Packet {
    version: usize,
}

pub(crate) fn parse_packets(input: &String) -> Vec<Packet>{
    let packets = Vec::new();
    let bytes = hex::decode(input).unwrap();
    println!("{:?}", bytes);

    packets
}

#[cfg(test)]
#[test]
fn test_parse_shortest() {
    let input = read_lines("data/day_16_shortest_sample.txt");
    let packets = parse_packets(&input[0]);

    let sum_versions: usize = packets.iter().map(|p| p.version).sum();
    assert_eq!(sum_versions, 16);
}