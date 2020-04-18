//module responsible for basic routines
use std::fmt::Write;
use std::ops::{Bound, RangeBounds};

pub fn len_vec_str(input_vec : Vec<&str>) -> usize
{
	let mut out = String::new();
    
	for n in input_vec 
	{
		let _ = write!(&mut out, "{}", n);
	}

	return out.chars().count();
}

pub fn vec_str_to_i32(input_vec : Vec<&str>) -> (bool, i32)
{
	let mut out = String::new();

	out.push_str(&vec_str_to_string(input_vec));

	match out.parse::<i32>() {
		Ok(val)    => return (true,val),
		Err(_i)     => return (false,0),
	}
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


pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

/*
pub fn get_substring_from_string(input_string : String, pos : i32, length : i32)
{
	let mut out = String::new();

	out.push_str(input_string);

	if pos >= out.chars().count() { return ""; }
	if pos < 0 

	match out[pos..length-pos] {
		Ok(val)    => return (true,val),
		Err(_i)    => return (false,0),
	}
}
*/
