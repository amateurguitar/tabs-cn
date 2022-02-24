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
                $duration = _1_8
                "i1" Tone [ _** .6* 1* ] |
                "a1" Tone [ 5* _* 5= 4- 5= 6- ] |
                "a2" Tone [ _= 3- @ 3* _* _- 3- 5- ] |
                "a3" Tone [ 7* _* _= 3- 5= 7- ] |
                "a4" Tone [ ^1* @ ^1= _- _** ] |
                "a5" Tone [ .6= 1* 5* _- _* ] |
                "a6" Tone [ .6= 1* 6- 5* _= .6- @ ] |
                "a7" Tone [ @ .6= 1* 4- 5** @ ] |
                "a8" Tone [ @ 5* _* .6* 1* ] |
                "a9" Tone [ 5* _* 5= 4* 5- ] |
                "a10" Tone [ 6= 3- @ 3* _* _- 3- 5- ] |
                "a11" Tone [ 7* _* _= 3- 5= ]
                $duration = T_1_16
                      Tone [ 7 ^1 @ ] |
                $duration = _1_8
                "a12" Tone [ @ ^1** _** ] |
                "a13" Tone [ .6= 1* 4- 5* _* ] |
                "a14" Tone [ .6= 1* 6- 5* _* ] |
                "a15" Tone [ .6= 1* 4- 5** @ ] |
                "a16" Tone [ @ 5**+ _* ] |
                "a1_2" Tone [ 5* _* 5= 4- 5* ] |
                "a2_2" Tone [ 6= 3- @ 3* _* _- 3- 5- ] |
                "a3_2" Tone [ 7* _* _= 3- 5= 7- ] |
                "b1" Tone [ 1= 1- 2= 2- 3= 3- 2= 1- ] |
                "b2" Tone [ 1= 1- 2= 2- 3= 3- _- 2- 1- ] |
                "b3" Tone [ 4= 5- 6= 6- 7- 7- 6* 5- @ ] |
                "b4" Tone [ @ 5= 7* @ 7- _* 6= 7- ] |
                "b5" Tone [ ^1= ^1- ^1= ^1- ^1- ^1- ^1* ^1-] |
                "b6" Tone [ ^1= ^1- ^1= ^1- ^1= 6- 7= ^1- ] |
                "b7" Tone [ ^2* _**+ ] |
                "b8" Tone [ _** .6* 1* ] |

            ]}
        ]
        Sections: [
            {i Intro [
                {
                    chord [ "1" 1 ]
                    vocal [ "i1" | ]
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
                } {
                    chord [ "1" 1 ]
                    vocal [ "a9" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a10" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a11" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a12" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a13" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a14" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a15" | ]
                } {
                    chord [ "1" 1 ]
                    vocal [ "a16" | ]
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
        Form: i a b
    }
}

