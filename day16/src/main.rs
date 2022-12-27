use regex::Regex;
use std::io::{stdin, BufRead, BufReader};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Pipe {
  name: String,
  flow: i32,
  is_open: bool,
  connected_to: Vec<String>,
}

enum Step {
  MoveTo(String),
  OpenValve,
}

struct State {
  location: String,
  steps: Vec<Step>,
  pressure: i32,
}

fn main() {
    let br = BufReader::new(stdin().lock());
    let lines_it = br.lines().map(|v| v.unwrap());

    let line_re =
        Regex::new(r"Valve (.{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();

    let dest_re = Regex::new(r"(\w{2})").unwrap();
    let pipes: Vec<Pipe> = lines_it.map( |line| {
        let caps = line_re.captures(&line).unwrap();
        let name = caps[1].to_string();
        let flow = caps[2].parse::<i32>().unwrap();
        let destinations = caps.get(3).unwrap().as_str();

        let mut connected_to: Vec::<String> =
            dest_re.captures_iter(destinations).map( |caps| caps[1].to_string()).collect();

        Pipe { name, flow, is_open: false, connected_to}

    }).collect();

    let mut working_set =  VecDeque::from([State { location: "AA".to_string(), steps: vec![], pressure: 0}]);

    while let Some(state) = working_set.pop_front() {

    }
}
