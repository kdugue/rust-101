enum HockeyPosition {
    Center,
    Wing,
    Defense,
    Goalie,
}

struct HockeyPlayer {
    name: String,
    number: u8,
    position: HockeyPosition,
    goals_ytd: u8,
}

impl HockeyPlayer {
    fn new(name: String, number: u8, position: HockeyPosition) -> HockeyPlayer {
        HockeyPlayer {
            name: name,
            number: number,
            position: position,
            goals_ytd: 0,
        }
    }

    fn shoot_puck(&self, seconds_remaining: u16) {
        if seconds_remaining < 300 {
            match self.position {
                HockeyPosition::Center => println!("Goal!"),
                _ => println!("Miss!"),
            }
        } else {
            println!("Goal!");
        }
    }
}

pub fn get_struct() {
    let player = HockeyPlayer::new(String::from("Kemet"), 17, HockeyPosition::Wing);

    /*
    let player = HockeyPlayer {
        name: String::from("Kemet"),
        number: 23,
        position: HockeyPosition::Goalie,
        goals_ytd: 10,
    };
    */
    // why does semicolon have to go here

    println!(
        "{} has number {}  and has scored {} goals this seasion",
        player.name, player.number, player.goals_ytd
    );

    player.shoot_puck(1000);
    player.shoot_puck(2000);
}
