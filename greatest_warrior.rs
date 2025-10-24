pub struct Warrior {
    exp: u16,
    achievements: Vec<&'static str>
}

impl Warrior {
    pub fn new() -> Self {
        Self {
            exp: 100,
            achievements: vec![]
        }
    }

    pub fn level(&self) -> u16 {
        (self.exp/100).min(100)
    }

    pub fn experience(&self) -> u16 {
        self.exp.min(10000)
    }

    pub fn rank(&self) -> &str {
        ["Pushover", "Novice", "Fighter", "Warrior", "Veteran", "Sage", "Elite", "Conqueror", "Champion", "Master", "Greatest"][(self.exp.min(10000)/1000) as usize]
    }

    pub fn achievements(&self) -> &[&'static str] {
        &self.achievements
    }

    pub fn training(&mut self, desc: &'static str, exp: u16, min_level: u16) -> &'static str {
        if self.level()<min_level {return "Not strong enough";}
        self.achievements.push(desc);
        self.exp += exp;
        desc
    }

    pub fn battle(&mut self, n: u16) -> &str {
        if n<1 || n>100 {return "Invalid level";}
        let lvl = self.level();
        if (n/10).saturating_sub(lvl/10)>=1 && n-lvl>=5 {return "You've been defeated";}
        let x = if lvl==n {10} else if lvl.saturating_sub(n)==1 {5} else if lvl.saturating_sub(n)>=2 {0} else {20*(n-lvl)*(n-lvl)};
        self.exp += x;
        if lvl.saturating_sub(n)>=2 {"Easy fight"} else if lvl>=n && lvl-n<=1 {"A good fight"} else {"An intense fight"}
    }
}
