#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(
            Self{
                id: id,
                p1: (p1_name, 0),
                p2: (p2_name, 0),
                nb_games: nb_games
            }
        )
    }
    pub fn read_winner(&self) -> (String, u16) {
        println!("read winn: {:?}", self);
        if self.p1.1 > self.p2.1{
            return self.p1.clone()
        }else if self.p1.1 < self.p2.1{
            return self.p2.clone();
        }else if self.p1.1 == self.p2.1 && self.p1.1 != 0{
            return ("Same score! tied".to_string(), self.p1.1)
        }
        return ("Same score! tied".to_string(), 0)
    }
    pub fn update_score(&mut self, user_name: String) {
        // println!("update {:?}, {}",self, user_name);
        let total_score = self.p1.1 + self.p2.1;
        let (max, min) = if self.p1.1 > self.p2.1 {(self.p1.1, self.p2.1)} else {(self.p2.1, self.p1.1)};

        if user_name == self.p1.0 && total_score < self.nb_games || (self.nb_games - total_score) + min >= max{
            self.p1.1 +=1
        }else if user_name == self.p2.0 && total_score < self.nb_games{
            self.p2.1 += 1
        }
    }
    pub fn delete(self) -> String {
        return format!("game deleted: id -> {}", self.id)
    }
}