pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let mut winner = String::from("tie");
    let players = vec!["X", "O"];
    for player in players {
        if diagonals(player, &table) || horizontal(player, &table) || vertical(player, &table) {
            winner = player.to_string();
            break;
        }
    }
    if winner == "tie" {
        format!("{}", winner)
    } else {
        format!("player {} won", winner)
    }
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut diag1 = true;
    let mut diag2 = true;
    for i in 0..table.len() {
        if table[i][i] != player {
            diag1 = false;
        }
        if table[i][table.len() - 1 - i] != player {
            diag2 = false;
        }
    }
    diag1 || diag2
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for r in table {
        let mut win = true;
        for c in r {
            if *c != player {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for i in 0..table.len() {
        let mut win = true;
        for j in 0..table.len() {
            if table[j][i] != player {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    false
}
