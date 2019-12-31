
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Team {
    Known(String),
    Winner(u32), // represents winner of X
    Loser(u32),
    Bye,
}

impl fmt::Display for Team {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Team::Known(name) => write!(f, "{}", name),
            Team::Winner(round) => write!(f, "Winner of {}", round),
            Team::Loser(round) => write!(f, "Loser of {}", round),
            Team::Bye => write!(f, "Bye"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Match {
    team_a : Team,
    team_b : Team,
    id : u32,
}

impl Match {
    pub fn new(a : Team, b : Team, id : u32) -> Match {
        Match {
            team_a: a,
            team_b: b,
            id: id,
        }
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.team_a == Team::Bye {
            write!(f, "{} has a bye", self.team_b)
        } else if self.team_b == Team::Bye {
            write!(f, "{} has a bye", self.team_a)
        } else {
            write!(f, "Match {}: {} vs {}", self.id, self.team_a, self.team_b)
        }
    }
}

/*impl PartialEq for Match {
    fn eq(&self, b : &Match) -> bool {
        self.id == b.id
    }
}*/

pub fn fill_byes(team_names : Vec<String>) -> Vec<Team> {
    let num_teams = team_names.len();
    let mut num = 1;
    while num < num_teams {
        num *= 2;
    }
    let mut res = team_names.iter().map(|tn| Team::Known(tn.to_string())).collect::<Vec<Team>>();
    for _ in num_teams..num {
        res.push(Team::Bye);
    }
    res
}

fn do_bracket_round(teams : &mut Vec<Team>, match_number : u32, avoid_double_byes : bool) -> (u32, Vec<Team>, Vec<Team>, Vec<Match>) {
    let mut winners = Vec::new();
    let mut losers = Vec::new();
    let mut matches = Vec::new();
    let mut match_num = match_number;
    let mut i = 0;
    let mut bye_index = None;
    if teams.len() % 2 == 1 {
        teams.push(Team::Bye);
    }
    while teams.len() >= 2 {
        let team_a = teams.remove(0);
        let team_b = teams.pop().expect("Failed to remove last element of teams array");
        let (winner, loser) = match (team_a.clone(), team_b.clone()) {
            (Team::Bye, b) => (Some(b), None),
            (a, Team::Bye) => (Some(a), None),
            (Team::Bye, Team::Bye) => (None, None),
            _ => {
                match_num += 1;
                (Some(Team::Winner(match_num - 1)), Some(Team::Loser(match_num - 1)))
            },
        };
        if avoid_double_byes && (team_a == Team::Bye || team_b == Team::Bye) {
            bye_index = Some(i);
        }
        if let Some(winner) = winner {
            winners.push(winner);
        }
        if let Some(loser) = loser {
            losers.push(loser);
        }
        matches.push(Match::new(team_a, team_b, match_num - 1));
        i += 1;
    }
    if let Some(bye_index) = bye_index {
        let num_winners = winners.len();
        winners.swap(bye_index, num_winners - 1);
    }
    (match_num, winners, losers, matches)
}

pub fn x_elim(team_names : Vec<String>, x : usize, avoid_double_byes : bool) -> (Vec<Vec<Vec<Match>>>, Vec<Team>, u32) {
    /*let mut winning_teams = fill_byes(team_names);
    let mut losing_teams : Vec<Team> = Vec::new();*/
    let mut bracket_teams : Vec<Vec<Team>> = (1..x).map(|_| Vec::new()).collect();
    bracket_teams.insert(0, fill_byes(team_names));
    let mut match_num = 1;
    let mut res = Vec::new();
    while bracket_teams.iter().any(|b| b.len() > 1) {
        let mut all_bracket_matches : Vec<Vec<Match>> = (0..x).map(|_| Vec::new()).collect();
        for i in 0usize..x {
            if bracket_teams[i].len() > 1 {
                let (new_match_num, winners, losers, matches) = do_bracket_round(bracket_teams.get_mut(i).expect(&format!("Failed to get brackets at {}", i)), match_num, avoid_double_byes);
                bracket_teams[i] = winners;
                if i != x - 1 {
                    bracket_teams[i + 1].extend(losers);
                }
                match_num = new_match_num;
                all_bracket_matches[i] = matches;
            }
        }
        res.push(all_bracket_matches);
    }
    (res, Vec::new(), match_num)
}

/*pub fn double_elim(team_names : Vec<String>) -> (Vec<[Vec<Match>; 2]>, Team, Team, u32) {
    let mut winning_teams = fill_byes(team_names);
    let mut losing_teams : Vec<Team> = Vec::new();
    let mut match_num = 1;
    let mut res = Vec::new();
    while winning_teams.len() > 1 || losing_teams.len() > 1 {
        let mut o_winners_matches = Vec::new();
        let mut o_losers_matches = Vec::new();
        if winning_teams.len() > 1 {
            // cycle of winner's
            let (new_w_match_num, winners_winners, winners_losers, winners_matches) = do_bracket_round(&mut winning_teams, match_num);
            winning_teams = winners_winners;
            o_winners_matches = winners_matches;
            match_num = new_w_match_num;
            losing_teams.extend(winners_losers);
        }
        if losing_teams.len() > 1 {
            // cycle of loser's
            let (new_l_match_num, losers_winners, _, losers_matches) = do_bracket_round(&mut losing_teams, match_num);
            println!("Stays in losers: {:?}", losers_winners);
            losing_teams = losers_winners;
            o_losers_matches = losers_matches;
            match_num = new_l_match_num;
        }
        res.push([o_winners_matches, o_losers_matches]);
    }
    if winning_teams.len() != 1 || losing_teams.len() != 1 {
        panic!("Failed to get down to a single team in the winners' and losers' brackets");
    }
    (res, winning_teams[0].clone(), losing_teams[0].clone(), match_num)
}

// not including byes
pub fn gen_bracket(team_names : Vec<String>) -> Vec<Vec<Match>> {
    let mut teams = fill_byes(team_names);
    let mut res = Vec::new();
    let mut match_number = 1;
    while teams.len() >= 2 {
        let mut round_matches = Vec::new();
        let mut next_teams = Vec::new();
        while teams.len() > 0 {
            let teams_in_match = [teams[0].clone(), teams[teams.len() - 1].clone()];
            if teams_in_match[0] == Team::Bye && teams[1] == Team::Bye {
                panic!("Match between two byes");
            } 
            round_matches.push(Match::new(teams_in_match[0].clone(), teams_in_match[1].clone(), match_number));
            let winner = if teams_in_match[0] == Team::Bye {
                teams_in_match[1].clone()
            } else if teams_in_match[1] == Team::Bye {
                teams_in_match[0].clone()
            } else {
                let w = Team::Winner(match_number);
                match_number += 1;
                w
            };
            teams.remove(0);
            teams.remove(teams.len() - 1);
            next_teams.push(winner);
        }
        teams = next_teams;
        if !round_matches.is_empty() {
            res.push(round_matches);
        }
    }
    res
}
*/