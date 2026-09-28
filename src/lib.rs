//! A crate to parse files using atoms, or to create
//! atoms.
//!
//! # What is an atom?
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
//! In files, atoms are made up of 3 parts:
//! ```text
//! size
//! name
//! payload
//! ```
//! The name is the paper title, payload is
//! what is written on the paper. Size is the
//! size of the atom, so the parser knows where
//! each atom ends and a new atom begins.
//!
//! An example of a file format that works like this is MP4. Note that
//! this parser is not recommended for parsing mp4.
//!
//! # Example
//!
//! ## Parsing atoms
//!
//! Make sure you have the `read` feature enabled, though
//! it is enabled by default.
//!
//! Minimal example:
//! ```ignore
//! use std::fs::File;
//! use std::io::Read;
//! use atomic_parser::Atom;
//!
//! let mut file = File::open("input.hex").expect("failed to open file.");
//! let mut value = vec![];
//! file.read_to_end(&mut value).expect("failed to read file.");
//! println!("{:?}", Atom::parse(&value, &[]).expect("Atoms are incorrect"));
//! ```
//! Or, if we want to ignore `skip` atoms:
//! ```ignore
//! use std::fs::File;
//! use std::io::Read;
//! use atomic_parser::Atom;
//!
//! let mut file = File::open("input.hex").expect("failed to open file.");
//! let mut value = vec![];
//! file.read_to_end(&mut value).expect("failed to read file.");
//! println!("{:?}", Atom::parse(&value, &[[b's', b'k', b'i', b'p']]).expect("Atoms are incorrect"));
//! ```
//! We can also ignore multiple atoms, e.g. a `skip` and a `meta` atom:
//! ```ignore
//! use std::fs::File;
//! use std::io::Read;
//! use atomic_parser::Atom;
//!
//! let mut file = File::open("input.hex").expect("failed to open file.");
//! let mut value = vec![];
//! file.read_to_end(&mut value).expect("failed to read file.");
//! println!("{:?}", Atom::parse(&value, &[[b's', b'k', b'i', b'p'], [b'm', b'e', b't', b'a']]).expect("Atoms are incorrect"));
//! ```
//!
//! ## Building atoms
//!
//! Make sure you have the `write` feature enable.
//! Run:
//! ```bash
//! cargo add atomic_parser -F write
//! ```
//!
//! Minimal example:
//! ```
//! use atomic_parser::Atom;
//! use atomic_parser::atoms_into_bytes;
//!
//! let input = vec![
//!     Atom {
//!         name: [b'a', b't', b'0', b'1'],
//!         payload: vec![
//!             0x01, 0x02, 0x03, 0x04,
//!         ],
//!     },
//!     Atom {
//!         name: [b'a', b't', b'0', b'2'],
//!         payload: vec![
//!            0x01, 0x02, 0x03, 0x04,
//!         ],
//!     },
//!     Atom {
//!         name: [b'a', b't', b'0', b'3'],
//!         payload: vec![
//!             0x01, 0x02, 0x03, 0x04,
//!         ],
//!     },
//!     Atom {
//!         name: [b'a', b't', b'0', b'4'],
//!         payload: vec![
//!             0x01, 0x02, 0x03, 0x04,
//!         ],
//!     },
//!     Atom {
//!         name: [b'a', b't', b'0', b'5'],
//!         payload: vec![
//!             0x01, 0x02, 0x03, 0x04,
//!         ],
//!     }
//! ];
//!
//! let bytes = atoms_into_bytes(input);
//! println!("{:#?}", bytes);
//!
//! ```

use std::fmt;

#[derive(PartialEq, Debug, Clone, Hash)]
/// A parsed atom.
///
/// Calling `Atom::parse` will generate a vector of these
/// atoms, consisting of a name and a payload (the contents of the atom)
pub struct Atom {
    pub name: [u8; 4],
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
/// An error while parsing the atoms.
pub struct AtomError {
    /// At what index the error began (starts at 0)
    pub idx: usize,
    /// What kind of error happend
    pub kind: AtomErrorKind
}

#[derive(Debug, PartialEq, Clone)]
pub enum AtomErrorKind {
    /// The file ends before the atom could end,
    /// usually caused by size headers being wrong.
    MalformedAtom,
}

impl Atom {
    /// Creates new atom with name and payload.
    #[must_use]
    pub fn new(name: [u8; 4], payload: Vec<u8>) -> Atom {
        Atom {
            name,
            payload
        }
    }

    /// Creates a new empty atom.
    #[must_use]
    pub fn empty(name: [u8; 4]) -> Atom {
        Atom {
            name,
            payload: vec![],
        }
    }

    /// Creates an atom from a `&str`.
    ///
    /// The function returns a `None` if the
    /// name is a &str that is not exactly 4 bytes.
    #[must_use]
    pub fn from_str(name: &str, payload: &str) -> Option<Atom> {
        let name: [u8; 4] = name.as_bytes().try_into().ok()?;
        let payload = payload.as_bytes().to_vec();

        Some(Atom {
            name,
            payload
        })
    }

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
    ///
    /// The size field is the size of the _payload_, not the atom. This means, that,
    /// for an atom whose payload is size 8, the size would be written as 8,
    /// not 16 (size field + name + payload).
    ///
    /// `skips` are atoms that should be ignored by the parser and not be included
    /// inside the parsed list.
    #[cfg(feature = "read")]
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

    /// Converts the atom into a vector
    /// of bytes that the parser would
    /// parse into the same aton,
    ///
    /// *NOTE*: Available on crate feature
    /// `write` only!
    #[cfg(feature = "write")]
    pub fn into_bytes(&self) -> Vec<u8> {
        [
            &(self.payload.len() as u32).to_be_bytes(),
            &self.name,
            self.payload.as_slice(),
        ].concat()
    }
}


/// Turns a vector of atoms into a vector of bytes.
///
/// *NOTE*: Available on crate feature
/// `write` only!
#[cfg(feature = "write")]
pub fn atoms_into_bytes(items: Vec<Atom>) -> Vec<u8> {
    let mut output = vec![];
    for i in items {
        output = [output, i.into_bytes()].concat();
    }

    output
}

impl fmt::Display for AtomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error at {}: {} ", self.idx, self.kind)
    }
}

impl fmt::Display for AtomErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomErrorKind::MalformedAtom => write!(f, "Some atom's size header may be incorrect, as the input ends before the atom ends.")
        }
    }
}

#[cfg(test)]
mod tests;
