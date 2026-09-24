//! A crate to parse files using atoms.
//!
//! _Atoms_ are a way for a file to store data.
//! A file like this can be represented as a
//! (real-life) folder with pieces of paper.
//! Each paper has a title (a name) and contents
//! (a payload). Say there are these papers
//! in the folder:
//! ```text
//! John Doe
//! foo bar
//! Ryan Smith
//! ```
//! You can find John Doe's paper in the folder,
//! no matter the order or what other papers there
//! are.
//! An example of a file format that works like this is MP4. Note that
//! this parser is not recommended for parsing mp4.
//!
//! # Example
//!
//! ```ignore
//! use std::fs::File;
//! use std::io::Read;
//! use atomic_parser::Atom;
//!
//! let mut file = File::open("input.hex")?;
//! let mut value = vec![];    
//! file.read_to_end(&mut value)?;
//! println!("{:?}", Atom::parse(&value, &[])?);
//! Ok(())
//! ```

#[derive(PartialEq, Debug)]
/// A parsed atom.
///
/// Calling `Atom::parse` will generate a vector of these
/// atoms, consisting of a name and a payload (the contents of the atom)
pub struct Atom {
    pub name: [u8; 4],
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq)]
/// An error while parsing the atoms.
pub struct AtomError {
    /// At what index the error began (starts at 0)
    pub idx: usize,
    /// What kind of error happend
    pub kind: AtomErrorKind
}

#[derive(Debug, PartialEq)]
pub enum AtomErrorKind {
    /// The file ends before the atom could end,
    /// usually caused by size headers being wrong.
    MalformedAtom,
}

impl Atom {
    /// Parse an input into atoms.
    ///
    /// Uses the syntax
    /// ```text
    /// 4 bytes size 
    /// 4 bytes name
    /// size bytes payload
    /// ```
    /// Note the parser is _big-endian_. That means for an atom
    /// of 4 bytes, you write `0 0 0 4`, not `4 0 0 0`.
    ///
    /// This returns either a vector of atoms or an error if the parse
    /// is unsuccesful.
    ///
    /// The atom payload is raw bytes. For nested atoms, a call to
    /// `Atom::parse` is needed with the atom's payload.
    ///
    /// Each atom payload can only be 4 GB large (minus 1 byte), as the payload size
    /// is stored as a 32-bit unsigned integer.
    pub fn parse(input: &[u8], skips: &[[u8; 4]]) -> Result<Vec<Self>, AtomError> {
	
	let mut cursor = 0;
	let mut atoms = vec![];


	//let mut i = 0; // For debugging
	loop {
	    //i += 1; // For debugging
	    
	    if input.is_empty() {
			return Ok(vec![]);
	    }

	    if input.len() - cursor == 0 {
			break;
	    }
	    
	    if input.len() - cursor < 8 {
			return Err(AtomError {
				idx: cursor,
				kind: AtomErrorKind::MalformedAtom,
			})
	    }
	    let size = u32::from_be_bytes(input[cursor..cursor + 4].try_into().unwrap());
	    let atype: [u8; 4] = input[cursor + 4..cursor + 8].try_into().unwrap();


	    if input.len() - cursor < 8 + size as usize {
			return Err(AtomError {
				idx: cursor,
				kind: AtomErrorKind::MalformedAtom,
			})
	    }

	    if size == 0 {
			atoms.push(Atom {
				name: atype,
				payload: vec![],
			});
			cursor += 8;
			continue;
	    }
	    
	    let contents = &input[cursor + 8..cursor + 8 + size as usize];

	    // ignore skip atoms.
	    if skips.contains(&atype) {
			cursor += (8 + size) as usize;
			continue;
	    }

	    atoms.push(Atom {
			name: atype,
			payload: contents.to_vec(),
	    });
	    
	    if input.len() - (cursor + size as usize + 8) == 0 {
			break;
	    }
	    
	    cursor += (8 + size) as usize;
	}

	
	
	Ok(atoms)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_atom() {
		let input = &[
			0x00, 0x00, 0x00, 0x04,
			b'a', b't', b'o', b'm',
			0x01, 0x02, 0x03, 0x04,
		];

		let expected = vec![
			Atom {
				name: [b'a', b't', b'o', b'm'],
				payload: vec![
					0x01, 0x02, 0x03, 0x04,
				],
			},
		];
	
		assert_eq!(expected, Atom::parse(input, &[[b's', b'k', b'i', b'p']]).unwrap());
    }
    
    
    #[test]
    fn few_atoms() {
		let input = &[
			0x00, 0x00, 0x00, 0x04,
			b'a', b't', b'0', b'1',
			0x01, 0x02, 0x03, 0x04,
	    
			0x00, 0x00, 0x00, 0x08,
			b'a', b't', b'0', b'2',
			0x11, 0x12, 0x13, 0x14,
			0x15, 0x16, 0x17, 0x18,
	    
			0x00, 0x00, 0x00, 0x0C,
			b'a', b't', b'0', b'3',
			0x21, 0x22, 0x23, 0x24,
			0x25, 0x26, 0x27, 0x28,
			0x29, 0x2A, 0x2B, 0x2C,
		];

		let expected = vec![
			Atom {
				name: [b'a', b't', b'0', b'1'],
				payload: vec![
					0x01, 0x02, 0x03, 0x04,
				],
			},
			Atom {
				name: [b'a', b't', b'0', b'2'],
				payload: vec![
					0x11, 0x12, 0x13, 0x14,
					0x15, 0x16, 0x17, 0x18,
				],
			},
			Atom {
				name: [b'a', b't', b'0', b'3'],
				payload: vec![
					0x21, 0x22, 0x23, 0x24,
					0x25, 0x26, 0x27, 0x28,
					0x29, 0x2A, 0x2B, 0x2C,
				],
			},
		];
	
		assert_eq!(expected, Atom::parse(input, &[[b's', b'k', b'i', b'p']]).unwrap());
    }

    #[test]
    fn no_atoms() {
		let input = &[];
		let expected: Vec<Atom> = vec![];
		assert_eq!(expected, Atom::parse(input, &[[b's', b'k', b'i', b'p']]).unwrap());
    }
    
    #[test]
    fn skip_test() {
		let input = &[
			0x00, 0x00, 0x00, 0x04,
			b'a', b't', b'0', b'1',
			0x01, 0x02, 0x03, 0x04,
	    
			0x00, 0x00, 0x00, 0x08,
			b'a', b't', b'0', b'2',
			0x11, 0x12, 0x13, 0x14,
			0x15, 0x16, 0x17, 0x18,
	    
			0x00, 0x00, 0x00, 0x0C,
			b'a', b't', b'0', b'3',
			0x21, 0x22, 0x23, 0x24,
			0x25, 0x26, 0x27, 0x28,
			0x29, 0x2A, 0x2B, 0x2C,

			0x00, 0x00, 0x00, 0x06,
			b's', b'k', b'i', b'p',
			0x0A, 0x0B, 0x0C, 0x0D,
			0x0E, 0x0F
		];

		let expected = vec![
			Atom {
				name: [b'a', b't', b'0', b'1'],
				payload: vec![
					0x01, 0x02, 0x03, 0x04,
				],
			},
			Atom {
				name: [b'a', b't', b'0', b'2'],
				payload: vec![
					0x11, 0x12, 0x13, 0x14,
					0x15, 0x16, 0x17, 0x18,
				],
			},
			Atom {
				name: [b'a', b't', b'0', b'3'],
				payload: vec![
					0x21, 0x22, 0x23, 0x24,
					0x25, 0x26, 0x27, 0x28,
					0x29, 0x2A, 0x2B, 0x2C,
				],
			},
		];

		assert_eq!(expected, Atom::parse(input, &[[b's', b'k', b'i', b'p']]).unwrap());
    }

    #[test]
    fn malformed_atom() {
		let input = &[
			0x00, 0x00, 0x03,
			b'a', b't', b'o', b'm',
			0x01, 0x02, 0x03
		];

		assert!(Atom::parse(input, &[[b's', b'k', b'i', b'p']]).is_err());
    }
    
    #[test]
    fn one_skip() {
		let input = &[
			0x00, 0x00, 0x00, 0x04,
			b's', b'k', b'i', b'p',
			0x01, 0x02, 0x03, 0x04,
		];

		let expected: Vec<Atom> = vec![];

		assert_eq!(expected, Atom::parse(input, &[[b's', b'k', b'i', b'p']]).unwrap());
    }

    #[test]
    fn no_payload() {
		let input = &[
			0x00, 0x00, 0x00, 0x00,
			b'a', b't', b'o', b'm',
		];

		let expected = vec![
			Atom {
				name: [b'a', b't', b'o', b'm'],
				payload: vec![],
			}
		];

		assert_eq!(expected, Atom::parse(input, &[[b's', b'k', b'i', b'p']]).unwrap());
    }
    
    #[test]
    fn few_skips() {
		let input = &[
			0x00, 0x00, 0x00, 0x04,
			b's', b'k', b'i', b'p',
			0x01, 0x02, 0x03, 0x04,
	    
			0x00, 0x00, 0x00, 0x08,
			b's', b'k', b'i', b'p',
			0x11, 0x12, 0x13, 0x14,
			0x15, 0x16, 0x17, 0x18,
	    
			0x00, 0x00, 0x00, 0x0C,
			b's', b'k', b'i', b'p',
			0x21, 0x22, 0x23, 0x24,
			0x25, 0x26, 0x27, 0x28,
			0x29, 0x2A, 0x2B, 0x2C,
		];

		let expected: Vec<Atom> = vec![];

		assert_eq!(expected, Atom::parse(input, &[[b's', b'k', b'i', b'p']]).unwrap());
    }
    
    #[test]
    fn combined_test() {	
		let input = &[
			0x00, 0x00, 0x00, 0x04,
			b'a', b't', b'o', b'm',
			0x01, 0x02, 0x03, 0x04,
	    
			0x00, 0x00, 0x00, 0x00,
			b'0', b'p', b'a', b'y',
	    

			0x00, 0x00, 0x00, 0x06,
			b's', b'k', b'i', b'p',
			0x0A, 0x0B, 0x0C, 0x0D,
			0x0E, 0x0F,

			0x00, 0x00, 0x00, 0x06,
			b'm', b'e', b't', b'a',
			0x0A, 0x0B, 0x0C, 0x0D,
			0x0E, 0x0F
		];

	
		let expected = vec![
			Atom {
			name: [b'a', b't', b'o', b'm'],
			payload: vec![
				0x01, 0x02, 0x03, 0x04,
			],
			},
			Atom {
				name: [b'0', b'p', b'a', b'y'],
				payload: vec![],
			},
		];

	
		assert_eq!(expected, Atom::parse(
			input, &[[b's', b'k', b'i', b'p'],  [b'm', b'e', b't', b'a']]
		).unwrap());


    }

    #[test]
    fn garbage_test() {
		let input = &[
			0x00, 0x00, 0x00, 0x04,
			b'a', b't', b'o', b'm',
			0x01, 0x02, 0x03, 0x04,
			0x0A
		];
	
		assert!(Atom::parse(input, &[[b's', b'k', b'i', b'p']]).is_err());
    }

    #[test]
    fn many_skip_types() {
		let input = &[
			0x00, 0x00, 0x00, 0x04,
			b'm', b'e', b't', b'a',
			0x01, 0x02, 0x03, 0x04,

			0x00, 0x00, 0x00, 0x04,
			b's', b'k', b'i', b'p',
			0x01, 0x02, 0x03, 0x04,

			0x00, 0x00, 0x00, 0x04,
			b'a', b't', b'o', b'm',
			0x0A, 0x0B, 0x0C, 0x0D,
		];

		let expected = vec![
			Atom {
				name: [b'a', b't', b'o', b'm'],
				payload: vec![
					0x0A, 0x0B, 0x0C, 0x0D,
				],
			},
		];

		assert_eq!(expected, Atom::parse(
			input, &[[b's', b'k', b'i', b'p'], [b'm', b'e', b't', b'a']]
		).unwrap());
    }

    #[test]
    fn no_skip_provided() {
		let input = &[
			0x00, 0x00, 0x00, 0x04,
			b's', b'k', b'i', b'p',
			0x01, 0x02, 0x03, 0x04,

			0x00, 0x00, 0x00, 0x04,
			b'a', b't', b'o', b'm',
			0x0A, 0x0B, 0x0C, 0x0D,
		];

		let expected = vec![
			Atom {
				name: [b's', b'k', b'i', b'p'],
				payload: vec![
					0x01, 0x02, 0x03, 0x04,
				],
			},
			Atom {
				name: [b'a', b't', b'o', b'm'],
				payload: vec![
					0x0A, 0x0B, 0x0C, 0x0D,
				],
			},
		];

		assert_eq!(expected, Atom::parse(input, &[]).unwrap());
    }
}
