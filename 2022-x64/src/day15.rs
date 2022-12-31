use std::collections::HashMap;
use std::collections::HashSet;

pub fn do_part_1() -> i64 {
    let map = parse_input(INPUT).expand_to_covered_area();
    return count_positions_where_a_beacon_cannot_be_present(&map, 2_000_000);
}

pub fn do_part_2() -> i64 {
    let map = parse_input(INPUT);
    let pt = map.find_uncovered_point(0, 4_000_000).unwrap();
    let res = (pt.x as i64) * 4_000_000 + pt.y as i64;
    return res;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }

    const fn manhattan_distance_to(&self, other: Point) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    position: Point,
    beacon: Point,
}

impl Sensor {
    const fn covers(&self, pt: Point) -> bool {
        let distance_to_beacon = self.position.manhattan_distance_to(self.beacon);
        return self.position.manhattan_distance_to(pt) <= distance_to_beacon;
    }
}

#[derive(Debug)]
struct SensorMap {
    top_left: Point,
    bottom_right: Point,
    sensors: Vec<Sensor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    None,
    Beacon,
    Sensor,
    Covered,
}

impl SensorMap {
    fn new(sensors: &[Sensor], expand_to_covered_area: bool) -> Self {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for s in sensors {
            if expand_to_covered_area {
                let dist = s.position.manhattan_distance_to(s.beacon);
                min_x = *[min_x, s.position.x - dist].iter().min().unwrap();
                max_x = *[max_x, s.position.x + dist].iter().max().unwrap();
                min_y = *[min_y, s.position.y - dist].iter().min().unwrap();
                max_y = *[max_y, s.position.y + dist].iter().max().unwrap();
            } else {
                min_x = *[min_x, s.position.x, s.beacon.x].iter().min().unwrap();
                max_x = *[max_x, s.position.x, s.beacon.x].iter().max().unwrap();
                min_y = *[min_y, s.position.y, s.beacon.y].iter().min().unwrap();
                max_y = *[max_y, s.position.y, s.beacon.y].iter().max().unwrap();
            }
        }

        return Self {
            top_left: Point::new(min_x, min_y),
            bottom_right: Point::new(max_x, max_y),
            sensors: sensors.iter().map(|s| *s).collect(),
        };
    }

    fn expand_to_covered_area(&self) -> Self {
        return Self::new(&self.sensors, true);
    }

    fn kind_at(&self, pt: Point) -> Kind {
        let mut is_covered = false;
        for s in &self.sensors {
            if s.position == pt {
                return Kind::Sensor;
            }
            if s.beacon == pt {
                return Kind::Beacon;
            }
            is_covered = is_covered || s.covers(pt);
        }
        if is_covered {
            return Kind::Covered;
        }
        return Kind::None;
    }

    fn find_uncovered_point(&self, min_coord: i32, max_coord: i32) -> Option<Point> {
        for sensor in &self.sensors {
            let dist = sensor.position.manhattan_distance_to(sensor.beacon) + 1;
            let x = sensor.position.x;
            let y = sensor.position.y;

            for delta in -dist..=dist {
                let x_delta = delta;
                let y_delta = dist - x_delta;
                let actual_dist = y_delta.abs() + x_delta.abs();
                if actual_dist != dist {
                    continue;
                }

                let pt = Point::new(x + x_delta, y + y_delta);
                if pt.x < min_coord || pt.x > max_coord || pt.y < min_coord || pt.y > max_coord {
                    continue;
                }

                if self.kind_at(pt) != Kind::None {
                    continue;
                }
                let sibling_pts = [
                    Point::new(pt.x, pt.y - 1),
                    Point::new(pt.x, pt.y + 1),
                    Point::new(pt.x + 1, pt.y),
                    Point::new(pt.x - 1, pt.y),
                ];
                let are_siblings_covered = sibling_pts
                    .iter()
                    .all(|x| self.kind_at(*x) == Kind::Covered);

                if are_siblings_covered {
                    return Some(pt);
                }
            }
        }

        return None;
    }

    fn to_string_with_coverage(&self, with_coverage: bool, expand_with: i32) -> String {
        let mut s = String::new();
        for y in (self.top_left.y - expand_with)..=(self.bottom_right.y + expand_with) {
            for x in (self.top_left.x - expand_with)..=(self.bottom_right.x + expand_with) {
                let pt = Point::new(x, y);
                let ch: char = match self.kind_at(pt) {
                    Kind::None => '.',
                    Kind::Beacon => 'B',
                    Kind::Sensor => 'S',
                    Kind::Covered => {
                        if with_coverage {
                            '#'
                        } else {
                            '.'
                        }
                    }
                };
                s.push(ch);
            }

            let needs_newline = y != self.bottom_right.y;
            if needs_newline {
                s.push('\n');
            }
        }
        return s;
    }
}

fn parse_input(data: &str) -> SensorMap {
    let values = find_integers(data);
    let mut sensors: Vec<Sensor> = Vec::new();

    assert_eq!(0, values.len() % 4);

    for chunk in values.chunks(4) {
        let sensor_pt = Point::new(chunk[0], chunk[1]);
        let beacon_pt = Point::new(chunk[2], chunk[3]);
        sensors.push(Sensor {
            position: sensor_pt,
            beacon: beacon_pt,
        });
    }

    return SensorMap::new(&sensors, false);
}

// what am i doing with my life
fn find_integers(data: &str) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let mut i: usize = 0;
    let bytes = data.as_bytes();

    while i < bytes.len() {
        let mut k = i;

        while bytes[k].is_ascii_digit() || bytes[k] == b'-' {
            k += 1;
        }

        if k != i {
            let thing: i32 = data[i..k].parse().unwrap();
            out.push(thing);
            i = k;
        } else {
            i += 1;
        }
    }

    return out;
}

fn count_positions_where_a_beacon_cannot_be_present(map: &SensorMap, y: i32) -> i64 {
    let mut sum: i64 = 0;

    for x in map.top_left.x..=map.bottom_right.x {
        let pt = Point::new(x, y);
        sum += match map.kind_at(pt) {
            Kind::Covered => 1,
            Kind::Sensor => 0,
            Kind::Beacon => 0,
            _ => 0,
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use crate::day15::*;

    #[test]
    fn test_count_positions_where_a_beacon_cannot_be_present() {
        let data = parse_input(TEST_INPUT);
        let s = count_positions_where_a_beacon_cannot_be_present(&data, 10);

        // println!("{:?}", data);

        assert_eq!(26, s);
    }

    #[test]
    fn test_find_uncovered_point() {
        let data = parse_input(TEST_INPUT);
        let pt = data.find_uncovered_point(0, 20);

        assert_eq!(Point::new(14, 11), pt.unwrap());
    }

    #[test]
    fn test_to_string() {
        let data = parse_input(TEST_INPUT);
        let s = r#"
....S.......................
......................S.....
...............S............
................SB..........
............................
............................
............................
..........S.......S.........
............................
............................
....B.......................
..S.........................
............................
............................
..............S.......S.....
B...........................
...........SB...............
................S..........B
....S.......................
............................
............S......S........
............................
.......................B....
        "#;

        let res = data.to_string_with_coverage(false, 0);

        assert_eq!(s.trim(), res);
    }
}

const TEST_INPUT: &'static str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

const INPUT: &'static str = r#"
Sensor at x=325337, y=2568863: closest beacon is at x=-518661, y=2000000
Sensor at x=3988825, y=837820: closest beacon is at x=4305648, y=2127118
Sensor at x=1611311, y=2053174: closest beacon is at x=2827226, y=1579510
Sensor at x=101890, y=3940049: closest beacon is at x=955472, y=3457514
Sensor at x=3962702, y=2558425: closest beacon is at x=4226981, y=2604726
Sensor at x=2957890, y=2160813: closest beacon is at x=2827226, y=1579510
Sensor at x=3907456, y=3325610: closest beacon is at x=3696221, y=3226373
Sensor at x=3354177, y=3435919: closest beacon is at x=3696221, y=3226373
Sensor at x=3997379, y=3071868: closest beacon is at x=3696221, y=3226373
Sensor at x=145143, y=1714962: closest beacon is at x=-518661, y=2000000
Sensor at x=611563, y=3148864: closest beacon is at x=955472, y=3457514
Sensor at x=3080405, y=3904777: closest beacon is at x=3696221, y=3226373
Sensor at x=644383, y=10732: closest beacon is at x=364635, y=-294577
Sensor at x=3229566, y=1694167: closest beacon is at x=2827226, y=1579510
Sensor at x=1600637, y=3984884: closest beacon is at x=955472, y=3457514
Sensor at x=2959765, y=2820860: closest beacon is at x=2491502, y=2897876
Sensor at x=2235330, y=3427797: closest beacon is at x=2491502, y=2897876
Sensor at x=2428996, y=210881: closest beacon is at x=2827226, y=1579510
Sensor at x=369661, y=687805: closest beacon is at x=364635, y=-294577
Sensor at x=3558476, y=2123614: closest beacon is at x=4305648, y=2127118
Sensor at x=3551529, y=2825104: closest beacon is at x=3696221, y=3226373
Sensor at x=64895, y=3577: closest beacon is at x=364635, y=-294577
Sensor at x=3079531, y=1538659: closest beacon is at x=2827226, y=1579510
"#;
