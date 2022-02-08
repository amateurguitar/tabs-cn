use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

pub fn new_tab() -> Tab {
    tab! {
        "dbb0e1e4-b9ce-4b40-a6b2-641fc9f15940"
        Meta: G Major 4 _4 80
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 1 )
            ]}
            {vocal Vocal [
                $octave = P3
                $duration = _1_4
                "a1" Tone [ 5 3, 5, ^1* ] |
                "a2" Tone [ 6 ^1, 6, 5* ] |
                "a3" Tone [ 5 1, 2, 3 2, 1, ] |
                "a4" Tone [ 2*+ _ ] |
                "a5" Tone [ 5 3, 5, ^1+ 7, ] |
                "a6" Tone [ 6 ^1 5* ] |
                "a7" Tone [ 5 2, 3, 4+ .7, ] |
                "a8" Tone [ 1*+ _ ] |
                "b1" Tone [ 6 ^1 ^1* ] |
                "b2" Tone [ 7 6, 7, ^1* ] |
                $duration = _1_8
                "b3" Tone [ 6 7 ^1 6 6 5 3 1 ] |
                $duration = _1_4
                "b4" Tone [ 2*+ _ ] |

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
        ]
        Form: a b
    }
}
