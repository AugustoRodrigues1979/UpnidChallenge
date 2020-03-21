//module responsible for basic routines
use std::fmt::Write;

pub fn len_vec_str(input_vec : Vec<&str>) -> usize
{
	let mut out = String::new();
    
	for n in input_vec 
	{
		let _ = write!(&mut out, "{}", n);
	}

	return out.chars().count();
}

pub fn vec_str_to_string(input_vec : Vec<&str>) -> String
{
	let mut out = String::new();
    
	for n in input_vec 
	{
		let _ = write!(&mut out, "{}", n);
	}

	return out;
}
