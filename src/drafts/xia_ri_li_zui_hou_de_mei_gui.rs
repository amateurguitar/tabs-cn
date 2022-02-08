use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

pub fn new_tab() -> Tab {
    tab! {
        "20e17d60-3498-4e78-8982-2d3770eadbb4"
        Meta: G Major 3 _4 80
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 1 )
            ]}
            {vocal Vocal [
                $octave = P3
                $duration = _1_8
                "i" Tone [ _** 1+ 2, ] |
                "a1" Tone [ 3* ^1+ 7, 6+ 5, ] |
                "a2" Tone [ 5 3*+ 1+ 2, ] |
                "a3" Tone [ 3 5* 3 2+ 1, ] |
                "a4_1" Tone [ 1** 1+ 2, ] |
                "a4_2" Tone [ 1** 5+ 3, ] |
                "a4_3" Tone [ 1**+ ] |
                "b1" Tone [ ^1*+ 7 6+ 5, ] |
                "b2" Tone [ 5* 3* 5+ 3, ] |
                "b3" Tone [ ^1*+ 7 6 5# ] |
                "b4" Tone [ 6+ 7, ^1** ] |
                "b4_" Tone [ @ ^1** 1+ 2, ] |
            ]}
        ]
        Sections: [
            {i Intro [
                {
                    chord [ "1" 1 ]
                    vocal [ "i" | ]
                }
            ]}
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
                    vocal [ "a4_1" | @ 1 ; "a4_2" | @ 2 ; "a4_3" | @ 3 ]
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
                    vocal [ "b4_" | ]
                }
            ]}
        ]
        Form: i a a b a
    }
}

