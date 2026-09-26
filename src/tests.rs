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

#[test]
fn atom_into_bytes() {
    let input = Atom {
        name: [b'a', b't', b'0', b'1'],
        payload: vec![
            0x01, 0x02, 0x03, 0x04,
        ],
    };
    

    let expected = vec![
	0x00, 0x00, 0x00, 0x04,
        b'a', b't', b'0', b'1',
        0x01, 0x02, 0x03, 0x04,
    ];

    assert_eq!(expected, input.into_bytes());
}

#[test]
fn one_atom_into_byte() {
    let input = vec![Atom {
        name: [b'a', b't', b'0', b'1'],
        payload: vec![
            0x01, 0x02, 0x03, 0x04,
        ],
    }];
    

    let expected = vec![
	0x00, 0x00, 0x00, 0x04,
        b'a', b't', b'0', b'1',
        0x01, 0x02, 0x03, 0x04,
    ];

    assert_eq!(expected, atoms_into_bytes(input));
}

#[test]
fn few_atom_into_byte() {
    let input = vec![
	Atom {
            name: [b'a', b't', b'0', b'1'],
            payload: vec![
		0x01, 0x02, 0x03, 0x04,
            ],
	},
	Atom {
            name: [b'a', b't', b'0', b'2'],
            payload: vec![
		0x01, 0x02, 0x03, 0x04,
            ],
	},
	Atom {
            name: [b'a', b't', b'0', b'3'],
            payload: vec![
		0x01, 0x02, 0x03, 0x04,
            ],
	},
	Atom {
            name: [b'a', b't', b'0', b'4'],
            payload: vec![
		0x01, 0x02, 0x03, 0x04,
            ],
	},
	Atom {
            name: [b'a', b't', b'0', b'5'],
            payload: vec![
		0x01, 0x02, 0x03, 0x04,
            ],
	}
    ];
    

    let expected = vec![
	0x00, 0x00, 0x00, 0x04,
        b'a', b't', b'0', b'1',
        0x01, 0x02, 0x03, 0x04,

	0x00, 0x00, 0x00, 0x04,
        b'a', b't', b'0', b'2',
        0x01, 0x02, 0x03, 0x04,

	0x00, 0x00, 0x00, 0x04,
        b'a', b't', b'0', b'3',
        0x01, 0x02, 0x03, 0x04,

	0x00, 0x00, 0x00, 0x04,
        b'a', b't', b'0', b'4',
        0x01, 0x02, 0x03, 0x04,

	0x00, 0x00, 0x00, 0x04,
        b'a', b't', b'0', b'5',
        0x01, 0x02, 0x03, 0x04,
    ];

    assert_eq!(expected, atoms_into_bytes(input));
}

#[test]
fn from_str_test() {
    let expected = Atom {
	name: [b'a', b't', b'o', b'm'],
	payload: vec![
	    b'p', b'a', b'y', b'l', b'o', b'a', b'd',
	],
    };

    assert_eq!(Some(expected), Atom::from_str("atom", "payload"));
}
