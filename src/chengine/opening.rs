use crate::chengine::*;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Opening {
    pub moves: Vec<(Square, Square)>,
    pub next: Vec<Opening>,
}

macro_rules! opening {
    ( $($from: expr => $to: expr),+; $next: expr) => {
        Opening {
            moves: vec![
                $( (Square::new($from).unwrap(), Square::new($to).unwrap()), )+
            ],
            next: $next
        }
    };
}

lazy_static! {
    pub static ref OPENING_BOOK: Opening = {
        let finish = opening! {
            "f3" => "e5",
            "f6" => "e4",
            "d1" => "f3";
            vec![]
        };
        opening! {
            "e2" => "e4";
            vec![
                opening! { //queen gambit counter
                    "d7" => "d5",
                    "e4" => "d5",
                    "d8" => "d5",
                    "b1" => "c3";
                    vec![]
                },
                opening! { //muffin gambit
                    "e7" => "e5",
                    "g1" => "f3";
                    vec![
                        opening! {
                            "g8" => "f6";
                            vec![finish.clone()]
                        },
                        opening! {
                            "b8" => "c6",
                            "f1" => "b5",
                            "g8" => "f6",
                            "b5" => "c6";
                            vec![
                                opening! {
                                    "d7" => "c6";
                                    vec![finish.clone()]
                                },
                                opening! {
                                    "b7" => "c6";
                                    vec![finish.clone()]
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    };
}
