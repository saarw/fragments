use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::borrow::Cow;

fn parse_type(iter: Iterator<Item=&str>) -> (&str, usize) {
	let mut parsed_type = "";
		
	match iter.next().expect("Could not parse type information.") {
		"int" => parsed_type = "c_int",
		"char" => parsed_type = "c_char",
		"short" => parsed_type = "c_short",
		"ushort" => parsed_type = "c_ushort",
		"float" => parsed_type = "c_float",
		"double" => parsed_type = "c_double",
		"long" => { parsed_type = "c_long"; if iter.next() != "long" { iter.prev(); }; }, 
		"unsigned" => {
			match iter.next() {
				"int" => parsed_type = "c_uint",
				"char" => parsed_type = "c_uchar",
				"short" => parsed_type = "c_ushort",
				"ushort" => parsed_type = "c_ushort",
				"float" => parsed_type = "c_ufloat",
				"double" => parsed_type = "c_udouble",
				"long" => { parsed_type = "c_ulong"; if iter.next() != "long" { iter.prev(); }; },	
				_ => { parsed_type = "c_uint"; iter.next() } 			
			};
			iter.prev();
		},
		"void" => parsed_type = "c_void",
		_ => parsed_type = iter.prev(),
	}
	
	
	let mut token_count = 1;
	
	(parsed_type, token_count)
}

fn extract_name_and_array_pointer_type<'a>(base_type: &'a str, name: &'a str) -> (Cow<'a, str>,Cow<'a, str>) {
	let mut full_type: Cow<str>;	
	let mut name_buf: &str;
	let mut var_name: Cow<str>;	
	// Find out if the variable is a pointer
	if let Some(ptr_start) = name.find("*") {
		let mut buf = String::new();
		buf.push_str("*");
		buf.push_str(base_type);
		full_type = Cow::Owned(buf);
		if let Some(name_terminator) = name[ptr_start..].find(|c: char| c.is_whitespace() || c == ')') {
			name_buf = &name[(ptr_start + 1)..(name_terminator - 1)];
		} else {
			name_buf = &name[(ptr_start + 1)..];
		}
	} else {
		full_type = base_type.into();		
		name_buf = name;
	}	
	// Convert arrays of x[2][3] into [[type of x; 3]; 2]
	let (start_brackets, end_brackets): (Vec<(usize, char)>, Vec<(usize, char)>) = name_buf.char_indices().
		filter(|&(_, c)| c == '[' || c == ']').partition(|&(_, c)| c == '['); 
	let array_sizes: Vec<i32> = start_brackets.into_iter().zip(end_brackets).
		map(|((start_idx, _), (end_idx, _))| name_buf[(start_idx + 1)..end_idx].parse::<i32>().
				unwrap_or_else(|e| panic!("Failed to parse array size from \"{}\" : {}",name_buf, e))).
		collect();
	
	if array_sizes.len() > 0 {
		let mut array_type = (0..array_sizes.len()).map(|_| "[").collect::<String>();
				
		array_type.push_str(full_type.as_ref());
		for s in array_sizes.iter().rev() {
			array_type.push_str(&format!("; {}]", s));
		}
		full_type = Cow::Owned(array_type);	
		var_name = Cow::Owned(name_buf[..name_buf.find(|c: char| c == '[' || c == ')' || c.is_whitespace()).unwrap()].to_string());
	} else {
		var_name = name_buf.into();
	}
	(full_type, var_name)
}

#[test]
fn test_extract_name_and_array_pointer_type() {	
	assert_eq!(extract_name_and_array_pointer_type("int", "hej").0, "int");
	assert_eq!(extract_name_and_array_pointer_type("int", "hej").1, "hej");
	
	let (rust_type, name) = extract_name_and_array_pointer_type("int", "*hej[12][23]");
	assert_eq!(rust_type, "[[*int; 23]; 12]");
	assert_eq!(name, "hej"); 	
}

fn parse_fields(line: &str) -> (Vec<(Cow<str>, Cow<str>)>) {
	let mut first_3_words: Vec<&str> = line.split_whitespace().take(3).collect();
	let padding = (first_3_words.len()..3).map(|_| "");
	first_3_words.extend(padding);	
	let (parsed_type, token_skip) = parse_type(first_3_words); 	
	let rust_vars = line.split(",").skip(token_skip).map(|name| extract_name_and_array_pointer_type(parsed_type, name)).collect();	
	rust_vars
}

#[test]
fn test_parse_fields() {
	println!("return {:?}", parse_fields("int i"));	
	assert_eq!(parse_fields("int i")[0].0, "u32");
}


fn main() {
	let filename = ""; 
    let mut infile = File::open(filename).ok().expect("Failed to open file");
    let mut buf = String::new();
    infile.read_to_string(&mut buf).ok().expect("Failed to read file");
    let mut comment_free_buf = String::new();
    let mut pos = 0;
    // Remove comments
    while let Some(comment_pos) = buf[pos..].find('/') {
    	match buf[pos..].as_bytes()[comment_pos] {
    		b'/' => {
    				comment_free_buf.push_str(&buf[pos..(comment_pos - 1)]);
    				if let Some(endline) = buf[comment_pos..].find('\n') {
    					pos += comment_pos + endline;
    				} else {
    					break; // End file
					}
    				},
    		b'*' => {
    				comment_free_buf.push_str(&buf[pos..(comment_pos - 1)]);
    				if let Some(end_comment) = buf[(comment_pos + 2)..].find("*/") {
    						pos += comment_pos + 2 + end_comment;
    					} else {
    						break;
						}
    				},
    		_ => {},
		}
	}      
}
