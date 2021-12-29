#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! # notation_tab = "0.3.0"
//! notation_tab = { version = "0.3.0", path = "../../../../../../win/projects/notation-fun/notation/crates/notation_tab" }
//! ```

use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

pub fn new_tab() -> Tab {
    tab! {
        "9622d99c-42c6-4f5a-8bc3-d2e862d44181"
        Meta: TabMeta::new(Key::A, Scale::Minor, Signature::_4_4, Tempo::Bpm(64))
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 3 5 )
                "2-7" Chord ( 2: 3- 5 7- )
                "3.7" Chord ( 3: 3 5 7- )
                "4" Chord ( 4: 3 5 )
                "5" Chord ( 5: 3 5 )
                "5.7" Chord ( 5: 3 5 7- )
                "6-" Chord ( 6: 3- 5 )
                "6" Chord ( 6: 3 5 )
                "7" Chord ( 7: 3 5 7- )
                $duration = _1_2
                "5,4" Chord [ ( 5: 3 5 ) ( 4: 3 5 ) ] |
            ]}
            {guitar Guitar [
                Fretboard
                $duration = _1
                "Am" Shape ( 0 0 2 2 1 0 )
                "F" Shape ( 0 2 2 1 0 0 ) +1
                "Am;5" Shape ( 0 2 2 0 0 0 ) +5
                "Dm7" Shape ( _ _ 0 2 1 1 )
                "G7" Shape ( 3 2 0 0 0 1 )
                "C" Shape ( 0 3 2 0 1 0 )
                "B7" Shape ( _ 2 1 2 0 2 )
                "E7" Shape ( 0 2 2 1 3 0 )
                $duration = _1_2
                "G,F"
                "G;1/2" Shape ( 0 2 2 1 0 0 ) +3
                "F;1/2" Shape ( 0 2 2 1 0 0 ) +1 |
                $duration = _1_8
                "intro:1" Pick [ (5 4 3 2 1) 3 2 3 5 1@8 1@7 1@5 ] |
                "intro:2" Pick [ (6 1) 3 2 3 (6 1) 3 2 3 ] |
                "intro:3" Pick [ 4@0 1@10 1@10 1@10 1@10 1@8 1@7 1@8 ] |
                "intro:4" Pick [ 1 3 2 3 (5 4 3 2 1) 3 2 3 ] |
                "5-1" Pick [ 5 4 2 3 1 3, 2, 4, 1, 3, 2, ] |
                "4-1" Pick [ 4 3 2 3 1 3 4, 1, 3, 2, ] |
                "6-1" Pick [ 6 4 2 3 1 3, 2, 4, 1, 3, 2, ] |
                "5-2" Pick [ 5 4 1 2, 3, 4 3 1 2, 3, ] |
                "4-2" Pick [ 4 3 2 3 1 3, 2, 4, 1, 3, 2, ] |
                "6-2" Pick [ 6 4 2 3 1 2 2@0 3 ] |
            ]}
        ]
        Sections: [
            intro Intro [
                {
                    chord [ "6-" 1]
                    guitar [ "Am" 1 ; "intro:1" | ]
                }
                {
                    chord [ "5,4" | ]
                    guitar [ "G,F" | ; "intro:2" | ]
                }
                {
                    chord [ "2-7" 1 ]
                    guitar [ "Dm7" 1 ; "intro:3" | ]
                }
                {
                    chord [ "6-" 1]
                    guitar [ "Am;5" 1 ; "intro:4" | ]
                }
            ]
            verse Verse [
                {
                    chord [ "6-" 1]
                    guitar [ "Am" 1 ; "5-1" | ]
                }
                {
                    chord [ "2-7" 1]
                    guitar [ "Dm7" 1 ; "4-1" | ]
                }
                {
                    chord [ "5.7" 1]
                    guitar [ "G7" 1 ; "6-1" | ]
                }
                {
                    chord [ "1" 1]
                    guitar [ "C" 1 ; "5-2" | ]
                }
                {
                    chord [ "4" 1]
                    guitar [ "F" 1 ; "4-2" | ]
                }
                {
                    chord [ "6-" 1]
                    guitar [ "Am" 1 ; "5-1" | ]
                }
                {
                    chord [ "7" 1]
                    guitar [ "B7" 1 ; "5-1" | ]
                }
                {
                    chord [ "3.7" 1 @ 1 ; "6-" 1 @ 2 ]
                    guitar [ "E7" 1 @ 1 ; "6-2" | @ 1; "Am" 1 @ 2 ; "5-1" | @ 2 ]
                }
            ]
            chorus Chorus [
                {
                    chord [ "6-" 1 ]
                    guitar [ "Am" 1 ]
                }
                {
                    chord [ "2-7" 1]
                    guitar [ "Dm7" 1 ]
                }
                {
                    chord [ "5.7" 1]
                    guitar [ "G7" 1 ]
                }
                {
                    chord [ "1" 1]
                    guitar [ "C" 1 ]
                }
                {
                    chord [ "6-" 1 ]
                    guitar [ "Am" 1 ]
                }
                {
                    chord [ "4" 1]
                    guitar [ "F" 1 ]
                }
                {
                    chord [ "3.7" 1]
                    guitar [ "E7" 1 ]
                }
                {
                    chord [ "6-" 1 ]
                    guitar [ "Am" 1 ]
                }
            ]
            outro Outro [
                {
                    chord [ "6-" 1]
                    guitar [ "Am;5" 1 ]
                }
            ]
        ]
        Form: intro verse verse chorus outro
    }
}
