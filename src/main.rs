// Daniel Yu

use std::time;
use std::thread::sleep;

mod update;

pub const SIZE : usize = 20;

fn printarr(arr : &[[u8; SIZE]; SIZE], reset : bool) -> ()
{
    for row in arr { println!("{:?}", row); }
    if reset { print!("\x1b[{}A", SIZE); }
}

fn updatearr(arr : &mut[[u8; SIZE]; SIZE]) -> Option<()>
{
    let last = arr.clone();
    update::step(arr, &last);
    if update::and_arrs(arr, &last) == true { return None; }
    Some(())
}


fn initstate(state : &mut[[u8; SIZE]; SIZE]) -> ()
{
    // glider configuration
    state[10][8] = 1;
    state[10][9] = 1;
    state[10][10] = 1;
    state[9][10] = 1;
    state[8][9] = 1;
}

fn main() -> ()
{
    let mut state = [[0u8; SIZE]; SIZE];

    initstate(&mut state);

    printarr(&state, true);
    loop 
    {
        sleep(time::Duration::from_secs(1));
        if updatearr(&mut state) == None 
        {
            printarr(&state, false);
            return ();
        }
        printarr(&state, true);
    }
}