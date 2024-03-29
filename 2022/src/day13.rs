use crate::println;
use crate::util::Day;
use crate::util::FnIterator;
use heapless::pool;
use heapless::pool::singleton::Box;
use heapless::pool::singleton::Pool;
use heapless::Vec;

pub const DAY_13: Day<i32> = Day {
    year: 2022,
    day: 13,
    parts: &[do_part_1, do_part_2],
    tests: &[
        test_read_integer,
        test_read_list_of_integers,
        test_read_nested_stuff,
        test_verify_pairs,
        test_parse_weird_stuff,
        test_find_sum_of_verified_indices,
        test_verify_equal_length_lists,
        test_find_indices_of_divider_packets,
        // test_sort,
    ],
};

type Elements = heapless::Vec<Packet, 16>;
pool!(A: Elements);
static mut MEMORY: [u8; 32_768] = [0; 32_768];

#[derive(Debug)]
pub enum Packet {
    Integer(i8),
    List(Box<A>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a == b,
            (Packet::List(a), Packet::List(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                for k in 0..a.len() {
                    if a[k] != b[k] {
                        return false;
                    }
                }
                return true;
            }
            _ => false,
        };
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        return match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                for k in 0..a.len() {
                    let element_left = &a[k];
                    let maybe_element_right = b.get(k);
                    if maybe_element_right.is_none() {
                        return core::cmp::Ordering::Greater;
                    }
                    let res = element_left.cmp(maybe_element_right.unwrap());
                    if res != core::cmp::Ordering::Equal {
                        return res;
                    }
                }

                // we iterated through both A and B but could not find a conclusive
                // answer. if they are both of the same length we must continue checking.
                // otherwise, we know that A is smaller so we can return OK.
                if a.len() == b.len() {
                    return core::cmp::Ordering::Equal;
                }

                return core::cmp::Ordering::Less;
            }
            (Packet::Integer(a), Packet::List(_)) => {
                let mut elems = Elements::new();
                let element_a = Packet::Integer(*a);
                elems.push(element_a).unwrap();
                let left_as_list = Packet::List(A::alloc().unwrap().init(elems));
                return left_as_list.cmp(other);
            }
            (Packet::List(_), Packet::Integer(b)) => {
                let mut elems = Elements::new();
                let element_b = Packet::Integer(*b);
                elems.push(element_b).unwrap();
                let right_as_list = Packet::List(A::alloc().unwrap().init(elems));
                return self.cmp(&right_as_list);
            }
        };
    }
}

fn do_part_1() -> i32 {
    unsafe {
        A::grow(&mut MEMORY);
    }
    let sum = find_indices_of_verified_pairs::<256>(INPUT).iter().sum();

    return sum;
}

fn do_part_2() -> i32 {
    unsafe {
        A::grow(&mut MEMORY);
    }
    let (a, b) = find_indices_of_divider_packets::<512>(INPUT);

    return a * b;
}

struct PacketIterator<'a> {
    lines: core::str::Lines<'a>,
}

impl<'a> PacketIterator<'a> {
    fn new(data: &'a str) -> Self {
        return Self {
            lines: data.trim().lines(),
        };
    }
}

impl<'a> Iterator for PacketIterator<'a> {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        let mut maybe_line = self.lines.next();
        while maybe_line.is_some() && maybe_line.unwrap().trim() == "" {
            maybe_line = self.lines.next();
        }
        return match maybe_line {
            Some(line) => Some(read_value(line)),
            None => None,
        };
    }
}

fn find_indices_of_verified_pairs<const N: usize>(data: &str) -> Vec<i32, N> {
    let mut k: i32 = 1;
    let mut iter = PacketIterator::new(data);
    let mut out: Vec<i32, N> = Vec::new();

    while let Some(left) = iter.next() {
        let maybe_right = iter.next();

        if maybe_right == None {
            panic!("Could not find right packet!");
        }

        let right = maybe_right.unwrap();

        if left.cmp(&right) == core::cmp::Ordering::Less {
            out.push(k).unwrap();
        }

        k += 1;
    }

    return out;
}

const DIVIDER_PACKET_1: &'static str = "[[2]]";
const DIVIDER_PACKET_2: &'static str = "[[6]]";

fn find_indices_of_divider_packets<const N: usize>(data: &str) -> (i32, i32) {
    let mut lines: Vec<&str, N> = data.trim().lines().filter(|line| *line != "").collect();
    lines.push(DIVIDER_PACKET_1).unwrap();
    lines.push(DIVIDER_PACKET_2).unwrap();

    lines.sort_unstable_by(|a, b| {
        // we don't have enough ram to keep the parsed values in memory.
        let packet_a = read_value(a);
        let packet_b = read_value(b);

        return packet_a.cmp(&packet_b);
    });

    let mut index_a: Option<i32> = None;
    let mut index_b: Option<i32> = None;

    for k in 0..lines.len() {
        let line = lines[k];

        if line == DIVIDER_PACKET_1 {
            index_a = Some((k + 1) as i32);
        } else if line == DIVIDER_PACKET_2 {
            index_b = Some((k + 1) as i32);
        }
        if index_a.is_some() && index_b.is_some() {
            break;
        }
    }

    return (index_a.unwrap(), index_b.unwrap());
}

fn read_list(data: &str, start_index: usize) -> (Elements, usize) {
    let bytes = data.as_bytes();
    let start_index = skip_whitespace(data, start_index);

    assert_eq!(b'[', bytes[start_index]);

    if let Some(end_index) = find_closing_delimeter(data, start_index) {
        let mut elements = Elements::new();
        let mut k: usize = start_index + 1;

        while k < end_index {
            let (el, next_index) = read_value_from_index(data, k);
            elements.push(el).unwrap();
            k = next_index;

            if bytes[k] == b',' {
                k += 1;
            }
        }

        return (elements, end_index + 1);
    }

    panic!("Unclosed list at index = {}", start_index);
}

fn skip_whitespace(data: &str, start_index: usize) -> usize {
    let bytes = data.as_bytes();

    for k in start_index..bytes.len() {
        if !bytes[k].is_ascii_whitespace() {
            return k;
        }
    }

    panic!("Reached end of string...");
}

fn read_integer(data: &str, start_index: usize) -> (i8, usize) {
    let bytes = data.as_bytes();
    let start_index = skip_whitespace(data, start_index);

    assert_eq!(true, bytes[start_index].is_ascii_digit());

    let mut k: usize = start_index;

    while k < bytes.len() {
        if !bytes[k].is_ascii_digit() {
            break;
        }
        k += 1;
    }

    let found = data[start_index..k].parse().unwrap();
    return (found, k);
}

fn read_value_from_index(data: &str, start_index: usize) -> (Packet, usize) {
    let bytes = data.as_bytes();
    let start_index = skip_whitespace(data, start_index);

    if bytes[start_index] == b'[' {
        let (elements, next) = read_list(data, start_index);
        let boxed = A::alloc().unwrap().init(elements);
        return (Packet::List(boxed), next);
    }

    let (int, next) = read_integer(data, start_index);
    return (Packet::Integer(int), next);
}

fn read_value(data: &str) -> Packet {
    let (found, next_index) = read_value_from_index(data, 0);

    assert_eq!(data.len(), next_index);

    return found;
}

fn find_closing_delimeter(data: &str, start_index: usize) -> Option<usize> {
    let bytes = data.as_bytes();
    assert_eq!(b'[', bytes[start_index]);
    let mut stack = 1;

    for k in (start_index + 1)..bytes.len() {
        stack += match bytes[k] {
            b'[' => 1,
            b']' => -1,
            _ => 0,
        };
        if stack == 0 {
            return Some(k);
        }
    }

    return None;
}

fn packet_to_string<const N: usize>(value: &Packet, buffer: &mut heapless::String<N>) {
    if let Packet::Integer(x) = value {
        buffer.push_str(&heapless::String::<16>::from(*x)).unwrap();
    }
    if let Packet::List(elements) = value {
        buffer.push('[').unwrap();
        for i in 0..elements.len() {
            let el = &elements[i];
            packet_to_string(el, buffer);

            let should_write_comma = i < (elements.len() - 1);
            if should_write_comma {
                buffer.push(',').unwrap();
            }
        }
        buffer.push(']').unwrap();
    }
}

fn test_read_integer() {
    let list = "[42, 69]";
    let (a, n) = read_integer(list, 1);

    assert_eq!(42, a);
    assert_eq!(3, n);

    let (b, n) = read_integer(list, 5);

    assert_eq!(69, b);
    assert_eq!(7, n);
}

fn test_read_list_of_integers() {
    unsafe {
        A::grow(&mut MEMORY);
    }

    let list = "[42, 69]";
    let (elements, _) = read_list(list, 0);

    assert_eq!(2, elements.len());
    assert_eq!(Packet::Integer(42), elements[0]);
    assert_eq!(Packet::Integer(69), elements[1]);
}

fn test_read_nested_stuff() {
    unsafe {
        A::grow(&mut MEMORY);
    }

    let list = "[42, [69, 7]]";
    let mut elems = Elements::new();
    let mut inner = Elements::new();

    inner.push(Packet::Integer(69)).unwrap();
    inner.push(Packet::Integer(7)).unwrap();

    elems.push(Packet::Integer(42)).unwrap();
    elems
        .push(Packet::List(A::alloc().unwrap().init(inner)))
        .unwrap();
    let expected = Packet::List(A::alloc().unwrap().init(elems));

    let value = read_value(list);

    assert_eq!(expected, value);
}

fn test_verify_pairs() {
    unsafe {
        A::grow(&mut MEMORY);
    }

    let packets: Vec<Packet, 32> = PacketIterator::new(TEST_INPUT).collect();
    let results: Vec<core::cmp::Ordering, 32> = packets
        .chunks(2)
        .map(|chunk| {
            let a = &chunk[0];
            let b = &chunk[1];

            return a.cmp(&b);
        })
        .collect();

    assert_eq!(
        [
            core::cmp::Ordering::Less,
            core::cmp::Ordering::Less,
            core::cmp::Ordering::Greater,
            core::cmp::Ordering::Less,
            core::cmp::Ordering::Greater,
            core::cmp::Ordering::Less,
            core::cmp::Ordering::Greater,
            core::cmp::Ordering::Greater,
        ],
        results
    );
}

fn test_parse_weird_stuff() {
    unsafe {
        A::grow(&mut MEMORY);
    }

    let a = read_value("[[[[7,9]],[[0,9]],0,[[2,1,1,2,9],4],[[5]]],[]]");
    let b = read_value("[[8]]");

    if let Packet::List(_) = a {
    } else {
        assert_eq!(1, 2);
    }

    if let Packet::List(_) = b {
    } else {
        assert_eq!(1, 2);
    }
}

fn test_find_sum_of_verified_indices() {
    unsafe {
        A::grow(&mut MEMORY);
    }

    let ok_indices = find_indices_of_verified_pairs::<16>(TEST_INPUT);

    assert_eq!([1, 2, 4, 6], ok_indices);
}

fn test_verify_equal_length_lists() {
    unsafe {
        A::grow(&mut MEMORY);
    }
    let data = "[1, 2, 3]";
    let a = read_value(data);
    let b = read_value(data);
    let ok = a.cmp(&b);

    assert_eq!(core::cmp::Ordering::Equal, ok);
}

/*
fn test_sort() {
    unsafe {
        A::grow(&mut MEMORY);
    }
    let pairs = PairIterator {
        lines: TEST_INPUT.trim().lines(),
    };
    let mut packets: Vec<Value, 32> = Vec::new();
    for (a, b) in pairs {
        packets.push(a).unwrap();
        packets.push(b).unwrap();
    }

    let divider_packet_a = read_value("[[2]]");
    let divider_packet_b = read_value("[[6]]");

    packets.push(divider_packet_a).unwrap();
    packets.push(divider_packet_b).unwrap();
    packets.sort_unstable();

    let mut res = heapless::String::<512>::new();

    for p in packets {
        let mut s = heapless::String::<64>::new();
        packet_to_string(&p, &mut s);
        res.push_str(&s).unwrap();
        res.push('\n').unwrap();
    }

    let expected = r#"
[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[[2]]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[[6]]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]
    "#;

    assert_eq!(expected.trim(), res.trim());
}
*/

fn test_find_indices_of_divider_packets() {
    unsafe {
        A::grow(&mut MEMORY);
    }
    let (a, b) = find_indices_of_divider_packets::<32>(TEST_INPUT);

    assert_eq!(10, a);
    assert_eq!(14, b);
}

const TEST_INPUT: &'static str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

const INPUT: &'static str = r#"
[[[[8,6]],[8,[7],[6,7,6,2,4]],10,[[1,7,9],7,[7,9]]],[[4,[],[10,5],[5,4,7],5],8,9,[[5,3,3,6,9],[9,5,10],8],[[0,6,9],[8],4,6,8]]]
[[[],3,[[10,6,9,6],[6,8,7],[1,2]],8]]

[[4,[4],[[10,7,2],[1,6,5,7,4],[7,3,3,1,5],[]],1],[[],[[7,6,3]],5,5]]
[[[10]]]

[[[[7,9]],[[0,9]],0,[[2,1,1,2,9],4],[[5]]],[]]
[[8]]

[[9,[[5,4,5,1,10]]],[[[6,9],3,[0,2]],5,6,[3,4,[2,2],10],[10,5,9]],[[[10,3,8],[],[6,9]],4]]
[[[4,[7],[1,1,4,3,8],[3],[]],[[5,10,2,1],9,[2],6,7],4,[],[[10],[6,3,1,7]]],[4,[6],[[6],[6,6]]],[[[0,2,5],[3,5,10,7],[10,7,2,9],[],[0]],[[9,9,8],[]],[10,[2,10,7,7,3],4],[2,7]]]

[[[],2,2,9,[[9],[1,10,5,1]]],[[[],9,5,[8]],10,9,[[1,4,0],[5]]]]
[[5],[3],[],[9,[[7,2,0,9,3],[6,8,6],[]]],[[[6],3],6,[[2,8],8]]]

[[]]
[[[5,4],5]]

[[7,7,5]]
[[[[4,7,3],[6,0],1,6],[6,1,4,5]],[]]

[[[[4,7,0,9,8],10,7,2]],[6,6,9],[[2,10],8,9,1,[1,[0,4]]],[[[2,0,2],[]]]]
[[[[]],[],3],[],[[[],[8]],[[4,7,3,7,10],3,5]],[[9,[10,5,8,9,1]],0],[[[6,7,5,4,10],4,6,0,[8,3,1,2,7]],5,[10,1,5],10,0]]

[[],[[3,8,2]]]
[[8,4],[5],[3]]

[[0,1,5,[[],[6,6,8,0],[0,2,5,9,9],[3,2,7],[5,2]],10],[],[[8,8],[1,3],3,2],[[[4,0],[]],[[4,1,2],[2],6,9,6],1,10,[[7],[0,7],5]],[4]]
[[[],[6,[5,9,6,0,8],2,9,[4,2]],7,[9,6,0,10]]]

[[],[],[],[9,[[1],[6,3,4,9]],10,8]]
[[1,[[2],10,[0,9,3]]]]

[[0,1,4,4],[[7,[6],[2,5,1,3],5],[[8,3,7],[6],[],8],8,1,[9,0,[]]],[[0,[3,9,3,5],[2,0,9,0,7],6]],[[[3,3,1,10,9]],[[0,7,1],8,[10,0],8],[[9,3,5,4],5,[0],5],8,[[3,6,10],0,2,6]]]
[[[8,[1,2,10,9],2,[8,4,4]],[3,[5,5]],4],[[],2,[],6,3],[[10,7],[],7,[8,6,[5],4,3],2]]

[[[[],1,2],[[7,7,7],6,[1,7,1]]],[4,8],[]]
[[3,1,[1,6,[9,9,5],5],[[1,7,0,6],[10,8,10],[3,8],[6,2,7,5]]]]

[[1]]
[[[3],[[9,4,10,10],[8,9,1],[],[0,3,10,5]],[8,[]]],[[[10,6,0],2,1],5,0,[7],[[9]]],[[[],[9,9,3,2,3],5]]]

[[],[7,2],[5,[9,[]],1,[4,6,5],[[9,10,8,3],6]]]
[[[[]],0,[1,8],[5,2,7,2],6],[],[[],[[2,1,3,9],6,9,5,3],2],[[[10],[0],[4,0,3,6,8]],4],[9]]

[[],[],[],[]]
[[],[[3,6],6,[6],[[3,10,4],6],[]]]

[[8,7,10],[3],[9,[],[[5,1,6,10,4]],4,[[9,8,1,1],[0,0,2],1]]]
[[[1,6,[4,5,3,1]]],[[],[[7,0,5,1,9],6,[0,9,4,3]]]]

[[2,2,1,8],[],[[[10,4,10,6],[2,8,3,10,6],[]],1,6],[6,0]]
[[2,3,[8,[2,7]],8],[4,[[6,6,4,2],4,5,[7,5]],8,1,[]],[7]]

[[[],[1,[9,1,9],[8,8,8,5]],3,[4,9,4,[2,6,6]],[]],[[7,[1,9,6]],6,[[10,0,0],[4,7,1],4,9,7]]]
[[[[],8,5,1],3,[[8,2,2],4,[2,1,2,3,7]],10,[2,[5,7,6],3,[9,6,5,0,9],4]],[],[9,3,2,7],[4,[[4,10],[8,3,8]],6,[],9]]

[[[],[[],[5]],4],[[0,[],9,[8,5,3,2,2]],[8]],[[[2,5]],[10,8,[7,8,7,3]],5,[[2,10],3,8,0,[6,9,10]],5],[[0,[7],[]],[[3,9,4,1,6],8],[[4,4,7,2],8],[[3],[],3],7]]
[[[2,0,[7,5]],[1,5,7,6,[]],9,[2,1,[6,0],3,[3]]]]

[[1,3,10,[]],[],[7,[8,4],[[],0,[4,8],6],4,[[4,0],[5,0,8,3],[0,1],8,[2,9,7,6,5]]],[3,5],[9,[[8,3,6,0],[10,0,5],10],[[10,1,4,0],4,8,5],9,[7,[0],[7,9],7,2]]]
[[2,9,0,[[5,8,10,10,5],8,4,[5,6,10]],10],[8,[0,[10],[6],4,[]],[[9,4,0],[],[2,10,3,3,4],[3]],1,10],[[[],0,7,3,3],9,[[5,3,0],5,[4,6,6,7]],[]],[6,[[3,1],10,[0,8],0],8,[4,[],[8,0,7,9]],7],[9,4,[10],[[2,10,7,6]],1]]

[[[],8,9,2,[6,[0,2,10,2],[4,9,7,2,10]]],[9,[[3,4]],5,9]]
[[9],[[],[1,6,[5,9,9,9,0]],6]]

[[5,2,10],[2,[[],7,9,3],0,[7,[]]],[[9,[5,6,5,0],4,0,3],10,1,[]],[8,10,3,9]]
[[[2,[3]],9]]

[[],[6,[3,[],[]],8,[[7,7,8,2],4],0]]
[[[[0,5,7]],[[]],7,[4,[9,0,5],7,3]],[5,[[0,8,10,0],3],[1,4],[10],[]]]

[[1,5],[10,[[9,7,1]],0,[9,[6,3],[7],9,[]]]]
[[5,[2,[4,0,4,7,8],[4]]],[8,[2,9],5],[],[4,2]]

[[7,[[0,0,5,4]],4],[[[2,4]],[1,[3,2,8],[9,1,8,2,2],0,1],2],[[[4,7],3,3,9]]]
[[],[[[8],0,10,[0,4]],2,[9,2]],[10,4],[2,[7],10]]

[[[],10],[[6,7,[8,6,6],2],5,[9,[5,7,9,0],[10,6],7]],[[],[5],10,[[6],6,7,6,[6]]],[[]]]
[[[[],[]],[]],[[[6],[9],[4,0,8,5]],[3],[[8,2,7,7]],[],[3,[5,1],[9],[7,1]]],[[7,[0],[],3],[2]],[[]]]

[7,10,4,6]
[7,10,4,6,9]

[[9]]
[[7,2,6],[[6,9,[]],[5,3,6],[2],[[0,7,1],[0],[2],[9,9,0,8,2],[9,5,7,6,8]]],[[8,6,10,0,8],[9,[3,10],4,[2,4,3,4,4],5]],[],[]]

[[[3,8,[]],1,7],[[4],[10]],[[[2],8,8],[[1,0,6,5],4],6,[4]],[[[],[9,8,7,10,6],2,[1]],[[],9,3,[1,6,5],[]],6,4,9]]
[[8,[2,[3,7,1,0]]],[8,4,[[9,4],[0,3,5,5]]],[8,[[2,10,4],7,5,[9,0,5,0,4]],3,9,[8,8,3,3,0]],[[9,4,[0],4,[6]],[[0,10,0],6,[9,4,10,0]]]]

[]
[[[[0,4,6,7],3],2],[5,[10,[8],[4,9,4],[],5],6,[[0,6],[4,7,7],[],10,[]]]]

[[[3],8,10,[[8],2]],[9,8,10],[[[],[],3,[]],[],[1,9]]]
[[2,0,0,9],[[],[[10,2],[]],[7,[1],4,[2,5,8,1,2],6]],[[8],[[9,3,3,8],2,[5,5,2,9],2],[5,[],6,[2,1,4,10,1],7]],[],[]]

[[[[1],[7,1,9]],[[],0,[0,7]],[6],9,[]],[]]
[[[[2,8,7,3],[4,2],10,[8,4],7]],[],[[7,[3,2]]],[[[8,7],8,10],0,2]]

[[[1,[6,7,0,8]],6],[2],[6,7],[],[[[1,9,2,10],[9],[0,4,3,1],[6,10,2,4],[4,0,10,6]],[6,5],6,[5,10,[1,0,10,1,2],[5,10,5],9],[[3]]]]
[[[[3,9,0,6,3],[]],1,7,4,[[9,8,5],1,6,[]]]]

[[],[[[]],9,[[2,6,2,9,5],9,[6,6]],[[1,0,7],[4,4]]]]
[[1,[[5]],1,0],[2,[[9,9,1,8],2,8,[0,5]],[]],[0,[5,8,4],[[6,2],[3,6,0,8],[4,2,5],[5,10,4,6,6],[5,5,6]],[7,2,10,[]]],[7,[]]]

[[5,[4],[0,[5,3,2],[]],0],[3,[10,2,7,[]]]]
[[],[4,9],[[[2],[0,10,6,1],1,[0],[9,4,8,1,9]],0,[[0],8,9,[1,1,10,10]],7],[[[3,10],10],10,2,[10,3]],[[5]]]

[[4,[],[[0,3,5]],[[]],1],[9,[0,4,7],[3,[5,10,9,2],6,2,6]],[]]
[[[1,4,[10,3,8],[7,7]],[1,[]],[[1,2,5]],9],[],[],[0,6,2,8]]

[[[[5,9,4,1]]],[0,1,[],7],[[[]],[9,[2,5,10],5,1],[[]],[[9],[6,4,0,4,9],1],[8]],[[10,1,7],6,10,7,6]]
[[[8,9,[0,9,2,5]],10]]

[[9,[[4,8,2,0],[0,1,7,7],5,[4,3,0]],6,[1,[3,9],[3,10,10,3,1],6]]]
[[],[5,[[4,0,9,5],1,[2,0,5,6],8,3],1,[[9,3,10,10],[],[10],7]]]

[[[[4,7,4,1],8],1],[[8,0,0,5,[0]],2,[]],[[[5,8,6,10,0],[],7,[],1],[7,[7,8,0,2,5],9,[8,5]],[8,[7,4]]],[10,[9,3,[0,5],[2]]],[[6,3,6],6,[[7,1,10],5,4,[3,5,1,7,10],9],[1,5,[2],[5],[3,7,0]]]]
[[],[[],10],[0,[],5],[2,[8,2,0],[3]]]

[[2,3,[3,6,8],5,2]]
[[],[0,0,[],8,2],[[[],[0]],[[],4,6,0,[8]],5,7,10],[[],[[1,6,3],[6],[6,7,0],4],[],[[1,10,4],2,0]],[[9,[1,0,7,1,7],3],3,9]]

[6,1,3,9,6]
[6,1,3,9]

[[3,10],[8,[[8,4]],0],[],[0]]
[[[[6,6,9,5]],[[4,0,0],[9,8,10,9]],6],[[1,[1,8,7,9],[]],4]]

[[[],4],[[[]],6,1],[[4,1,[8,8,9,3],6,9],1,8,[],3],[[[3,4,6,0]],9,0,0,[]],[[[5],[4,7,9,6]],2]]
[[9],[[[3,2,2,3],0],5,[[6,6,0,9,6],[10,3,3],3],[]]]

[[],[],[],[7]]
[[[4,[5,0,7],[5,5,4,4,2],9,[8,0,0]]]]

[[7,[[0,3]],4],[9]]
[[],[4,[[7,10,4,7,6],[3,8,2,1,6],8,8],2,5],[7]]

[[[4,0],0,7,[0]]]
[[1,[8,[4,2],6,2],0,[],8],[[]]]

[[],[0],[[[7,2,10]],3,[[8],2,[5,2,1,5],1],[8,[9,9],1]],[],[2,[[10,3,8],0,[8,6,0,5,1]],[2,7],[[3,6,7]],[2,[0,7,2,4]]]]
[[],[[],6,7],[3,5,[0]]]

[[10,7,[[9,2,5,10],[],3,[2,3]]],[[[1,0,10],0],[[6,6,2,9,6],[2,3],4,[2,4,5,2]],7,[5,7,8,[10,4,10,4],9],4],[]]
[[[9,9,3,2]]]

[[5,0,9,8,1]]
[[3,5,[[],5,0],7],[7,9,9,[[6,0,4,4],2,[10,3],5,6]],[]]

[[0],[[],[],[1,[7]],[]]]
[[],[[4],[8,[0,1,5,2],[0,9,3]],9],[1,[1,[],4,9],6,[],8],[[2],[[7,3,4,9],10,6,[5,7,10],8],5,1,5]]

[[],[9,[[],3,4],[],3,[1,[8,5,1,9,7],[1,9,3,2],[2],4]],[[1,4,1,[0]],[[6,6,3,6],[2,2,6,9,5],0]],[[3],[]]]
[[],[[[2,8,10,3]],10,[[5,3],4,8,[8]]],[1,1,[[]]],[],[]]

[[],[7,[6,[1,4],9],10,8,[[],6,1,3,8]]]
[[9,0,[[7,0]]],[],[2,9,3],[[[5,8,8,8],9],[6],0,[[1],9,8,[]],2],[[],[5,0,[],8],[[9,4,0,2],9,1,9],3]]

[[[5],[[3,1,7,10,3],8,[],7,[1,7]],5,0]]
[[[2,[3,4,9,8,10],[5,6],[9,10,8],[6]],[1,[10,7,3,7],[8],4],3,9,9],[[2,[5,1,10,8],[1,9,8,9,9],10],6,2,[]]]

[[9,[],[[1,0,4],[7,5],[0]],[2,2,[5,3,10,9],[8,4,6,0,0]],[1,[4,2]]]]
[[[[5,6,4,3,6],4,[3,5,10,7]],1,[8,[7,5,9,10,7],2,5,7],0],[[6,[0],[],[3,3,2]],[[5,2,4,2,3],10,2,4,8]]]

[[[],8],[6],[[[7]],6,[[9,7,5],[5,0,3,3],[2,2,0,8,3],[5,7,8,1,3],[1,6,4]]],[8,[6,4,[],0]]]
[[[[3,3,3],7,10,2,[0,1]],[[2,4,3,2,2],[7,2,4,4],10,[8,1,1]],4],[4,3,[[],5,[1],[0,7]]],[8,6,10,[]],[[5,[7,0,6]],3,10,10],[7,0,[10,4,3,7],3,2]]

[[[7,[0,2],[],3,[5,0,6,9,0]],0],[[[8],[6,8,4,2],[8,6,4,8],4,3]],[[3,2,4],[6,2],10,[[6],3,[2,6,0]],[[7],4,6,[]]],[]]
[[7,1,2,1,[[2,0,8,10,10],[5,6,1],[6],[5]]],[[6,[10,5,10,1,1],6],1],[[[8,1,2,5,8],0],[10,9,[2,4,4,8]],6,[2,[1],0],6]]

[[9,3,8,[1,[1,2,6,8,0],9,0]],[[6,[9],[6,2,1,5],[3,6,0,0]],[[6,0,1]]],[[[3,9,8,8,5],[1,8,3,10,5]]],[1,[[3,4],4,[9,7]]]]
[[0],[[[],7,[8],[7,8,9],3],[[],3,[9,7,7]],[8,[3],0],[1,4],[[8],[]]],[[[6,10,0,1],[4,6,1,5],[6,3,1,4,0]],2,4],[]]

[[[[2,0],[10,1,7,7,7],[8,4],[],[7,4,5]],4,[],1,4],[7],[4,7,9,[3],[[3,4],3,4,[5,6,4,7,10]]],[3,7,8,[2,7,[8,0],4]]]
[[8,7,9],[[[3,7,9,8,8],[5,6,5],1],9,[[0,5],[]],[5]],[[[8,2,7,9,0],1],1,[],[],[[4,2,0,10],[],0,[2]]],[]]

[[[],3,5,[[],[6,10],[7],5]],[[[9,7,0]],[0,4,3],9,[[],0,[5,5,1],10],[8,[]]],[[[3,3,10,3]],0],[[1,[3,3],[],0],[[],8,7,[5,3,6,2],[7,6,9,3,5]],6,2]]
[[[[1,4,6,8]],0,[3,0,[],8]],[1,8],[7,[]],[7,[6,7,4],10,[3,0,[7,3],7,7],1],[5,[8,[2,4],3],[3],1]]

[[[1,0,2],[5]]]
[[[9],[[1,3,9],[4,3,9,2,4],[7,0,1,10,1]],[0]],[3,[[2,7],4,[6,1,5,5,6],10],9,7,[[8,7,8,9],9,[],[0,2,3,4,10],0]],[4,[[6,6,1,9,0],2,0,[2,10,6]],6,0,3],[3,5,9,[]]]

[[0,0,0,[10]],[[[1,7,4,0,5]],7,2]]
[[[[9,10,9,10,7],1],[],1,[[]],4],[8],[3,[7,5,9,[0,0,5,10]],6,10,2],[5,3,4,[[7],7],3]]

[[[[6,2],5,10],0,8,[[10,1,4,4]]],[[[1],5,7]],[[[2,7,9,5,7],4,10],[[8,8,2],[],6,0,[3,8]],[[],[],[],[6,7,8],9]],[[6,4,[6,10],1,[3,2,8]],[],1,[[0,3,9,5,2],[3,2],9,8],1],[]]
[[2,[7,[3,5,6,0],4,1],6,[[7,2,3,10],3]],[],[6]]

[[8,[],[],[10,8,[0,7,8,8,3]],[[7,3,7],[8,8,10,10]]],[8,10,[4],7]]
[[[]]]

[[9,[[3,10]],5,10],[9,1],[5,4]]
[[[]],[6,6,5],[[1]],[6],[]]

[[6,[5,2],9],[6,9],[4,[[4,4],1,[0,2,4],8,1],0,8]]
[[[8]],[[[5,6],2,3,6],[2,9,[7,0,0,7,3]]],[[[],3]]]

[[[],10,[5,[3,3,3,6,0],[7,8,2]]],[],[]]
[[5,[8]]]

[[[4,[],[],[1,6,4,7,9]]],[[9,[1,4,7,10],9,[9,3,1,7]],2],[],[]]
[[[[2,6]],0,10,[1,10,0],[5,[5]]],[4],[4]]

[[10,[],10,2],[],[5]]
[[[]],[6,5,8,10,[[0],[5,9],[8]]],[],[10,[[8,2],7]],[[[],9],6,[[8,8],[1,3,7,3],6,8,[9,3,0,9,8]],[],9]]

[[[[],[7,10],0,5],[0,[]],9],[2,7]]
[[3,[]],[[6,10],[1,[7],[2,0],7,[]],[[6,8],1]]]

[[[],3]]
[[[3,9,6],9,[6,3,[0,8]],[6,8,5,[10,5,9,10]]]]

[[[7,2,10,[]],7,[[],[],[],10]],[0,0,9]]
[[],[[[1,1,8,1],[1],6,8],[[8,3,0],9,[3]],[4,[7],5,[1,2,9,8],[6]]],[8,[[],[7,2,4,6,5],[1,4,6,7]]],[[1,3,2,9],[[0,3,4,0,0],3],3,9],[5,[[5,7],[5,7,3,8,3],[6,0,2,3]],5]]

[[],[[1]],[5],[2,[],[2,[9,7,1,4],[9,4,4,2,4]],3]]
[[3],[],[6,[[10,10,10],2],[3,2],5],[]]

[[],[[9,[10,3],[1,4]]]]
[[5],[[],[[5,6],[],9],[]]]

[[5,5,10]]
[[[[10,9],[2,10,0,8],[4]]]]

[[],[[10,[]],[[8,7,3,9],2,10,7],1,7,[1,0]]]
[[[[6,4,3],0,5,3,9],[],9,[3,[4,4,0,1,9],10],[10,[0,2],[7,7,1,5,9]]],[2,3,3],[[],[2,1,[]],9,9],[[3],[5,9],[2,[10,7,3,4],[2],10]],[]]

[[5],[[],5,8,7],[[[0,0,10,6,5],3,[10,4,8],[]],[[3,5],[2,0,2,7],9],9,[7,9],[1]],[],[[0,5]]]
[[8,[8],3,[[4],8],[5]],[8,4,5,[7,1,[4,7,1,2],4,9]]]

[[2,9,4,[[10,8,10,2,6],[],7,[7,0,6,8,2]]]]
[[5,[],10],[2,[3,6],[],[],[]],[[6]],[9,7,[0]],[]]

[[1,1],[0]]
[[7,[9,10,5],[[5,6,5,6,7]],0,[[1,8,4,10,1]]],[],[[8],[[],9,6],10,9],[[[],10,0,3,10],[[3,3,8,5]]],[[1,6,10,5,[2,0,3,5,0]],[8,[],10,10]]]

[[[8,2,[],[],8],[[9,3],10]],[]]
[[[4,10,9,2],5,2],[[10]],[[1,7],[10,5,2],2,5,4]]

[[[9],4,4,9,[1]],[[7],5]]
[[],[8,6,[],[[0,4],[],[7,4,6],7,7],[]],[[4],[[7,3,6,8],5,1],[8,7,5],[2,7,[3,4,6,2],[],[]],8],[2],[[1,9,[9,8,9,2,7]],3,[],3,[]]]

[[2,[10,2],[[6],10,[10,6,8],[5,8,7,5,6]]],[2,8,9],[]]
[[5,[[4,10,6,10],5,[1,1,10,8],9,[4,1,4,0,3]],9,9]]

[[6,5],[[7,0]],[7,4,7]]
[[[7,[],[2,6],1,4],8,7,9],[[[]]],[[5,4,[2,9,1]],10,2]]

[[[[0,2,4,8]],[[6,10,8,6,1],10,[4],7,7],10,[[6,5],[],3,[8,6],[]],[[]]],[[9,10,7]],[],[7,6,[[]]],[[4,8,[6,5,3,5],9],6,[[7,8],[1,0,8,10,7],[7,3]],[7,2,0,8,[5,7]]]]
[[[[7,3,6],3,[6,1,10,9],7,[5,0,7]],10]]

[[[6,[6],9,[3,1,7,3]],6,[[6,9,6,10,0],[],5,4,3]],[[[10,1,2,4,2]],[2,[],[6],8],[[5,4],[3,4,9],[4],5],0]]
[[[[1,4],[2,5,8]]],[9,[4,0,1],[10,8,[3,9],6,7],[[3],1,[0,1],[3,0]],[[9],1,1,9]],[[9,[3],8],3,3],[]]

[[2,6,[2]],[4,2,9],[],[7,4,[2]]]
[[[[2,8,1,3],[],9,2],8,[],[8,[]]],[[],[[2,9,2,0],9,[],1],5],[8,[[7,7,3,8,3]],8,3],[[[]],[6,2,[6,8,6,1],8],[[0,6],[3,9,2],[8,7,4],9],[2],[6]],[2,8,9,[[5,2,4]],10]]

[[[]],[],[[[],10],[2,9,5],0,[[9,4,3,10,10],[2,0,8,9],[1,6,6],1,[]],[[10]]],[7,6,[[],[5],[2,6,7,10,5],8],[],[[],9,2,5,2]],[[3,3,[6,10,10]],6,8]]
[[1,0,4,[7],[7,9]],[10,[[1],[],2,8,0],0,7]]

[[[],10,6,3],[1,[[5,5,8],4]],[5,4]]
[[[[8],[2,7],[0,3,3,7]],1,[2,[1,5,5,5],[],[2]],1],[[[4,9,8,9],[8,10,4,2,3]],[],3],[[[10,0,6,10,7],6,[8,4,5,3,9],[1,7]],10],[[[9,3],9,[]],8,3,10,[10,10,[5,1,2,7,4]]]]

[[[[]],[[9],[4,1,4,4,0],7,1,3],8,[8,[10,8,6,1,3],[],[7,2],0],[]],[0,5,6],[5,[10,[0,3],4],[6,[5,9,7],[3,8],[4,1],9],3,[]],[5,[],10,[]],[0,9,[],3]]
[[10,3,8],[],[],[0,[6,0],[6],[[],0,[10,0,6,3]]]]

[[[[7,6,6],[7,1,0,9],6,5,[]],[0],[]],[6,0,5,0],[10,[[4,6]],[[3]]],[[],[[0,5,6,10],2,[3]],5]]
[[3,1,[[9],8],[9,10,2]]]

[[[],[4,3],[[],[9,3,6,9,7],10,[0,2]]],[[8,7,10],8]]
[[[3,9,[],[10],0],[2,10,2,0]],[10],[],[0,[3,[2,2]],10,3],[[[3,4,4,7,1],1,9],2,[3,[7,7],[4,4,0]],0,[]]]

[[6,8,[[0,4,0],2],[[3,2,9,2],0,[8,7,0,5,9],[3,8,2,6]]],[7,3,2],[6,[[]],[8,1,[8,4,2,0,7]],[],[[3,4,1,8,0],[7,5],6,[4,9],7]],[5,[4,[],1],[[8],1],6],[]]
[[[7,[]]],[1,10]]

[[0,3,9],[1],[3,0],[4],[[9,[4,8,8],[],8,[]]]]
[[[[0,10],[2,10,7],1,10]],[8,8,2,[6]],[[1],[2,[9,6,9],3,4],3],[5,2,[],[0,4],1],[[2,10,[0,3,6],7],9,[3,[5,7,4,8,3],4,4,0]]]

[[[4,[0,10,9],3,[]],2,0,[5,5,9,[],0],10],[[6,4,[6,2,9,9,2],[9,1,9,6],2]],[],[[],6,6,4,[]],[[],8,[[9,9,7],[2,0,3,8,0]]]]
[[],[4],[[4,[0,9,5,7,3]],3],[[9,[5,1],[6]],5]]

[[6,[[5],2],7,[0,9,[6,3,5,0,6],2],[[4,7,3,8],[9,8,6,5],8,8]],[]]
[[[[4,2,3,10],[],0]],[[[9,0,7,4],[6],3,8,5],[],3,3],[[3,[3],[],[3,5,2,4,1],[7,3,5]],9,4,1],[],[5,6,[1]]]

[[1,[],[6,[7,1]],10,[[1,7,2],8,[9,6,3]]],[4,[[],[8,8,5,5,3],1]],[[[]],6,2,[2],[[],3]]]
[[6,2,[3,6,0,5],[9,[9,8,9,3,10]]],[4,[6,7,8,[],[2]]],[4,[[1,9],[],[7,9,6,10,7],9,8],6,0],[]]

[[[[8]],[9,8,[3,2,6,2,7],[3,6,2,5],8],[[3,6],0]]]
[[[[5,3],1,7],9,3,[9,0,[3,2]],[10,9,[4,9,6,8,8],[],[7,5]]]]

[[4,2,5,6,[1,1,5,[8,2]]],[[0,[3],3],0,0,0],[7,[3,[9,9,5,1],[],3],[]],[4,[[9,8],[6]]]]
[[],[2,0,[[],[8,5,4,7],9,[3,5]],[9,2,[8,4],[7,5,6,7,3]],7]]

[[1],[[10,5],[[],[],[],3],7,4],[[10,[6,4,9,1],6,0,[2,0,0,6]],[5,2,9,[8,1]],[[9,0,2,3]],[],6]]
[[7,[2,[6],1,1,[1,8,1,6]],[],4],[7,[7]]]

[[[[5],10,[10],[],[8,6,4,2]],5],[8,[[5],6,[],[8,4,9]],[10,0]],[[3],[],9],[4,[6,6],[]]]
[[1],[4],[[7,[],[3,9],1],5,[[9,0,8,4],7,[8,6,3],[10,9,4,2],9],10],[[8,7,[2,8,10,9,9],9],5,[]]]

[[[[5,1,5],[8,4,6,5],[9,10,0]],7,4],[[[7,2,8,5],0,[]],[],3,4],[[4,3,[2,6,9,9,6],9,[8,7]],5,10],[1,[],9],[9,[[9,9]],[1,6]]]
[[[[9,5,9,10],[]]],[9],[]]

[[[[3,3,6,0],6],10]]
[[[3,2,[0],0],[],[[6]]],[5]]

[[[2],[]],[1],[[[9],10],5]]
[[8],[10,7],[],[4,6]]

[[[[5],10,[1,8,1,5,1]]],[[],[9,8,8]]]
[[3,1],[],[3,4,[[7,3],5]]]

[[3,10],[[4,7,[1,8,6,10,8],5]]]
[[[10,[8,6,7,6,10],[1]],[[10,4,5,5],[5,5,9,4,1]],5]]

[[[10,8,2]],[],[]]
[[[[0,9,0],[6],[0],6],8],[[10,[0,1]]]]

[[[]],[[[9,9,8],[],[8,10]]]]
[[5,[[0,6,0,2,1],4,9,[6,2]],9,[[4,4,8,0,0]],[[],[5,3,1]]],[8],[[[6,10]],2,6]]

[[[[6,1,5,10],6,[8,7,3]],5,[1,[10,5,7],2],[[7],8,6,8]],[[[10,10,5,4]],3]]
[[],[10,10,[9,[8,2,0,10],9,[7,7]],8,[6,9,[6,8],[5,5,10,5],4]],[5,8,10,[0]],[[[10]],[5,10,5,[0,5,9,10]],3],[]]

[[[[4,1,2,9],[8,5,10,5,7],0],[[0,3,6],10],8]]
[[[3,[2,2],[5],[8,2,0,0,9],4]],[8,7,[[3,8,9,4],10],7]]

[[9,9,[10,[0,5,1,5],[5,9],[7,10,9,10]],8],[1,0],[],[5,3,[5,10],2]]
[[[0,8,8,8]]]

[[5,[[10]]],[[],1,[[9],[10,6,6,0],4],[7,8,6]],[[5,[2]],[],[8,[10],1],10],[[10,[5,5,1],6,0],0]]
[[10,[9,1,[7,5,3],8],[[10],8]],[[6,[6]],[10,0,6,0,2]],[[[],0],[10,9,8,4,[1,6,8,3]]],[9,9,6],[10,[]]]

[[[[6],[1]],4,[[5,10],[0,10],6],[[5,7,10,10],[6,5,0,10],[],[3,9,8,1,0]],4],[],[[0],3,1,0],[6,[[9]],[[5,9],4,[9]]]]
[[[[9,8,6]]]]

[[10,6]]
[[],[4,[[0,3,8,8,1],[0,10,7,7,7],[],[8]],[[],[1,1,10],4],2,[[6,3,1,5]]],[]]

[[[],0,5,8]]
[[3,[10],10,[],7],[[[5],[8,8,6]],[[7,2,0],6],0,6],[[5],[[3,6,1,8,6],8,5],[[],[9,7,0,9,4],0,9],10,5]]

[[],[]]
[[1,0,[[5,4,5,2],0,6,[7,2,8],4],[[2,10],9,3],4],[4],[[4]],[[6,3,[],4],0,4],[4,[]]]

[[[[4],[2,4],6,[]],1],[10,[3,0,4,[2,0,4,3]],[[5,6,0,1,2]],[[3],0,[]],[[6,10]]],[4],[],[2,2]]
[[[3,[4,1,4,2],[1,3],[6,6]],[[10],9,[2,0,1]]],[3,9,[3,[5,9,5],7,[1,10,9],8],10],[[3,[7,9,1,10,4],[5,8],[9,10,8,7,1],5],9,2,[[3,7,0],10,[],[7,6,2,5,1],[]],[]]]

[[[[2,9],[]]],[[10,[2,10],[],[7,1,2],9]]]
[[5,9,6,[0,[2,9,6],[2,2,1]]],[1,[[8,4],[9,1],[7,1,10,5,4],[9,4]],9,6],[4]]

[[[0,[10,3,10,0],5,4,1],[],8,1,10],[[],[[7],[8,3,7,8],[3,9,2,0],9,[9,5,7,4,1]]],[],[0,10,[0,[9,5,0,5],[7,9,7,10,9],0,[]]],[]]
[[1,1,7,0],[10,7],[[1,0,[8],5],6]]

[[[8,4,[5,5,2],3]]]
[[0,7,[6,[],[10,5]],5],[3],[9,8,[],[[2],[1],9,[3,6]],[[]]],[0]]

[[[6],[[2,6,10],[8,3,10,1,10]],2],[[[8,2,8,10,3],5]]]
[[[[]],8,[[8,4],[],[],[4,4,1,0,10],[1,5,2,4,6]],3,[0,[8,0,2]]]]

[[8,3,[4,[]],[[3,5,5,7,8],[2],1]],[]]
[[[[6,5,1,3],0,6]],[7,9,[],0]]

[[4,0,[[8,0,10,7],10,1,[10,1,8,10],[]],4,10],[7,1],[8],[7,[[4,0,7,1]],[7],[]],[]]
[[8,[[10,8,4,7,10]],[[10,2]],[5],[[1,5,2,6]]],[[[4,4,3,4,4],6],9,[6],[0,[9,6,5,1],[9],[8,5,0],[0]]],[8,5,[8,[2,6,0,3,4],7,4],8,[6,[4,0,10],[]]],[[[8],[0,2,8,0]],[6,[1]],[],1],[]]

[[2],[[[8]],6],[2,[[7],10,0,[8,7]],9,[9]],[]]
[[2,[[4,9],4,[],9]],[[1,[8,7,3],0,2],2],[[],5,2],[0,10]]

[[4,7,0,[],[0,[0],[3,2,10,1],[0,4,4]]],[7,5,[[0,4,6]],[[2,9,1,4,0],9,[8,10,5,7]]],[6,[5,9,[8,0]],[5],[]]]
[[[6,[4,1,6,0,2],10,[2,5,3]],[5],9]]

[[8,[2],[]],[1,10,3],[10,[[1,8,6,7],9,6,0,5],[3,10,0]],[[[1,9]],5],[10]]
[[[[],[6,6,4,6,2],10,[7,5,8]],[[3,8,7,3],2,9],0],[2,1,[9,[7,7,3]],[9,8],[[3,9],[8,10],[7,10,6],0,[]]],[0,2,5],[[[3,6,6,10,3],[],[3,6,5,0],[]],[9,[4,6,8,5,9]],[[8],[1,1,8,1],7,4,[2,10]],8],[10,[6,[],[]]]]

[[[3],[],8,[[3,9,9]]]]
[[[[3,0,3],9,[3],2,10],6],[[],[0,7,8,8,[5]],9,[8]]]

[[1,3,[[5],[3,7,1],7,[10,8,5,10,6]]],[[10,5,4],7,[0,[4,8,1,4,5],[5,0,5,4,8],5,8]],[6],[7],[[[7,6]],9,[0,[9,10,10],9],2,9]]
[[5,10],[6,5,[]],[2,2,1,[],5]]

[[[9,[3,7,5,0,10],[2,4,8],[8,9,5]],[],2],[10,8],[[[10,6,4,9],[5,5,3,0,10],[8,10,3,7,5],[8,4,3,5]]]]
[[[1]],[[],6,[1,[4,3,6,8,8],7,[3,3,4,9],[9,8,1,3]],[[7,8],0,1]],[5,[8],8],[[[0],4,[1],[7,0,7,8,8],7],[6,5,[5,0]],10,[[7],[6],9]],[3,10,[2],[[8,2,4]],0]]

[[[3,9],5,3,7],[2,2,4],[[[7,1],3,4],[]],[[[8,1,10,9,10],7,[7,10,3,8],1,3]],[[10]]]
[[[0]],[]]

[[[]],[[4,8,[8,0,7,3]],5,3,1],[5,8,[8,[2,7,9,3,2],[5,0,1,4,7],5],8,[[9],3,[9],[8,2,5,3],[5,8,4]]],[9,1,1,[7],[]],[6,[2,[1,3]],[3,4,[8,6],0,2]]]
[[[2,7,2],[]],[6,3,[8]],[4]]

[[],[[2],[[5,8,1,9,6],[],7],4],[],[[6,9,[7],2,1],[8]]]
[[[3,7,[],6,3],[6],5,3]]

[[3,8,[[3,4,1,2,3]],[0,[1],[7,3,8,10,9]],[[2,10,7]]],[[[0,3,8,9],[0,6,8],8,1,2],7,[9,0,7,[9,5,0]],[[]],[[5,3,6]]]]
[[10,[]],[2,[4],[]],[[[0],[0,6,5],[7,4,3,2,3]]],[]]

[[4,[[8],[5,10,9,6,5]],9,5],[2],[2,10,5,[]],[[[0,10,8,1],3,6],8,8,[[5,10,9,6,10],[4,10,9,9],4,10]],[2,[4,[9]],8,1,[7,7,1,[]]]]
[[[8,5,0,[10,6,5],[0,6,1,0]],[[1,1,3],[6,6,3,7,5],9,10,[8]],[[3,3],7,[5,0],4,[9,3,9]]],[[4],[[6,6,0],7,5,3]],[],[3,[0,[10,8,10],[3,0],8],[3,3]],[[[10]],[[7,6],[2,7,10,0,5],6,0],[[9,3,10,2],8,[5,8,1]],10]]

[[[],6,9,[2,[10,4,6,9,1]]],[[[8,6,4,5],1,[5,5,3,7,8],[]]],[[[5,5,10,7]],[[2,4],0,[2,2],10,[]],[[1],[0],[1,1,7,3,8],[4,10,0,0]]]]
[[7,5,[[],10,[],6,[4,1,1,1]],[8,4,[4,0,9,0]],[]]]

[[[[4,0],[]]]]
[[[6],7],[[6],3],[6,[[2,6,10,7,8],1,2,10,4],[[1],6,5,6]],[]]

[[8,[2],[4]],[],[]]
[[[[6,8,10,8,5]],9],[]]

[[[]],[],[[[8,3,5,4,4],[9,5,4],[2],[9,4,3],[3,7,3,9]]],[4,[3,[]],0,[[10,9,0],2,8,[1],[]]]]
[[],[7,[]]]

[[],[1,[2,5,[]],4,10,2],[0,8,5,[[10,9],1],[[],3,[3,6,0,7,2],0,[2]]],[[[7,2,9,2],[],[3],8,[]],10,[8,0,[6]],[10]],[5,[[9,10],2,[9]],3,[9,[4,9,5,9,7]]]]
[[[[2],[5,1,3,2,10]],[7,[3,7,4,2,7],[9,2,6,8],1],[[8,4,3],[8,8,5],5,6,6]],[]]

[[6],[7,[[0]],[3,7,[7,5,8,0,9]]],[],[7,[3]],[0,0]]
[[0,0],[[9,0],[[10,7],[2,5,8,8],6],[[9,5,3,3,9],[7,4],8,6,2]]]

[[[],[3],1],[[[0,4,0,8,0],3,[5,1,9,5]],8,8,6]]
[[2,1,5]]

[[],[[8,[6,1,0,9],6],9,3,1]]
[[],[4]]

[[[[3,5,4,10,4],10,[2,3],9],8,[4,[2,4,3,10,6]],5]]
[[0,8,[]],[10],[8,[2,[9,6,9,9],[10,9]],8,[1,0,2],7],[[8,5,[1,2],[7,7,5,1],[1,5,6,6,0]],8,[],[0,[9],1,[3],[8,4]]],[[6],9,8,5,[0,3,[8,10,4,9]]]]

[[[1,[5,2,4,0],[3,8,7,3],[6,4],[]]],[],[],[7,8]]
[[[[7,4],7,[5,8,2,4,9],8,[8,7,10,7]]],[[5,[],8,[0,9,4,5,8]],[[5,1,8],10,[],7,8],0,[],1],[[[0,3,3],[]],2]]

[[],[[7,[7,10,3,8,8],[],6]],[3,[[2,0,8,2],[],2],[[4,7,2,7,10],5,[0,6]],10,[[],[8,0,8,9,0],[0,1,10,8,1],3,9]],[[[3,3,0,6]],[[10,1,0,5,8],7,3]],[0,[4,[1,1,0,10,10],5,1],[[1,4,8,2],8,[6,9,1],8],1]]
[[10,10],[6,6,[5,[],6,[4,4,9,9]],4],[],[7]]

[[],[[[8,5,2,0]],7,4],[[8,[7]],1],[],[0,0,[1,[0,6,10],[4],[],1],[4],3]]
[[[[],[10,1],9,[2,6]],[[],9],[3,[7,0,10,3,8],[0,6]],3],[1,[[3,7,6,5]]],[[]],[[],[10,2,0,[8,3]]]]

[[[0]],[4,7,[[3,8,4],9],[[8,1,5],[1,1,7,6,2]]],[],[]]
[[[5,0],[3],[1],[]],[[[5,1],6,7,0,10]],[[[6,0,1],0,[]],[9],[],[[9]],[9,[6,0,4,1]]],[]]

[[10],[],[3]]
[[[[5,2,0,3],5,[],[7,1,5,7]]]]

[[[5,10,[3,4,5,4],8,0]],[[4],[[3,9,2,8,0],6,6]],[[],7,5,[0,0,[],[10]]]]
[[2,3,5,[[9,5],6],0],[[[4,5,7,6,5],[6,1,9],8,[6,6,6],8],[7],8,5],[0,[6,[0,9,3,5],[9,1,9],3,7],[]],[[[1,6,0,5,0],[1,2,5,6,8],0,8],7]]

[[[],9],[]]
[[8],[[[]],[],[3,3,[10,7,9,0,6],0],9],[],[[[5,10,5,8,10],[5,1,8,10,8]],6,0,[3],[[]]]]

[[7,[]]]
[[[],9,[[0,9,1],[6],9,[8,4,10,4,7],[6]]],[3,[2,[0,5],7,9],2],[[[9],7,[1,7],9]]]
"#;
