#[derive(Clone)]
struct Match {
    bracket : u32,
    round : u32,
    id : String,
    upper : String,
    lower : String,
}

impl Match {
    fn def() -> Match {
        Match {
            bracket: 0,
            round: 0,
            id: String::new(),
            upper: String::new(),
            lower: String::new(),
        }
    }
}

impl std::fmt::Display for Match {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {} VS {}", self.id, self.upper, self.lower)
    }
}

fn main() {
    let num_participants = 32768;
    let mut participants = (1..=num_participants).map(|n| n.to_string()).collect::<Vec<String>>();
    let num_matches = num_participants - 1;
    let mut matches = vec![Match::def(); num_matches];
    let num_rounds = (num_participants as f64).log(2.) as u32;
    let last = &mut matches[num_matches - 1];
    last.bracket = 1;
    last.round = num_rounds;
    last.upper = participants[0].clone();
    last.lower = participants[1].clone();
    let mut opp_seed = 2usize.pow(num_rounds) - num_participants + 1;
    let mut round = 1;
    let mut match_num = 1;
    for i in (2..=num_participants).rev() {
        let test_num = (i as f64).log(2.);
        if test_num.fract() == 0. {
            opp_seed = 1;
            round += 1;
            match_num = 1;
        }
        let l = &mut matches[num_participants - i];
        l.upper = participants[opp_seed - 1].clone();
        l.lower = participants[i - 1].clone();
        l.round = round;
        l.bracket = 1;
        l.id = format!("B{}R{}M{}", 1, round, match_num);
        
        participants[opp_seed - 1] = format!("Winner of {}", matches[num_participants - i].id);

        println!("{}", matches[num_participants - i]);
        
        opp_seed += 1;
        match_num += 1;
    }
}

