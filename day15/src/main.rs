use std::io::{stdin, BufRead, BufReader};
use regex::Regex;


#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x : i32,
    y : i32,
}

impl Point {
  fn dist(self : &Point, rhs : &Point) -> i32 {
    (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Sensor {
  location : Point,
  range : i32,
}

impl Sensor {
  fn dist(self: &Sensor, p : &Point) -> i32 {
    self.location.dist(p)
  }
}

#[derive(Clone, Debug)]
struct Board {
    left : i32,
    width : i32,
    sensors : Vec<Sensor>,
    beacons : Vec<Point>,
}

impl Board {
    fn new(sensors: Vec<Point>, beacons: Vec<Point>) -> Self {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let sensors = sensors.iter().zip(beacons.iter()).map(|(sensor, beacon)| {
            let dist = sensor.dist(beacon);
            min_x = min_x.min(sensor.x-dist);
            max_x = max_x.max(sensor.x+dist);
            Sensor { location: *sensor, range: dist}
        }).collect();

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

    fn get(self: &Board, p : &Point) -> u8 {
      if self.sensors.iter().find(|s| s.location == *p).is_some() { return b'S' }
      if self.beacons.contains(&p) { return b'B' }

      if self.sensors.iter().any(|s| s.dist(p) <= s.range) { b'#' } else { b'.'}
    }

    fn is_empty(self: &Board, p : &Point) -> bool {
      if self.sensors.iter().any(|s| s.dist(p) <= s.range) { false } else { true }
    }

}
fn find_sensor(sd : &mut Vec<Sensor>, xs : &mut Vec<i32>) -> Option<Sensor>{
  //we're looking for a sensor that can advance our current "right edge"
  //which is stored in xs

  //for it to qualify we first test each left hemisphere of the sensors range
  //if every point is at most one unit to the right of the current max then
  //  we can use the sensor b/c it wouldn't leave a gap
  let pos = sd.iter().position(|s| {
    println!("testing sensor {:?}", s);
    let mut left_hemi = get_sensor_left_hemisphere(&s);
    left_hemi.retain(|p| p.y >= 0 && p.y < xs.len() as i32);
    //println!("{:?}", left_hemi.len());
    left_hemi.iter().all(|left_hemi_point| left_hemi_point.x <= xs[left_hemi_point.y as usize])
  });

  pos.map(|i| sd.swap_remove(i) )
}

fn get_sensor_left_hemisphere(s : &Sensor) -> Vec<Point> {
  //start at the top of the perimeter and go couter-clockwise to the bottom
  let mut result = vec![];
  //down and left
  let deltas = [(-1,1),(1,1)];
  let mut v = Point { x: s.location.x, y: s.location.y - s.range};
  result.push(v);
  for (dx, dy) in deltas {
    for _ in 0..s.range {
      v.x += dx;
      v.y += dy;
      result.push(v);
    }
  }
  result
}

fn get_sensor_right_hemisphere(s : &Sensor) -> Vec<Point> {
  //start at the top of the perimeter and go clockwise to the bottom
  let mut result = vec![];
  //down and left
  let deltas = [(1,1),(-1,1)];
  let mut v = Point { x: s.location.x, y: s.location.y - s.range};
  result.push(v);
  for (dx, dy) in deltas {
    for _ in 0..s.range {
      v.x += dx;
      v.y += dy;
      result.push(v);
    }
  }
  result
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let mut lines_it = br.lines().map(|v| v.unwrap());

    let mut sensors = vec![];
    let mut beacons = vec![];
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    while let Some(line) = lines_it.next() {
      let matches = re.captures(&line).unwrap();
      let coords : Vec<i32> = matches.iter().skip(1).map(|v| v.unwrap().as_str().parse::<i32>().unwrap()).collect();
      sensors.push( Point {x: coords[0], y: coords[1]});
      beacons.push( Point {x: coords[2], y: coords[3]});
    }

    let board = Board::new(sensors, beacons);

    for y in [10, 2000000] {
      let mut answer = 0;
      for x in board.left..board.left+board.width {
        if board.get(&Point {x, y }) == b'#' {
          answer += 1;
        }
      }
      println!("answer for y = {} -> {}", y, answer);
    }

    const MAX_INDEX : i32 = 4_000_000;

    let mut xs = vec![0; MAX_INDEX as usize];
    let mut sensors : Vec<Sensor> = board.sensors.clone();
    loop {
      let opt_sensor = find_sensor(&mut sensors, &mut xs);
      if opt_sensor.is_none() {
        //test all of the current xs to see which one is not covered by any sensor
        for y in 0..MAX_INDEX {
          let x = xs[y as usize];
          let p = Point{ x , y};
          if x < MAX_INDEX && board.is_empty(&p) {
            println!("found a location: {:?}", p);
            println!("tuning frequency is {}", p.x * 4000000 + p.y);
            return;
          }
        }
        panic!("couldn't find a sensor or a gap");
      }

      let s = opt_sensor.unwrap();
      println!("found sensor {:?}", s);
      let mut candidates = get_sensor_right_hemisphere(&s);
      candidates.retain(|c| c.y >= 0 && c.y < xs.len() as i32);

      for c in candidates {
        xs[c.y as usize] = xs[c.y as usize].max(c.x+1);
      }
    }
}
