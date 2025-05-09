

#[derive(Debug)]
struct Character {
    name :String ,
    health :i32,
    attack :i32,
    defence :i32,


}

impl Character{

    fn new(name :String , health :i32 , attack :i32 , defence :i32) -> Self{
        Character {
            name,
            health,
            attack,
            defence
        }
    }
    fn attack(&mut self, target :&mut Character){
        let damage = self.attack - target.defence;
        if(damage > 0){
            target.health -= damage;
        }
        else{
            println!("{} defence is too high", target.name);
        }
    }

    fn defence(&mut self){
        self.defence += 5;
    }
    fn heal(&mut self){
        self.health += 10;
    }
    fn is_alive(&self) -> bool {
        self.health > 0
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_health(&self) -> i32 {
        self.health
    }
}
fn main(){
    let mut hero = Character::new("hero".to_string(), 100 , 20, 10);

    let mut monster = Character::new("monster".to_string(), 80 , 15, 5);

   loop{
    println!("{} health: {}", hero.get_name(), hero.get_health());
    println!("{} health: {}", monster.get_name(), monster.get_health());
    println!("Choose an action: 1. Attack 2. Defence 3. Heal for hero");
    let mut action = String::new();
    std::io::stdin().read_line(&mut action).unwrap();

   match action.trim() {
       "1" => hero.attack(&mut monster),
       "2" => hero.defence(),
       "3" => hero.heal(),
       _ => println!("Invalid action"),

   }

    if !monster.is_alive() {
          println!("{} is defeated!", monster.get_name());
          break;
     }
     println!("Choose an action: 1. Attack 2. Defence 3. Heal for monster");
     let mut action = String::new();
     std::io::stdin().read_line(&mut action).unwrap();
     match action.trim() {
          "1" => monster.attack(&mut hero),
          "2" => monster.defence(),
          "3" => monster.heal(),
          _ => println!("Invalid action"),
     }

     if !hero.is_alive() {
          println!("{} is defeated!", hero.get_name());
          break;
     }
    }
}