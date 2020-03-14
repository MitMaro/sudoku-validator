use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;

fn is_valid(input: &[i32; 9]) -> bool {
	let mut result: i32 = 0;

	for i in input {
		let mask = 1 << i;

		if result & mask == mask {
			return false;
		}

		result |= mask;
	}

	true
}

fn main() {
	let stdin = io::stdin();

	let mut rows = [[0 as i32; 9]; 9];
	let mut columns = [[0 as i32; 9]; 9];
	let mut regions = [[0 as i32; 9]; 9];

	for (row, line) in stdin.lock().lines().enumerate() {
		for (col, value) in line.unwrap().split(' ').enumerate() {
			let value = value.parse::<i32>().unwrap();
			rows[row][col] = value;
			columns[col][row] = value;
			let cell_index = (col % 3) + (row % 3) * 3;
			let region_index = (row / 3) * 3 + col / 3;
			regions[region_index][cell_index] = value;
		}
	}

	let (sender, receiver) = mpsc::channel();

	let mut handlers = (0..rows.len())
		.map(|row_index| {
			let row = rows[row_index];
			let sender = sender.clone();
			thread::spawn(move || {
				if !is_valid(&row) {
					sender.send(("row", row_index)).unwrap();
				}
			})
		})
		.collect::<Vec<thread::JoinHandle<_>>>();

	handlers.append(
		(0..columns.len())
			.map(|column_index| {
				let column = columns[column_index];
				let sender = sender.clone();
				thread::spawn(move || {
					if !is_valid(&column) {
						sender.send(("column", column_index)).unwrap();
					}
				})
			})
			.collect::<Vec<thread::JoinHandle<_>>>()
			.as_mut(),
	);
	handlers.append(
		(0..regions.len())
			.map(|region_index| {
				let region = regions[region_index];
				let sender = sender.clone();
				thread::spawn(move || {
					if !is_valid(&region) {
						sender.send(("region", region_index)).unwrap();
					}
				})
			})
			.collect::<Vec<thread::JoinHandle<_>>>()
			.as_mut(),
	);

	for handle in handlers {
		handle.join().unwrap();
	}

	drop(sender);

	let handler = thread::spawn(move || {
		let mut errors_found = 0;
		// Perform the transactions in order
		receiver.into_iter().for_each(|(t, i)| {
			errors_found += 1;
			println!("{} {} is invalid", t, i);
		});

		if errors_found > 0 {
			println!("Errors found: {}", errors_found)
		}
		else {
			println!("Puzzle is valid!")
		}
	});

	handler.join().unwrap();
}
