use std::io::{self, Write};

fn main() {

    let base_price = loop {
	print!("Enter the current bill total:  ");
	io::stdout().flush().unwrap();

	let mut get_base = String::new();
	io::stdin()
	    .read_line(&mut get_base)
	    .expect("failed to read input");

	let get_base: f32 = match get_base.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	break get_base;
    };

    let perc_tip = loop {
	print!("Enter percentage tip to add: ");
	io::stdout().flush().unwrap();

	let mut get_perc = String::new();
	io::stdin()
	    .read_line(&mut get_perc)
	    .expect("failed to read input");

	let get_perc: f32 = match get_perc.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
	break get_perc;
    };

    println!("You entered {base_price} and wish a tip of approx +{perc_tip}%");
    println!("Some possible tips are:");

    let v: Vec<f32> = nearby_palindromes(base_price, perc_tip);
    for (_pos, e) in v.iter().enumerate() {
	println!("${:02}", e);
    }
}

fn nearby_palindromes(base: f32, perc: f32) -> Vec<f32> {

    let mut results = Vec::new();
    let mut start = base + (base * perc/100.0);

    // deal with rounding and bracket search within 5% around desired tip level
    start = (start * 100.0).round() / 100.0;
    let bottom = (start * 0.95 * 100.0).round() / 100.0;
    let top    = (start * 1.05 * 100.0).round() / 100.0;

    let mut cur = bottom;
    while cur < top {
	cur = (cur * 100.0).round() / 100.0;
	let s = format!("{:.2}", cur);
	let last_digit = s.chars().rev().nth(0).unwrap();
	if last_digit != '0' {
	    if is_palindrome(&cur) {
		results.push(cur);
	    }
	}
	cur = cur + 0.01;
    }
    results
}

fn is_palindrome(num: &f32) -> bool {
    let sf: String = num.to_string();

    let mut v = sf.into_bytes();
    v.retain(|x| *x != 0b101110); // remove decimal point before comparison
    v.iter().eq(v.iter().rev())
}
