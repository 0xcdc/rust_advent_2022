use regex::Regex;
use std::io::{stdin, BufRead, BufReader};

extern crate gcollections;
extern crate interval;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(self: &Point, rhs: &Point) -> i32 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Sensor {
    location: Point,
    range: i32,
}

impl Sensor {
    fn dist(self: &Sensor, p: &Point) -> i32 {
        self.location.dist(p)
    }
}

#[derive(Clone, Debug)]
struct Board {
    left: i32,
    width: i32,
    sensors: Vec<Sensor>,
    beacons: Vec<Point>,
}

impl Board {
    fn new(sensors: Vec<Point>, beacons: Vec<Point>) -> Self {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let sensors = sensors
            .iter()
            .zip(beacons.iter())
            .map(|(sensor, beacon)| {
                let dist = sensor.dist(beacon);
                min_x = min_x.min(sensor.x - dist);
                max_x = max_x.max(sensor.x + dist);
                Sensor {
                    location: *sensor,
                    range: dist,
                }
            })
            .collect();

        let left = min_x;
        let width = max_x - min_x + 1;

        println!("Board: {} {} {:?}", left, width, sensors);
        Board {
            left,
            width,
            sensors,
            beacons,
        }
    }

    fn get(self: &Board, p: &Point) -> u8 {
        if self.sensors.iter().any(|s| s.location == *p) {
            return b'S';
        }
        if self.beacons.contains(p) {
            return b'B';
        }

        if self.sensors.iter().any(|s| s.dist(p) <= s.range) {
            b'#'
        } else {
            b'.'
        }
    }
}

fn get_sensor_hemisphere_common<'a>(
    s: &'a Sensor,
    deltas: &'a [(i32, i32); 2],
) -> impl Iterator<Item = Point> + 'a {
    let mut count = -1;
    let mut v = Point {
        x: s.location.x,
        y: s.location.y - s.range,
    };

    std::iter::from_fn(move || {
        //first point is the top, so don't move any direction
        if count == -1 {
            count += 1;
            return Some(v);
        }

        //we've completed the hemisphere
        if count >= s.range * 2 {
            return None;
        }

        let (dx, dy) = if count >= 0 && count < s.range {
            //first arc
            deltas[0]
        } else {
            //second arc
            deltas[1]
        };

        v.x += dx;
        v.y += dy;
        count += 1;
        Some(v)
    })
}

fn get_sensor_left_hemisphere(s: &Sensor) -> impl Iterator<Item = Point> + '_ {
    //start at the top of the perimeter and go couter-clockwise to the bottom
    const LEFT_SIDE_DELTAS: [(i32, i32); 2] = [(-1, 1), (1, 1)];
    get_sensor_hemisphere_common(s, &LEFT_SIDE_DELTAS)
}

fn get_sensor_right_hemisphere(s: &Sensor) -> impl Iterator<Item = Point> + '_ {
    //start at the top of the perimeter and go clockwise to the bottom
    const RIGHT_SIDE_DELTAS: [(i32, i32); 2] = [(1, 1), (-1, 1)];
    get_sensor_hemisphere_common(s, &RIGHT_SIDE_DELTAS)
}

fn main() {
    use gcollections::ops::*;
    use interval::interval_set::*;

    let br = BufReader::new(stdin().lock());
    let lines_it = br.lines().map(|v| v.unwrap());

    let mut sensors = vec![];
    let mut beacons = vec![];
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    for line in lines_it {
        let matches = re.captures(&line).unwrap();
        let coords: Vec<i32> = matches
            .iter()
            .skip(1)
            .map(|v| v.unwrap().as_str().parse::<i32>().unwrap())
            .collect();
        sensors.push(Point {
            x: coords[0],
            y: coords[1],
        });
        beacons.push(Point {
            x: coords[2],
            y: coords[3],
        });
    }

    let board = Board::new(sensors, beacons);

    for y in [10, 2000000] {
        let mut answer = 0;
        for x in board.left..board.left + board.width {
            if board.get(&Point { x, y }) == b'#' {
                answer += 1;
            }
        }
        println!("answer for y = {} -> {}", y, answer);
    }

    const MAX_INDEX: i32 = 4_000_001;

    let mut covered_ranges = vec![IntervalSet::<i32>::empty(); MAX_INDEX as usize];
    let mut sensors: Vec<Sensor> = board.sensors;
    while let Some(s) = sensors.pop() {
        let left_points = get_sensor_left_hemisphere(&s);
        let right_points = get_sensor_right_hemisphere(&s);

        for (left, right) in left_points.zip(right_points) {
            assert_eq!(left.y, right.y);
            if left.y >= 0 && left.y < covered_ranges.len() as i32 {
                let i = vec![(left.x, right.x)].to_interval_set();
                covered_ranges[left.y as usize] = covered_ranges[left.y as usize].union(&i);
            }
        }
    }

    //test all of the covered_ranges to see which one is not covered by any sensor
    let full_row = vec![(0, MAX_INDEX)].to_interval_set();
    for y in 0..MAX_INDEX {
        let covered_range = &covered_ranges[y as usize];
        let missing = full_row.difference(covered_range);
        if missing.is_empty() {
            continue;
        }

        println!("found a y that doesn't cover every range {} {}", y, missing);
        assert_eq!(missing.interval_count(), 1);

        let missing_interval = missing.iter().next().unwrap();
        assert_eq!(missing_interval.size(), 1);

        let x = missing_interval.lower();
        let p = Point { x, y };
        println!("found a location: {:?}", p);
        println!("tuning frequency is {}", p.x as i64 * 4000000 + p.y as i64);
        return;
    }
    panic!("couldn't find a sensor or a gap");
}
