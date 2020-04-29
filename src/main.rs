use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {

    println!("vnesi spodnjo mejo: ");
    let mut spodnja = vnos();
    println!("vnesi zgornjo mejo: ");
    let mut zgornja = zgornja(spodnja);

    print!("\n");

    let skrivnost = rand::thread_rng().gen_range(spodnja, zgornja);
    println!("{}----------{}----------{}", spodnja, skrivnost, zgornja);
    let mut osnova: i32 = rand::thread_rng().gen_range(spodnja, zgornja);

    let velikost: usize = zgornja as usize;
    let mut seznam = vec![0; velikost];

    let mut tries = 0;

    loop {

        tries+=1;

        match osnova.cmp(&skrivnost) {
            Ordering::Less => {
                // println!("{}, {}", tries, osnova);
                // println!("more");
                // println!("spodnja set to {}", spodnja);
                spodnja = osnova;
                seznam[tries-1]=osnova;
                osnova = rand::thread_rng().gen_range(spodnja, zgornja);
            },
            Ordering::Equal => {
                print!("uganjeno stevilo je ");
                print!("{}", osnova);
                print!(" po {} poskusih.", tries);
                print!("\ngenerirani poskusi po vrsti: \n");

                let mut x = 1;
                for stev in seznam {
                    if stev != 0 {
                        if x>=10 {
                            print!("\n");
                            x=1;
                        }
                        print!(" {},", stev);
                    } else {
                        break;
                    }
                    x+=1;
                }

                print!(" {}\n", osnova);
                break;
            },
            Ordering::Greater => {
                // println!("{}, {}", tries, osnova);
                // println!("less");
                // println!("zgornja set to {}", zgornja);
                zgornja = osnova;
                seznam[tries-1]=osnova;
                osnova = rand::thread_rng().gen_range(spodnja, zgornja);
            },
        }

    }
}

fn zgornja(spodnja: i32) -> i32 {
    let mut newzgornja = vnos();

    loop {
        if newzgornja > spodnja {
            break;
        } else {
            println!("zgornja meja naj je večja od spodnje!");
            newzgornja = zgornja(spodnja);
        }
    }
    newzgornja
}

fn vnos() -> i32 {

    println!("(stevilo naj je pozitivno)");
    let mut vn = String::new();

    io::stdin()
        .read_line(&mut vn)
        .expect("napaka branja!");

    let vn = vn.trim().parse::<i32>().expect("napacen vnos!");

    if vn < 1 {
        println!("Napaka! stevilo mora biti pozitivno! ");
        let vn = vnos();
        vn
    } else {
        vn
    }

}
