use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let skrivnost = rand::thread_rng().gen_range(1, 101);
    println!("generirana={}", skrivnost);
    let mut osnova: i32 = rand::thread_rng().gen_range(1, 101);
    let mut spodnja = 1;
    let mut zgornja = 101;

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
