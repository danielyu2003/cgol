// Daniel Yu
// 4/15/23

use crate::SIZE;

// returns number of neighbors at a given position on the board
fn neighbors(x : usize, y : usize, arr : &[[u8; SIZE]; SIZE]) -> u8
{
	let left_x : usize;
	let right_x : usize;
	let top_y : usize;
	let bottom_y : usize;

	let mut count : u8 = 0;

	if x == 0 { left_x = SIZE - 1; }
	else { left_x = x - 1; }
	
	if x == SIZE - 1 { right_x = 0; }
	else { right_x = x + 1; }
	
	if y == 0 { top_y = SIZE - 1; }
	else { top_y = y - 1; }
	
	if y == SIZE - 1 { bottom_y = 0; }
	else { bottom_y = y + 1; }	

	count += arr[top_y][left_x];
	count += arr[top_y][x];
	count += arr[top_y][right_x];
	count += arr[y][left_x];
	count += arr[y][right_x];
	count += arr[bottom_y][left_x];
	count += arr[bottom_y][x];
	count += arr[bottom_y][right_x];
	
	count
}

// updates current board state
pub fn step(arr : &mut [[u8; SIZE]; SIZE], origin : &[[u8; SIZE]; SIZE]) -> ()
{
	for y in 0..SIZE
    {
        for x in 0..SIZE
        {
            let count = neighbors(x, y, origin);
            if count < 2 || count > 3
            {
            	arr[y][x] = 0;
            }
            if count == 3
            {
            	arr[y][x] = 1;
            }
        }
    }
}

