/***
	a variety of initial boards states.
***/
const BOARD_SIZE:usize = 9*9;

pub mod hard_boards{	
	//src: the first 3 are from
	//krazydad chpt 8, vol 1. https://krazydad.com/sudoku/sfiles/KD_Sudoku_CH_8_v1.pdf
	const BOARD_CHALLENGING_1:[usize; 9*9] = [
		
		0, 0, 0,	0, 0, 0,	2, 0, 8,
		9, 2, 0, 	0, 0, 4,	0, 0, 0,
		0, 0, 0,	2, 0, 8,	0, 7, 1,

		0, 3, 6,	0, 0, 0,	0, 0, 0,
		0, 0, 0, 	7, 0, 9, 	0, 0, 0,
		0, 0, 0,	0, 0, 0,	6, 4, 0,

		8, 6, 0,	4, 0, 1, 	0, 0, 0,
		0, 0, 0,	9, 0, 0,	0, 2, 7,
		2, 0, 9,	0, 0, 0,	0, 0, 0
	];
	
	const BOARD_CHALLENGING_2: [usize: BOARD_SIZE] = [
		0, 0, 9,	0, 0, 0,	1, 0, 0,
		0, 0, 4,	0, 3, 0,	0, 0, 0,
		0, 0, 0,	5, 6, 7,	0, 3, 0,
		
		0, 0, 0,	0, 0, 0,	0, 1, 7,
		8, 0, 1,	0, 0, 0,	2, 0, 4,
		2, 9, 0		0, 0, 0,	0, 0, 0,
		
		0, 7, 0,	3, 5, 1,	0, 0, 0,
		0, 0, 0,	0, 4, 0,	6, 0, 0,
		0, 0, 8,	0, 0, 0,	9, 0, 0,
	];
	
	const BOARD_CHALLENGING_3: [usize; BOARD_SIZE] = [
		0, 4, 0,	1, 3, 0,	0, 0, 8,
		0, 0, 0,	0, 0, 5,	6, 0, 0,
		0, 0, 0,	0, 0, 0,	0, 7, 3,
		
		0, 0, 0,	6, 0, 1,	0, 0, 0,
		8, 0, 0,	0, 0, 0,	0, 0, 4,
		0, 0, 0,	5, 0, 2,	0, 0, 0,
		
		7, 1, 0, 	0, 0, 0,	0, 0, 0,
		0, 0, 8,	4, 0, 0,	0, 0, 0,
		2, 0, 0,	0, 9, 6,	0, 5, 0

	];
}

pub mod intermediate_boards{
	
}
