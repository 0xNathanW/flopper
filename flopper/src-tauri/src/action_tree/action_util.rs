use gto::action::Action;


pub fn action_to_str(action: Action) -> String {
    match action {
        Action::Fold => "Fold:0".to_string(),
        Action::Call => "Call:0".to_string(),
        Action::Check => "Check:0".to_string(),
        Action::Bet(amount) => format!("Bet:{}", amount),
        Action::Raise(amount) => format!("Raise:{}", amount),
        Action::AllIn(amount) => format!("AllIn:{}", amount),
        _ => unreachable!(),
    }
}

pub fn decode_action(s: &str) -> Action {
    match s {
        "F" => Action::Fold,
        "C" => Action::Call,
        "X" => Action::Check,
        _ => {
            let mut chars = s.chars();
            let first = chars.next().unwrap();
            let amount = chars.as_str().parse().unwrap();
            match first {
                'B' => Action::Bet(amount),
                'R' => Action::Raise(amount),
                'A' => Action::AllIn(amount),
                _ => unreachable!(),
            }
        }
    }
}

pub fn encode_action(action: Action) -> String {
    match action {
        Action::Fold => "F".to_string(),
        Action::Call => "C".to_string(),
        Action::Check => "X".to_string(),
        Action::Bet(amount) => format!("B{}", amount),
        Action::Raise(amount) => format!("R{}", amount),
        Action::AllIn(amount) => format!("A{}", amount),
        _ => unreachable!(),
    }
}

pub fn encode_line(line: &[Action]) -> String {

    let mut flag = 0;
    let mut encoded = String::new();

    if line.is_empty() {
        return "(Root)".to_string();
    }

    for &action in line {
        if !encoded.is_empty() {
            let delimiter = if flag == 2 { "|" } else { "-" };
            flag = if flag == 2 { 0 } else { flag };
            encoded.push_str(delimiter); 
        }
        match action {
            Action::Check => flag += 1,
            Action::Call => flag = 2,
            _ => flag = 0,
        }
        encoded.push_str(&encode_action(action));
    }

    encoded
}