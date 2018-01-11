
// fn do_the_thing(data: &Vec<i32>) -> Context {
// 	let mut context = Context {
// 		thing: None,
// 		children: None,
// 	};

// 	let data_iter = data.drain();

// 	while let item = data_iter.next() {

// 	}

// 	context
// }

fn mutablething() {
	let mut data: Vec<i32> = vec![
		1, 2, 3,
		10,
		4, 5,
		10,
		6, 7, 8,
	];

	let rest: Vec<i32> = data.drain(1..).collect();

	println!("Rest: {:?}", rest);
}

#[derive(Debug)]
struct Context {
	things: Vec<i32>,
	children: Option<Box<Context>>,
}

fn secondmutablething(mut thing: &mut Vec<i32>) -> Context {
	println!("\n\nData: ({}) {:?}", thing.len(), thing);
	let mut context = Context { things: Vec::new(), children: None };

	// let mut thing: Vec<i32> = data.clone();

	while let Some(item) = thing.pop() {
		// println!("{}, remaining {}", item, thing.len());
		if item == 10 {
			context.children = Some(Box::new(secondmutablething(&mut thing)));
		} else {
			context.things.push(item);

		}
	}

	context
}

fn guard(data: &Vec<i32>) -> Context {
	let mut reversed: Vec<i32> = data.iter().cloned().rev().collect();

	secondmutablething(&mut reversed)
}

fn immutablething(data: &Vec<i32>) -> (Context, usize) {
	println!("\n\nData: ({}) {:?}", data.len(), data);

	let mut context = Context { things: Vec::new(), children: None };
	let mut skip: usize = 0;
	let data_iter = data.iter();

	for (i, item) in data_iter.enumerate() {
		if *item == 10 {
			let (children, skip) = immutablething(&data[i+1..].to_vec());

			context.children = Some(Box::new(children));
		} else {
			context.things.push(*item);

			skip += 1;
		}
	}

	(context, skip)
}

fn main() {
	let data: Vec<i32> = vec![
		1, 2, 3,
		10,
		4, 5,
		10,
		6, 7, 8,
	];

	let out = immutablething(&data);

	println!("{:?}", out);
}
