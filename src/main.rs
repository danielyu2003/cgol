// Daniel Yu
// 4/15/23

use std::time;
use std::thread::sleep;

mod update;

// Defines board size
pub const SIZE : usize = 20;

fn render(arr : &[[u8; SIZE]; SIZE], reset : bool) -> ()
{
    for row in arr { println!("{:?}", row); }
    if reset { print!("\x1b[{}A", SIZE); }
}

fn updatearr(arr : &mut[[u8; SIZE]; SIZE]) -> Option<()>
{
    let prev = arr.clone();
    update::step(arr, &prev);
    if *arr == prev { return None; }
    Some(())
}

fn init(state : &mut[[u8; SIZE]; SIZE]) -> ()
{
    state[10][8] = 1;
    state[10][9] = 1;
    state[10][10] = 1;
    state[9][10] = 1;
    state[8][9] = 1;
}

fn main() -> ()
{
    let mut state = [[0u8; SIZE]; SIZE];

    init(&mut state);

    render(&state, true);
    loop 
    {
        sleep(time::Duration::from_secs(1));
        if updatearr(&mut state) == None 
        {
            render(&state, false);
            return ();
        }
        render(&state, true);
    }
}