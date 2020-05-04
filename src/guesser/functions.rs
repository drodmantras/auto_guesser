use std::io;

pub fn vrsta(stevilo: i32, seznam: &mut [i32]) {

    let mut x = 1;

    print!("\ngenerirani poskusi po vrsti: ");

    for stev in seznam {
        let num = *stev;
        if num != 0 {
            if x>=10 {
                print!("\n");
                x=1;
            }
            print!(" {},", num);
        } else {
            break;
        }
        x+=1;
    }

    print!(" {}.\n", stevilo);
}

pub fn line(spodnja: i32, stevilo: i32, zgornja: i32, seznam: &mut [i32]) {

    print!("\nprikaz poskusov: {} ", spodnja);

    if spodnja+zgornja > 100 {
        print!("{} ---------- {} ---------- {}       [crta za prikaz je skrcena]", spodnja, stevilo, zgornja);
    } else {
        let mut x = 2;

        while x < stevilo {
            if seznam.contains(&x) {
                print!(" {} ", x);
            } else {
                print!("-");
            }
            x+=1;
        }

        print!(" {} ", stevilo);
        x+=1;

        while x < zgornja {
            if seznam.contains(&x) {
                print!(" {} ", x);
            } else {
                print!("-");
            }
            x+=1;
        }

        print!(" {}\n", zgornja);
    }
}

pub fn zgornja(spodnja: i32) -> i32 {
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

pub fn vnos() -> i32 {

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
