use rand::Rng;

trait Bite{
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Grapes{
    grapes_left: i32,
}

impl Bite for Grapes{
    fn bite(self: &mut Self) {
        self.grapes_left -= 1;
        println!("yum that grape was delicious.");
    }
}


pub(crate) fn main() {
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { grapes_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
        println!("yum that bite was delicious.");
    }
}

fn bunny_nibbles<T: Bite>(food: &mut T){
    let mut rng = rand::thread_rng();
    for _i in 0..=rng.gen_range(1..10){
        food.bite()
    }
}