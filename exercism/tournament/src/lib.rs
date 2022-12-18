use std::collections::HashMap;
use std::cmp::{ PartialOrd, PartialEq, Ordering };


#[derive(Clone, Copy, Debug)]
enum MatchResult {
    Win = 3,
    Draw = 1,
    Loss = 0,
}

impl From<&str> for MatchResult {
    fn from(result: &str) -> Self {
        match result {
            "win" => Self::Win,
            "draw" => Self::Draw,
            "loss" => Self::Loss,
            _ => panic!("invalid match result")
        }
    }
}

impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            Self::Win => Self::Loss,
            Self::Loss => Self::Win,
            Self::Draw => Self::Draw,
        }
    }
}


#[derive(Debug, PartialEq)]
struct Team {
    name: String,
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}
impl Team {
    fn new(name: String) -> Self {
        Self { name, mp: 0, w: 0, d: 0, l: 0, p: 0 }
    }

    fn apply_result(&mut self, result: &MatchResult) {
       self.mp += 1;
       self.p += *result as u32;
       match result {
           MatchResult::Win => self.w += 1,
           MatchResult::Loss => self.l += 1,
           MatchResult::Draw => self.d += 1,
       }
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.p.partial_cmp(&other.p) {
            Some(Ordering::Equal) => {},
            ord => return ord,
        }
        return other.name.partial_cmp(&self.name);
    }
}


#[derive(Debug)]
struct MatchRecord {
    team1: String,
    team2: String,
    result: MatchResult,
}

impl From<&str> for MatchRecord {
    fn from(match_line: &str) -> Self {
        let split_result = match_line.split(";").collect::<Vec<&str>>();
        Self { team1: split_result[0].to_owned(), team2: split_result[1].to_owned(), result: MatchResult::from(split_result[2]) }
    }
}

impl MatchRecord {
    fn apply_result(&self, teams: &mut HashMap<String, Team>) {
        let team1 = teams.entry(self.team1.to_owned()).or_insert_with(|| Team::new(self.team1.to_owned()));
        team1.apply_result(&self.result);
        let team2 = teams.entry(self.team2.to_owned()).or_insert_with(|| Team::new(self.team2.to_owned()));
        team2.apply_result(&self.result.reverse());
    }
}

fn get_tally_string(teams: &Vec<&Team>) -> String {
    let header = "Team                           | MP |  W |  D |  L |  P".to_string();
    if teams.len() == 0 {
        return header;
    }
    let teams_string = teams.into_iter().map(|team| format!(
        "{:30} |  {} |  {} |  {} |  {} |  {}", 
        team.name,
        team.mp,
        team.w,
        team.d,
        team.l,
        team.p,
    )).collect::<Vec<String>>();
    header
    + "\n"
    + teams_string.join("\n").as_str()
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Team>  = HashMap::new();
    match_results.lines()
    .map(|line| MatchRecord::from(line))
    .for_each(|match_result| match_result.apply_result(&mut teams));
    let mut final_tally = teams.values().collect::<Vec<&Team>>();
    final_tally.sort_by(|a, b| b.partial_cmp(a).unwrap());    
    return get_tally_string(&final_tally)
}
