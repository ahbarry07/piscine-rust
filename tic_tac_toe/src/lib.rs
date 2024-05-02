
pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if (table[0][0] == "O" && table[1][0] == "O" && table[2][0] == "O") || (table[0][1] == "O" && table[1][1] == "O" && table[2][1] == "O") || (table[0][2] == "O" && table[1][2] == "O" && table[2][2] == "O") ||
        (table[0][0] == "O" && table[0][1] == "O" && table[0][2] == "O") || (table[1][0] == "O" && table[1][1] == "O" && table[1][2] == "O") || (table[2][0] == "O" && table[2][1] == "O" && table[2][2] == "O") ||
        (table[0][0] == "O" && table[1][1] == "O" && table[2][2] == "O") || (table[0][2] == "O" && table[1][1] == "O" && table[2][0] == "O")
    {
        return String::from("player O won");
    }
    if  (table[0][0] == "X" && table[1][0] == "X" && table[2][0] == "X") || (table[0][1] == "X" && table[1][1] == "X" && table[2][1] == "X") || (table[0][2] == "X" && table[1][2] == "X" && table[2][2] == "X") || //vertical
        (table[0][0] == "X" && table[0][1] == "X" && table[0][2] == "X") || (table[1][0] == "X" && table[1][1] == "X" && table[1][2] == "X") || (table[2][0] == "X" && table[2][1] == "X" && table[2][2] == "X") || //horizontal
        (table[0][0] == "X" && table[1][1] == "X" && table[2][2] == "X") || (table[0][2] == "X" && table[1][1] == "X" && table[2][0] == "X") //diagonal
    {
        return String::from("player X won");
    }
    String::from("tie")   
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if (table[0][0] == player && table[1][1] == player && table[2][2] == player) || (table[0][2] == player && table[1][1] == player && table[2][0] == player){
        return true;
    }
    return false;
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if (table[0][0] == player && table[0][1] == player && table[0][2] == player) || (table[1][0] == player && table[1][1] == player && table[1][2] == player) || 
        (table[2][0] == player && table[2][1] == player && table[2][2] == player){
            return true;
    }
    return false;
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if (table[0][0] == player && table[1][0] == player && table[2][0] == player) || (table[0][1] == player && table[1][1] == player && table[2][1] == player) || 
        (table[0][2] == player && table[1][2] == player && table[2][2] == player){
            return true;
        }
    return  false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
