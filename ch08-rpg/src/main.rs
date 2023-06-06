use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

trait Enchanter : std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let prob_of_success = self.competency();
        let spell_success = rand::thread_rng().gen_bool(prob_of_success);

        println!("{:?} mutters incoherently. ", self);
        if spell_success {
            println!("The {:?} glows brightly!", thing);
        } else {
            println!("The {:?} fizzes, then turns into a worthless trinket.", thing);
            *thing = Thing::Trinket;
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.3
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

fn main() {
    let mut it = Thing::Sword;

    let d = Dwarf {};
    let h = Human {};
    let e = Elf {};

    let party: Vec<&dyn Enchanter> = vec![&d, &h, &e];
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it);
}