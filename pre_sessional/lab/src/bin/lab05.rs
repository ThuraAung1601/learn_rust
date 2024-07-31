use rand::Rng;
use std::io;

#[derive(Clone)]
struct Enemy {
    name: String,
    hp: i32,
    stamina: i32,
    power: i32,
    full_hp: i32,
}

struct Player {
    hp: i32,
    stamina: i32,
    power: i32,
    gold: i32,
    steps: i32,
    actions: i32,
}

enum EncounterType {
    Nothing,
    Bush,
    Meat,
    Water,
    Herb,
    IronOre,
    Enemy(EnemyType),
}

enum EnemyType {
    Rat,
    Wolf,
    Boar,
    Tiger,
    Dragon,
}

enum Direction {
    North,
    South,
    West,
    East,
    Exit,
}

impl Player {
    fn new() -> Player {
        Player {
            hp: 100,
            stamina: 60,
            power: 10,
            gold: 0,
            steps: 0,
            actions: 0,
        }
    }

    fn is_alive(&self) -> bool {
        self.hp > 0 && self.stamina > 0
    }

    fn walk(&mut self) {
        self.stamina -= 1;
        self.steps += 1;
        println!("You walked. Stamina is now: {}", self.stamina);
    }

    fn encounter(&mut self, enemies: &Vec<Enemy>) {
        let mut rng = rand::thread_rng();
        let encounter_type: EncounterType = match rng.gen_range(1..=100) {
            1..=17 => EncounterType::Nothing,
            18..=25 => EncounterType::Bush,
            26..=40 => EncounterType::Meat,
            41..=55 => EncounterType::Water,
            56..=69 => EncounterType::Herb,
            70..=73 => EncounterType::IronOre,
            74..=100 => {
                let enemy_type: EnemyType = match rng.gen_range(1..=100) {
                    1..=40 => EnemyType::Rat,
                    41..=55 => EnemyType::Boar,
                    56..=65 => EnemyType::Tiger,
                    66..=70 => EnemyType::Dragon,
                    _ => EnemyType::Wolf,
                };
                EncounterType::Enemy(enemy_type)
            }
            _ => unreachable!(),
        };

        match encounter_type {
            EncounterType::Nothing => println!("Nothing happens."),
            EncounterType::Bush => {
                self.stamina -= 1;
                println!("You walked into a bush. Stamina is now: {}", self.stamina);
            }
            EncounterType::Meat => {
                self.hp = std::cmp::min(self.hp + 4, 100);
                println!("You found meat. HP is now: {}", self.hp);
            }
            EncounterType::Water => {
                self.stamina = std::cmp::min(self.stamina + 2, 100);
                println!("You found water. Stamina is now: {}", self.stamina);
            }
            EncounterType::Herb => {
                self.power += 1;
                println!("You found an herb. Power is now: {}", self.power);
            }
            EncounterType::IronOre => {
                self.power += 10;
                println!("You found iron ore. Power is now: {}", self.power);
            }
            EncounterType::Enemy(enemy_type) => {
                let enemy = self.create_enemy(&enemy_type, enemies);
                println!(
                    "You encountered a {} with HP: {}, Stamina: {}, Power: {}.",
                    enemy.name, enemy.hp, enemy.stamina, enemy.power
                );
                print!("\x07"); // Beep sound
                self.fight_or_flee(enemy);
            }
        }
        self.actions += 1;
    }

    fn create_enemy(&self, enemy_type: &EnemyType, enemies: &Vec<Enemy>) -> Enemy {
        match enemy_type {
            EnemyType::Rat => enemies[0].clone(),
            EnemyType::Wolf => enemies[1].clone(),
            EnemyType::Boar => enemies[2].clone(),
            EnemyType::Tiger => enemies[3].clone(),
            EnemyType::Dragon => enemies[4].clone(),
        }
    }

    fn fight_or_flee(&mut self, mut enemy: Enemy) {
        let mut rng = rand::thread_rng();
        loop {
            println!("Player - HP: {}, Stamina: {}, Power: {}", self.hp, self.stamina, self.power);
            println!("Enemy - HP: {}, Stamina: {}, Power: {}", enemy.hp, enemy.stamina, enemy.power);
            println!("Do you want to (1) Fight or (2) Flee?");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read line");
            let choice: i32 = choice.trim().parse().expect("Please type a number");

            match choice {
                1 => {
                    self.stamina -= 1;
                    let hit_chance: i32 = rng.gen_range(1..=100);
                    if hit_chance <= 100 - enemy.stamina / 2 {
                        enemy.hp -= self.power;
                        println!("You hit the enemy! Enemy HP is now: {}", enemy.hp);
                        print!("\x07"); // Beep sound
                    } else {
                        println!("You missed!");
                    }

                    if enemy.hp <= 0 {
                        println!("You defeated the {}!", enemy.name);
                        self.gold += enemy.full_hp;
                        println!(
                            "You gained {} gold. Total gold: {}",
                            enemy.full_hp, self.gold
                        );
                        break;
                    }

                    let enemy_hit_chance: i32 = rng.gen_range(1..=100);
                    if enemy_hit_chance <= 70 - self.stamina / 2 {
                        self.hp -= enemy.power;
                        println!("The enemy hit you! Your HP is now: {}", self.hp);
                    } else {
                        println!("The enemy missed!");
                    }

                    if self.hp <= 0 {
                        println!("You have been defeated!");
                        break;
                    }
                }
                2 => {
                    self.stamina -= 2;
                    let flee_chance: i32 = rng.gen_range(1..=100);
                    if flee_chance <= 90 + self.stamina - enemy.stamina {
                        println!("You successfully fled!");
                        break;
                    } else {
                        println!("You failed to flee!");
                        let enemy_hit_chance: i32 = rng.gen_range(1..=100);
                        if enemy_hit_chance <= 100 - self.stamina / 2 {
                            self.hp -= enemy.power;
                            println!("The enemy hit you! Your HP is now: {}", self.hp);
                        } else {
                            println!("The enemy missed!");
                        }

                        if self.hp <= 0 {
                            println!("You have been defeated!");
                            break;
                        }
                    }
                }
                _ => println!("Invalid choice, please choose (1) Fight or (2) Flee."),
            }
        }
    }

    fn show_status(&self) {
        println!("Player Status:");
        println!("HP: {}", self.hp);
        println!("Stamina: {}", self.stamina);
        println!("Power: {}", self.power);
        println!("Gold: {}", self.gold);
        println!("Steps taken: {}", self.steps);
        println!("Actions performed: {}", self.actions);
    }
}

fn main() {
    let enemies = vec![
        Enemy {
            name: String::from("Rat"),
            hp: 10,
            stamina: 10,
            power: 2,
            full_hp: 10,
        },
        Enemy {
            name: String::from("Wolf"),
            hp: 20,
            stamina: 20,
            power: 10,
            full_hp: 20,
        },
        Enemy {
            name: String::from("Boar"),
            hp: 30,
            stamina: 40,
            power: 20,
            full_hp: 30,
        },
        Enemy {
            name: String::from("Tiger"),
            hp: 40,
            stamina: 50,
            power: 30,
            full_hp: 40,
        },
        Enemy {
            name: String::from("Dragon"),
            hp: 60,
            stamina: 60,
            power: 40,
            full_hp: 60,
        },
    ];

    let mut player = Player::new();

    println!("Welcome to the Text Adventure Game!");
    while player.is_alive() && player.gold < 200 {
        println!("Choose a direction to walk: (1) North, (2) South, (3) West, (4) East, (9) Exit and show status");
        let mut direction = String::new();
        io::stdin().read_line(&mut direction).expect("Failed to read line");
        let direction: i32 = direction.trim().parse().expect("Please type a number");

        if direction == 9 {
            player.show_status();
            break;
        }

        if direction >= 1 && direction <= 4 {
            player.walk();
            player.encounter(&enemies);
        } else {
            println!("Invalid direction. Please choose (1) North, (2) South, (3) West, (4) East, or (9) Exit and show status.");
        }

        if player.gold >= 200 {
            println!("Congratulations! You've collected more than 200 gold and won the game!");
            player.show_status();
            break;
        } else if !player.is_alive() {
            println!("Game Over! You've either run out of HP or stamina.");
            player.show_status();
            break;
        }
    }
}
