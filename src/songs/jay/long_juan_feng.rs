use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

//photo book 02 - page 68
pub fn new_tab() -> Tab {
    tab! {
        "ef6bb44b-17cf-47e6-a50e-0ab636868334"
        Meta: A Major 4 _4 72
        Tracks: [
            {chord Chord [
                $duration = _1
                "1" Chord ( 1: 3 5 )
                "1/4" Chord ( 1: 3 5 /4 )
                "1/5" Chord ( 1: 3 5 /5 )
                "1_9" Chord ( 1: 3 5 9 )
                "2-" Chord ( 2: 3- 5 7- )
                "3-" Chord ( 3: 3- 5 7- )
                "4" Chord ( 4: 3 5 7 )
                "5" Chord ( 5: 3 5 )
                "b6" Chord ( 6b: 3- 5% 7% )
                "6-" Chord ( 6: 3- 5 7- )
                $duration = _1_2
                "1,4"
                    Chord ( 1: 3 5 /4 )
                    Chord ( 4: 3 5 /2 )
            ]}
            {guitar Guitar [
                Fretboard capo: 2
                $duration = _1
                "G" Shape ( 3 2 0 0 0 3 )
                "Em7" Shape ( 0 2 0 0 0 0 )
                "G/C" Shape ( 0 3 0 0 0 0 )
                "G/D" Shape ( _ 0 0 0 0 0 )
                "Gadd9" Shape ( 3 2 0 2 0 _ )
                "Am7" Shape ( 0 0 2 0 1 0 )
                "D" Shape ( 0 0 0 2 3 2 )
                "Cmaj7" Shape ( 0 3 2 0 0 0 )
                "Bm7" Shape ( 2 2 4 2 3 2 )
                "bEdim7" Shape ( 0 0 1 2 1 2 )
                $duration = _1_8
                "i:1,2" Pick [ (6 3) 4 2 3 2@1 2, 3, 3@2, 2, 3@2 ] |
                "i:3" Pick [ (5 3) 4 2 3 2@1 2, 3, 3@2, 2, 3@2 ] |
                "i:4" Pick [ 3 4 2 3 2@1 2, 3, 3@2, 2, 3@2 ] |
                "v:1,2" Pick [ (6 3) 4 2 2, 3, (6 3) 4 2@3* ] |
                "v:3" Pick [ (5 3) 4 2 2, 3, (5 3) 4 2@1* ] |
                $duration = _1
                "v:4" Pick [ (4 3 2 1) ] |
                "o:4" Pick [ (6 5 4 3 2 1) ] |
                $duration = _1_8
                "p:1" Pick [ 5 4 2@3 3 2 3 2@3 3 ] |
                "p:2" Pick [ 5 4 2 3 3@4 3 2* ] |
                "p:3" Pick [ 5 4 2 4 (5 3) 4 2 4 ] |
                "p:4" Pick [ 4 3 2 1@3 1** ] |
                "o:3" Pick [ (5 3) 4 2 3 (4 3 2 1**) ] |
                $duration = _1_4
                "c:1,2" Pick [
                    (6 5 4 3 2 1)
                    (6 5 4 3 2 1)
                    (6 5 4 3 2 1)
                    (4 3 2 1 ,)
                    (5 4 3 ,)
                ] |
                $duration = _1_8
                "c:3" Pick [
                    (5 4 3) (5 4 3) (4 3 2 1) (4 3 2 1,) (1 2 3 4,)
                    (5 4 3) (5 4 3) (4 3 2 1) (4 3 2 1,) (1 2 3 4,)
                ] |
                "c:4" Pick [
                    (5 4 3) (5 4 3) (4 3 2 1) (4 3 2 1,) (1 2 3 4,)
                    (4 3 2 1,) (1 2 3 4,) (4 3 2 1) (4 3 2 1) (4 3 2 1)
                ] |
            ]}
            {lyrics Lyrics [
                $duration = _1_4
                "1:1" Word [ "???" "???" "???", "???", "???" ] |
                "1:2" Word [ "???" "???" "???", "???", "???" ] |
                "1:3" Word [ "???" "???" "???", "???", "???" ] |
                "1:4" Word [ "???" "???",+ "???",, "???", "???", "???" ] |
                "2:1" Word [ "???" "???" "???", "???", "???" ] |
                "2:2" Word [ "???" "???" "???", "???", "???" ] |
                "2:3" Word [ "???" "???" "???", "???", "???" ] |
                "2:4" Word [ "???" "???",+ "???",, "???", "???", "???", "???", ] |
                $duration = _1_8
                "p:1" Word [ "???" "???" "???" "???" "???" "???" "???" "???" ] |
                "p:2" Word [ "???" "???" "???" "???" "???" "???" "Ba-" "by"] |
                "p:3" Word [ _ "???" "???" "???" "???" "???", "???"+ "???", "???", @ ] |
                "p:4" Word [ @ "" "???" @ ""** "???"* ] |
                "c:1" $duration = T_1_8
                    Word [ "???" "???" "???" "???" "???" "???" "???" "???" "???" ]
                    $duration = _1_8
                    Word [ "???" "???" ] |
                "c:2" $duration = T_1_8
                    Word [ "???" "???" "???" "???" "???" "???" "???" "???" "???" ]
                    $duration = _1_8
                    Word [ "???" "???", "???", ] |
                "c:3" Word [ "???" "???" "???" "???", "???", "???" "???" "???" "???", "???", ] |
                "c:4" Word [ _ "???", "???", _ "???", "???", ]
                    $duration = _1_2
                    Word [ "???" ] |
                "c:1@2" $duration = T_1_8
                    Word [ "???" "???" "???" "???" "???" "???" "???" "???" "???" ]
                    $duration = _1_8
                    Word [ "???" "???" ] |
                "c:2@2" $duration = T_1_8
                    Word [ "???" "???" "???" "???" "???" "???" "???" "???" "???" ]
                    $duration = _1_8
                    Word [ "???" "???", "???", ] |
                "c:3@2" Word [ "???" "???" "???" "???", "???", "???" "???" "???" "???", "???", ] |
                "c:4@2" Word [ _ "???", "???", _ "???", "???", ]
                    $duration = _1_8
                    Word [ "???" "???" "???" "???" ] |
                "b:1" Word [ "???" "???" "???" "???" "???" "???", "???", "???", "???", "???" ] |
                "b:2" Word [ "???" "???" "???" "???" "???" "???", "???", "???", "???", "???" ] |
                "b:3" Word [ "???" "???" "???" "???" "???" "???", "???", "???", "???", "???" ] |
                "b:4" Word [ "???" "???" "???" "???" "???" "???", "???", "???", "???", "???" ] |
                "o:3" Word [ "???" "???" "???" "???" "???" "???" "???" "???" @ ] |
                "o:4" $duration = _1
                    Word [ @ "" ]
            ]}
            {vocal Vocal [
                $duration = _1_4
                "v:1,2" Tone [ .5 2 2, 1, 3 ] |
                "v:3" Tone [ .6 3 3, 2, 4 ] |
                "v:4_1" Tone [ .6b 3,+ 4,, 3, 2, 1, .6b,, .5,, ] |
                "v:4_2" Tone [ .6b 3,+ 4,, 3, 2, 1, 5,, 6,, ] |
                $duration = _1_8
                "p:1" Tone [ 6 3 3 1 1 .5 1 5 ] |
                "p:2" Tone [ 5 2 2 .7 .7 .5 .7 1 ] |
                "p:3" Tone [ _ .6 1 .6 1 .6, 1+ 2, 3, @ ] |
                "p:4" Tone [ @ 3 2 @ 2** 3 4 ] |
                "c:1" $duration = T_1_8
                    Tone [ 5 4 3 3 4 5 5 4 3 ]
                    $duration = _1_8
                    Tone [ 3 ^1 ] |
                "c:2" $duration = T_1_8
                    Tone [ 5 4 3 3 4 5 5 4 3 ]
                    $duration = _1_8
                    Tone [ 3 1, 2, ] |
                "c:3" Tone [ 3 5 5 1, 2, 3 5 5 1, 2, ] |
                "c:4" $duration = _1_16
                    Tone [ _* 1 2 _* 1 2 ]
                    $duration = _1_16
                    Tone [ 3 4, 3, 2 3, 4, 4 5, 4, 3 4, 5, ] |
                "c:4@2" $duration = _1_8
                    Tone [ _ 1, 2, _ 1, 2, 3 4 5 ^1 @ ] |
                "c:5" $duration = _1
                    Tone [ @ ^1 ] |
                "b" $duration = _1_8
                    Tone [ 1 .5 3 1 4 3, 1, 2, 3, 2 ] |
                "o:3" Tone [ 1 .5 3 1 1 .5 3 1 @ ] |
                "o:4" $duration = _1
                    Tone [ @ 1 ] |
            ]}
        ]
        Sections: [
            {intro Intro [
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "i:1,2" | ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em7" 1 ; "i:1,2" | ]
                } {
                    chord [ "1/4" 1 ]
                    guitar [ "G/C" 1 ; "i:3" | ]
                } {
                    chord [ "1/5" 1 ]
                    guitar [ "G/D" 1 ; "i:4" | ]
                }
            ]}
            {verse Verse [
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "v:1,2" | ]
                    lyrics [ "1:1" | @ 1 3 ; "2:1" | @ 2 4 ]
                    vocal [ "v:1,2" | ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em7" 1 ; "v:1,2" | ]
                    lyrics [ "1:2" | @ 1 3 ; "2:2" | @ 2 4 ]
                    vocal [ "v:1,2" | ]
                } {
                    chord [ "4" 1 ]
                    guitar [ "Cmaj7" 1 ; "v:3" | ]
                    lyrics [ "1:3" | @ 1 3 ; "2:3" | @ 2 4 ]
                    vocal [ "v:3" | ]
                } {
                    chord [ "b6" 1 ]
                    guitar [ "bEdim7" 1 ; "v:4" | ]
                    lyrics [ "1:4" | @ 1 3 ; "2:4" | @ 2 4 ]
                    vocal [ "v:4_1" | @ 1 3 ; "v:4_2" | @ 2 4]
                }
            ]}
            {pre PreChorus [
                {
                    chord [ "4" 1 ]
                    guitar [ "Cmaj7" 1 ; "p:1" | ]
                    lyrics [ "p:1" | ]
                    vocal [ "p:1" | ]
                } {
                    chord [ "3-" 1 ]
                    guitar [ "Bm7" 1 ; "p:2" | ]
                    lyrics [ "p:2" | ]
                    vocal [ "p:2" | ]
                } {
                    chord [ "2-" 1 ]
                    guitar [ "Am7" 1 ; "p:3" | ]
                    lyrics [ "p:3" | ]
                    vocal [ "p:3" | ]
                } {
                    chord [ "5" 1 ]
                    guitar [ "D" 1 ; "p:4" | ]
                    lyrics [ "p:4" | ]
                    vocal [ "p:4" | ]
                }
            ]}
            {chorus Chorus [
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "c:1,2" | ]
                    lyrics [ "c:1" | @ 1 3 ; "c:1@2" | @ 2 4 ]
                    vocal [ "c:1" | ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em7" 1 ; "c:1,2" | ]
                    lyrics [ "c:2" | @ 1 3 ; "c:2@2" | @ 2 4 ]
                    vocal [ "c:2" | ]
                } {
                    chord [ "4" 1 ]
                    guitar [ "Cmaj7" 1 ; "c:3" | ]
                    lyrics [ "c:3" | @ 1 3 ; "c:3@2" | @ 2 4 ]
                    vocal [ "c:3" | ]
                } {
                    chord [ "5" 1 ]
                    guitar [ "D" 1 ; "c:4" | ]
                    lyrics [ "c:4" | @ 1 3 ; "c:4@2" | @ 2 4 ]
                    vocal [ "c:4" | @ 1 3 ; "c:4@2" | @ 2 4 ]
                }
            ]}
            {bridge Bridge [
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "i:1,2" | ]
                    lyrics [ "b:1" | ]
                    vocal [ "b" | ; "c:5" | @ 1 ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em7" 1 ; "i:1,2" | ]
                    lyrics [ "b:2" | ]
                    vocal [ "b" | ]
                } {
                    chord [ "4" 1 ]
                    guitar [ "Cmaj7" 1 ; "i:3" | ]
                    lyrics [ "b:3" | ]
                    vocal [ "b" | ]
                } {
                    chord [ "1/5" 1 ]
                    guitar [ "G/D" 1 ; "i:4" | ]
                    lyrics [ "b:4" | ]
                    vocal [ "b" | ]
                }
            ]}
            {outro Outro [
                {
                    chord [ "1" 1 ]
                    guitar [ "G" 1 ; "i:1,2" | ]
                    lyrics [ "b:1" | ]
                    vocal [ "b" | ]
                } {
                    chord [ "6-" 1 ]
                    guitar [ "Em7" 1 ; "i:1,2" | ]
                    lyrics [ "b:2" | ]
                    vocal [ "b" | ]
                } {
                    chord [ "1,4" 2 ]
                    guitar [ "Cmaj7" 1 ; "o:3" | ]
                    lyrics [ "o:3" | ]
                    vocal [ "o:3" | ]
                } {
                    chord [ "1_9" 1 ]
                    guitar [ "Gadd9" 1 ; "o:4" | ]
                    lyrics [ "0:4" | ]
                    vocal [ "o:4" | ]
                }
            ]}
        ]
        Form: intro verse verse pre chorus chorus bridge bridge outro
    }
}
