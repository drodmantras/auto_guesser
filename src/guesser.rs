use std::cmp::Ordering;
use rand::Rng;

mod functions;

pub fn run() {

    println!("vnesi spodnjo mejo: ");
    let mut spodnja = functions::vnos();
    println!("vnesi zgornjo mejo: ");
    let mut zgornja = functions::zgornja(spodnja);

    let line_spodnja = spodnja;
    let line_zgornja = zgornja;

    print!("\n");

    let skrivnost = rand::thread_rng().gen_range(spodnja, zgornja);

    let mut osnova: i32 = rand::thread_rng().gen_range(spodnja, zgornja);

    let velikost: usize = zgornja as usize;
    let mut seznam = vec![0; velikost];

    let mut tries = 0;

    loop {

        tries+=1;

        match osnova.cmp(&skrivnost) {
            Ordering::Less => {

                spodnja = osnova;
                seznam[tries-1]=osnova;
                osnova = rand::thread_rng().gen_range(spodnja, zgornja);
            },
            Ordering::Equal => {
                print!("uganjeno stevilo je ");
                print!("{}", osnova);
                print!(" po {} poskusih.", tries);

                functions::line(line_spodnja, osnova, line_zgornja, &mut seznam);

                functions::vrsta(osnova, &mut seznam);

                break;
            },
            Ordering::Greater => {

                zgornja = osnova;
                seznam[tries-1]=osnova;
                osnova = rand::thread_rng().gen_range(spodnja, zgornja);
            },
        }

    }

}
