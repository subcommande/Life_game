extern crate rand;

use std::io;
use rand::Rng;
use std::{thread, time};
use std::sync::{Mutex, Arc};

const ROWS: usize = 40;
const COLUMNS: usize = 40;

fn main() {
    let input = read_input();
    let fps: f64 = read_fps();
    let ping = 1000.0/fps;
    let mut world: Vec<Vec<bool>> = create_world(input);
    let mut day = 0;
    let start_rate: f32 = count_alive(&world);

    loop {
        display_world(&world, ping as u64);

        println!("\nDay: {}, Alive at start: {}, Alive now: {}\n", day, start_rate, count_alive(&world));

        if is_dead(&world) {
            break;
        }

        day += 1;

        world = process_day(world);
    }
}

fn create_world(count: usize) -> Vec<Vec<bool>> {
    let mut random: Vec<Vec<usize>> = Vec::new();
    for _i in 0..count {
        loop {
            let mut temp_vec: Vec<usize> = Vec::new();
            temp_vec.push(rand::thread_rng().gen_range(0, ROWS));
            temp_vec.push(rand::thread_rng().gen_range(0, COLUMNS));

            if !random.contains(&temp_vec) {
                random.push(temp_vec);
                break;
            }
        }
    }

    let mut world: Vec<Vec<bool>> = Vec::new();

    for i in 0..ROWS {
        let mut string: Vec<bool> = Vec::new();
        for j in 0..COLUMNS {
            let mut coords: Vec<usize> = Vec::new();
            coords.push(i);
            coords.push(j);
            if random.contains(&coords) {
                string.push(true);
            } else {
                string.push(false);
            }
        }
        world.push(string);
    }

    return world;
}

fn bool_to_char(bl: bool) -> char {
    let digit: char;

    if bl {
        digit = '0';
    } else {
        digit = '_';
    }
    return digit;
}

fn process_day(world: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let temp_world = world.clone();
    let temp_world = Arc::new(Mutex::new(temp_world));
    let mut handles = vec![];

    let (snd, rcv) = std::sync::mpsc::channel::<Box<Fn() -> () + Send>>();

    for i in 0..ROWS {
        let handle = thread::spawn(move || {
            while let Ok(f) = rcv.recv() {
                f(i:Mutex<usize>, world:Mutex<Vec<Vec<bool>>>, temp_world:Arc<Mutex<Vec<Vec<bool>>>>);
            }
        });
        handles.push(handle);
    }

    for i in 0..ROWS {
        let world = Mutex::new(world.clone());
        let i = Mutex::new(i.clone());
        let temp_world = temp_world.clone();


        snd.send(Box::new(process_thread(i, world, temp_world))).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return temp_world.lock().unwrap().clone();
}

fn display_world(world: &Vec<Vec<bool>>, sleep_in_millis: u64) -> () {
    thread::sleep(time::Duration::from_millis(sleep_in_millis));

    print!("{}[2J", 27 as char);

    for i in 0..ROWS {
        for j in 0..COLUMNS {
            print!("{} ", bool_to_char(world[i][j]));
        }
        println!();
    }
    println!();
}

fn inc(digit: &usize, sign: bool) -> usize {
    let result: usize;

    if sign {
        if digit < &(ROWS - 1) {
            result = digit + 1;
        } else {
            result = 0;
        }
    } else {
        if digit > &0 {
            result = digit - 1;
        } else {
            result = ROWS - 1;
        }
    }

    return result;
}

fn is_dead(world: &Vec<Vec<bool>>) -> bool {
    let mut dead: bool = true;

    for i in 0..ROWS {
        for j in 0..COLUMNS {
            if world[i][j] {
                dead = false;
            }
        }
    }

    return dead;
}

fn count_alive(world: &Vec<Vec<bool>>) -> f32 {
    let mut counter: f32 = 0.0;
    for i in 0..ROWS {
        for j in 0..COLUMNS {
            if world[i][j] {
                counter += 1.0;
            }
        }
    }

    return counter;
}

fn read_input() -> usize {
    let mut input = String::new();

    println!("Total frames: {}\nPlease write number of alive frames at 0 day: ", ROWS * COLUMNS);

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let mut input: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => ROWS * COLUMNS / 2
    };

    if input > ROWS * COLUMNS {
        input = ROWS * COLUMNS;
    }

    return input;
}

fn read_fps() -> f64 {
    let mut fps = String::new();

    println!("Please write FPS (0 means max possible): ");

    io::stdin().read_line(&mut fps)
        .expect("Failed to read line");

    let fps: f64 = match fps.trim().parse() {
        Ok(num) => num,
        Err(_) => 1000.0,
    };

    return fps;
}

fn process_thread(i: Mutex<usize>, world: Mutex<Vec<Vec<bool>>>, temp_world: Arc<Mutex<Vec<Vec<bool>>>>) {
    for j in 0..COLUMNS {

        let world = world.lock().unwrap();
        let i = i.lock().unwrap();
        let mut temp_world = temp_world.lock().unwrap();

        let mut string = Vec::new();
        let mut counter = 0;
        let i: usize = *i;
        let j: usize = j;

        string.push((world)[inc(&i, false)][inc(&j, false)]);
        string.push((world)[inc(&i, false)][j]);
        string.push((world)[inc(&i, false)][inc(&j, true)]);
        string.push((world)[i][inc(&j, false)]);
        string.push((world)[i][inc(&j, true)]);
        string.push((world)[inc(&i, true)][inc(&j, false)]);
        string.push((world)[inc(&i, true)][j]);
        string.push((world)[inc(&i, true)][inc(&j, true)]);

        for m in 0..8 {
            if string[m] {
                counter += 1;
            }
        }

        if world[i][j] {
            if counter > 3 {
                temp_world[i][j] = false;
            }
            if counter < 2 {
                temp_world[i][j] = false;
            }
        } else {
            if counter == 3 {
                temp_world[i][j] = true;
            }
        }
    }
}


