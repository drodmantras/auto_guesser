use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {

    println!("vnesi spodnjo mejo: ");
    let mut spodnja = vnos();
    println!("vnesi zgornjo mejo: ");
    let mut zgornja = vnos();
    print!("\n");

    let skrivnost = rand::thread_rng().gen_range(spodnja, zgornja);
    println!("generirana={}", skrivnost);
    let mut osnova: i32 = rand::thread_rng().gen_range(spodnja, zgornja);

    let mut tries = 0;

    loop {

        tries+=1;

        match osnova.cmp(&skrivnost) {
            Ordering::Less => {
                // println!("{}, {}", tries, osnova);
                // println!("more");
                // println!("spodnja set to {}", spodnja);
                spodnja = osnova;
                osnova = rand::thread_rng().gen_range(spodnja, zgornja);
            },
            Ordering::Equal => {
                println!("uganjeno stevilo je {} po {} poskusih.", osnova, tries);
                break;
            },
            Ordering::Greater => {
                // println!("{}, {}", tries, osnova);
                // println!("less");
                // println!("zgornja set to {}", zgornja);
                zgornja = osnova;
                osnova = rand::thread_rng().gen_range(spodnja, zgornja);
            },
        }

    }
}

fn vnos() -> i32 {

    println!("(stevilo naj je pozitivno)");
    let mut vn = String::new();

    io::stdin()
        .read_line(&mut vn)
        .expect("napaka branja!");

    let vn = vn.trim().parse::<i32>().expect("napačen vnos!");

    if vn < 1 {
        println!("Napaka! stevilo mora biti pozitivno! ");
        let mut vn = vnos();
        vn
    } else {
        vn
    }

}
