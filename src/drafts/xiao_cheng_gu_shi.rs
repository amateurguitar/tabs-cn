use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

pub fn new_tab() -> Tab {
    tab! {
        "0a3a4d47-ce8c-4eec-b855-22079130f872"
        Meta: G Major 4 _4 100
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 1 )
            ]}
            {vocal Vocal [
                $octave = P3
                $duration = _1_8
                "a1" Tone [ 3*+ 5 6 5 6 ^1 ] |
                "a2" Tone [ 6 5*+ @ 5** ] |
                "a3" Tone [ 5*+ 6 3 2 3 5 ] |
                "a4" Tone [ 2*** ] |
                "a5" Tone [ 2*+ 3 5* 6 ^1 ] |
                "a6" Tone [ 6* 6 ^1 6* 5* ] |
                "a7" Tone [ 5* 5 2 3* 2* ] |
                "a8" Tone [ 1*** ] |
                "b1" Tone [ 1 .6 1 2 3** ] |
                "b2" Tone [ 2 3 .6 1 .5** ] |
                "b3" Tone [ .6 1 2 3 5 3 ^1 6 ] |
                "b4" Tone [ 5*** ] |
                "b5" Tone [ ^1* ^1 6 5* 5 6 ] |
                "b6" Tone [ 5* 5 3 2** ] |
                "b7" Tone [ 5* 5 6 3* 2* ] |
                "b8" Tone [ 1*** ] |
            ]}
        ]
        Sections: [
            {a Verse [
                {
                    chord [ "1" 1 ]
                    vocal [ "a1" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a2" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a3" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a4" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a5" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a6" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a7" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a8" | ]
                }
            ]}
            {b Verse [
                {
                    chord [ "1" 1 ]
                    vocal [ "b1" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "b2" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "b3" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "b4" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "b5" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "b6" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "b7" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "b8" | ]
                }
            ]}
        ]
        Form: a b
    }
}
