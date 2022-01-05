#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! notation_tab = "0.3.3"
//! ```

use notation_tab::prelude::*;

pub fn main() {
    print_tab(&new_tab());
}

pub fn new_tab() -> Tab {
    tab! {
        "55127814-52cb-4cbc-a327-378ecd0a329d"
        Meta: D Major 4 _4 80
        Tracks: [
            {chord Chord [
                $duration = _1
                "D" Chord ( 1: 3 5 )
                "Em" Chord ( 2: 3- 5 )
                "G" Chord ( 4: 3 5 )
                "E" Chord ( 2: 3 5 )
                "A7" Chord [ ( 5: 3 5 7- ) ] |
                $duration = _1_2
                "A,A7" Chord [ ( 5: 3 5 ) ( 5: 3 5 7- ) ] |
                "D,#Fm" Chord [ ( 1: 3 5 ) ( 3: 3- 5 ) ] |
                "Em,A" Chord [ ( 2: 3- 5 ) ( 5: 3 5 ) ] |
                "D,Dmaj7,D7" Chord [ ( 1: 3 5 ) ( 1: 3 5 7 ,) ( 1: 3 5 7- ,) ] |
                $duration = _1_4
                "D,G,D" Chord [ ( 1: 3 5 +) ( 4: 3 5 7 ,) ( 1: 3 5 *) ] |
            ]}
            {guitar Guitar [
                Fretboard
                $duration = _1
                "D" Shape ( _ 0 0 2 3 2 )
                "Em" Shape ( 0 2 2 0 0 0 )
                "G" Shape ( 3 2 0 0 0 3 )
                "E" Shape ( 0 2 2 1 0 0 )
                "A7" Shape ( 0 0 2 2 2 3 ) |
                $duration = _1_2
                "A,A7"
                "A" Shape ( 0 0 2 2 2 0 )
                "A7" Shape ( 0 0 2 2 2 3 ) |
                "D,#Fm"
                "D" Shape ( _ 0 0 2 3 2 )
                "#Fm" Shape ( 2 4 4 2 2 2 ) |
                "Em,A"
                "Em" Shape ( 0 2 2 0 0 0 )
                "A" Shape ( 0 0 2 2 2 0 ) |
                "D,Dmaj7,D7"
                "D" Shape ( _ 0 0 2 3 2 )
                "Dmaj7" Shape ( _ 0 0 2 2 2 ),
                "D7" Shape ( _ 0 0 2 1 2 ), |
                "D,G,D"
                $duration = D_1_4
                "D" Shape ( _ 0 0 2 3 2 )
                $duration = _1_4
                "G" Shape ( 3 2 0 0 0 3 ),
                "D" Shape ( _ 0 0 2 3 2 )*
                |
                $duration = _1_8
                "4-6" Pick [ 4 3 (2 1) 3 6 3 (2 1) 3 ] |
                "6-5" Pick [ 6 3 (2 1) 3 5 3 (2 1) 3 ] |
                "6-6" Pick [ 6 3 (2 1) 3 6 3 (2 1) 3 ] |
                "4" Pick [ 4 3 2 3 (2 1) 3 2 3 ] |
                "i:1" Pick [ 4 3 2@3 2@5 (4 1@2) 3 1@5 3 ] |
                "i:2" Pick [ (6 3 2 1@3) 1@5 1@3 1@2 (6 2@5) 2@3 2@2,, 2@3,, 2@2, 2@0 ] |
                "i:3" Pick [ (5 3@6) 1@5 3@6,, 3@4,+ 1@3 3@4,, 3@2,+ 1@2 3@0 (5 1@0) ] |
                "i:4_1" Pick [ 4 3, 2, 1@2 1@3 1@2 4 2 3 ] |
                "i:4_2" Pick [ 4 3 2 1 ]
                $duration = _1_4
                    Pick [ (4 3 2 1) ]
                $duration = _1_8
                    Pick [ (3 2 1) 4 ] |
                "i:4_3" Pick [ 4 3, 2, 1 (2 1) ]
                $duration = _1_2
                    Pick [ (4 3 2 1) ] |
                $duration = _1_8
                "v:4_2" Pick [ 4 3 (2 1) 3 (2 1) 3 (2 1) 3 ] |
                "c:4" Pick [ (4 2 1) 3 (2 1) 3 ]
                $duration = _1_4
                    Pick [ (4 3 2 1) ]
                $duration = _1_8
                    Pick [ (3 2 1) 4 ] |
                "c:7" Pick [ 6 5 4 3 2 6 4 6 ] |
                "c:8"
                $duration = _1_2
                Pick [ (5 4 3 2) ]
                $duration = _1_4
                Pick [ (3@2 1@5) (2@2 1@3) ] |
                "c:9"
                $duration = _1_2
                Pick [ (2@2 1@3) ] |
            ]}
            {vocal Vocal [
                $duration = _1_8
                "i:4" Tone [ _**+ _ .5 ] |
                "i:4_2" Tone [ _**+ _ 1 ] |
                "v:1" Tone [ 3 3 3 3 3* 5* ] |
                "v:1_3" Tone [ 3* 3 3 3 5 5 5 ] |
                "v:1_4" Tone [ 3* 3* _ 3 3 5 ] |
                "v:2" Tone [ 4 5 4 3 2* _ .5 ] |
                "v:3" Tone [ 2 2 2 2 2* 1+ 2, ] |
                "v:3_2" Tone [ 2* 2* .7 .7 .6, .7+ ] |
                "v:3_3" Tone [ 2 2 2 2 2 2 1 2 ] |
                "v:4" Tone [ 3**+ _ .5 ] |
                "v:4_2" Tone [ 1**+ _ 1 ] |
                "v:4_4" Tone [ 1*** ] |
                "c:1" Tone [ 6*+ 5 4* 6* ] |
                "c:2" Tone [ 5 6 5 4 3* 5* ] |
                "c:3" Tone [ 4+ 5, 4 3 2*+ .7 ] |
                "c:3_2" Tone [ 4+ 5, 4 3 2* .7 .7 ] |
                "c:4" Tone [ 1 1 2* 3* _ 1 ] |
                "c:6" Tone [ 5 6 5 4 3* 5+ 5, ] |
                "c:7" Tone [ #4 3 #4 5 6* 2* ] |
                "c:8" Tone [ 7*+ 6 5* 4* ] |
                "c:9" Tone [ @ 4** _*+ .5 ] |
            ]}
            {lyrics Lyrics [
                $duration = _1_8
                "i:4" Word [ _**+ _ "当" ] |
                "i:4_2" Word [ _**+ _ "我" ] |
                "v:1" Word [ "我" "死" "去" "的" "时"* "侯"* ] |
                "v:2" Word [ "亲"** "爱"* _ "你" ] |
                "v:3" Word [ "别" "为" "我" "唱" "悲"* "伤"+ "的", ] |
                "v:4" Word [ "歌"**+ _ "我" ] |
                "v:1_2" Word [ "坟" "上" "不" "必" "安"* "插"* ] |
                "v:2_2" Word [ "蔷"** "薇"* _ "也" ] |
                "v:3_2" Word [ "无"* "需"* "浓" "密" "的", "柏"+ ] |
                "v:4_2" Word [ "树"**+ _ "让" ] |
                "v:1_3" Word [ "悠"* "久" "的" "坟"* "墓" "中" ] |
                "v:2_3" Word [ "迷"** "惘"* _ "阳" ] |
                "v:3_3" Word [ "光" "不" "升" "起" "也" "不" "消"* ] |
                "v:4_3" Word [ "翳"**+ _ "我" ] |
                "v:1_4" Word [ "也"* "许"* _ "也" "许" "我" ] |
                "v:2_4" Word [ "还"* "记" "得" "你"* _ "我" ] |
                "v:3_4" Word [ "也"* "许"* "把" "你" "忘"* ] |
                "v:4_4" Word [ "记"*** ] |
                "c:1" Word [ "盖"*+ "着" "我"* "的"* ] |
                "c:2" Word [ "轻"* "轻" "的" "草"** ] |
                "c:3" Word [ "淋"*+ "着" "雨"*+ "也" ] |
                "c:4" Word [ "沾" "着" "露"* "珠"* _ "假" ] |
                "c:5" Word [ "如"*+ "你" "愿"* "意"* ] |
                "c:6" Word [ "请"* "记" "着" "我"** ] |
                "c:7" Word [ "要"* "是" "你" "甘"* "心"* ] |
                "c:8" Word [ "忘"*+ "了" "我"** ] |
                "c:9" Word [ @ ""** _*+ "在" ] |
                "c:1_2" Word [ "再"* "见" "不" "到"** ] |
                "c:2_2" Word [ "地"* "面" "的" "青"* "荫"* ] |
                "c:3_2" Word [ "觉"* "不" "到" "雨"* "露" "的" ] |
                "c:4_2" Word [ "甜"** "蜜"* _ "我" ] |
                "c:5_2" Word [ "再"* "听" "不" "到"** ] |
                "c:6_2" Word [ "夜"* "莺" "的" "歌"* "喉"+ "在" ] |
                "c:7_2" Word [ "黑"** "夜"* "里"* ] |
                "c:8_2" Word [ "倾"*+ "诉" "悲"* "啼"* ] |
                "c:9_2" Word [ @ ""** _*+ "在" ] |
            ]}
        ]
        Sections: [
            intro Intro [{
                chord [ "D" 1 ]
                guitar [ "D" 1 ; "i:1" | ]
            }{
                chord [ "Em" 1 ]
                guitar [ "Em" 1 ; "i:2" | ]
            }{
                chord [ "A,A7" | ]
                guitar [ "A,A7" | ; "i:3" | ]
            }{
                chord [ "D" 1 @ 1 ; "D,Dmaj7,D7" | @ 2 ; "D,G,D" | @ 3 ]
                guitar [ "D" 1 @ 1 ; "D,Dmaj7,D7" | @ 2 ; "D,G,D" | @ 3 ]
                guitar [ "i:4_1" | @ 1 ; "i:4_2" | @ 2 ; "i:4_3" | @ 3 ]
                vocal [ "i:4" | @ 1 ; "i:4_2" | @ 2 ]
                lyrics [ "i:4" | @ 1 ; "i:4_2" | @ 2 ]
            }]
            verse Verse [{
                chord [ "D,#Fm" | ]
                guitar [ "D,#Fm" | ; "4-6" | ]
                vocal [ "v:1" | @ 1 2 ; "v:1_3" | @ 3 5 ; "v:1_4" | @ 4 6 ]
                lyrics [ "v:1" | @ 1 ; "v:1_2" | @ 2 ; "v:1_3" | @ 3 5 ; "v:1_4" | @ 4 6 ]
            }{
                chord [ "Em,A" | ]
                guitar [ "Em,A" | ; "6-5" | ]
                vocal [ "v:2" | ]
                lyrics [ "v:2" | @ 1 ; "v:2_2" | @ 2 ; "v:2_3" | @ 3 5 ; "v:2_4" | @ 4 6 ]
            }{
                chord [ "Em,A" | ]
                guitar [ "Em,A" | ; "6-5" | ]
                vocal [ "v:3" | @ 1 ; "v:3_2" | @ 2 4 6 ; "v:3_3" | @ 3 5 ]
                lyrics [ "v:3" | @ 1 ; "v:3_2" | @ 2 ; "v:3_3" | @ 3 5 ; "v:3_4" | @ 4 6 ]
            }{
                chord [ "D" 1 @ 1 3 5 ; "D,Dmaj7,D7" | @ 2 4 6 ]
                guitar [ "D" 1 @ 1 3 5 ; "D,Dmaj7,D7" | @ 2 4 6 ]
                guitar [ "i:4_1" | @ 1 3 5 ; "v:4_2" | @ 2 4 6 ]
                vocal [ "v:4" | @ 1 3 5 ; "v:4_2" | @ 2 ; "v:4_4" | @ 4 6 ]
                lyrics [ "v:4" | @ 1 ; "v:4_2" | @ 2 ; "v:4_3" | @ 3 5 ; "v:4_4" | @ 4 6 ]
            }]
            chorus Chorus [{
                chord [ "G" 1 ]
                guitar [ "G" 1 ; "6-6" | ]
                vocal [ "c:1" | ]
                lyrics [ "c:1" | @ 1 ; "c:1_2" | @ 2 ]
            }{
                chord [ "D" 1 ]
                guitar [ "D" 1 ; "4" | ]
                vocal [ "c:2" | ]
                lyrics [ "c:2" | @ 1 ; "c:2_2" | @ 2 ]
            }{
                chord [ "Em,A" | ]
                guitar [ "Em,A" | ; "6-5" | ]
                vocal [ "c:3" | @ 1 ; "c:3_2" | @ 2 ]
                lyrics [ "c:3" | @ 1 ; "c:3_2" | @ 2 ]
            }{
                chord [ "D,Dmaj7,D7" | ]
                guitar [ "D,Dmaj7,D7" | ; "c:4" | ]
                vocal [ "c:4" | ]
                lyrics [ "c:4" | @ 1 ; "c:4_2" | @ 2 ]
            }{
                chord [ "G" 1 ]
                guitar [ "G" 1 ; "6-6" | ]
                vocal [ "c:1" | ]
                lyrics [ "c:5" | @ 1 ; "c:5_2" | @ 2 ]
            }{
                chord [ "D" 1 ]
                guitar [ "D" 1 ; "4" | ]
                vocal [ "c:2" | @ 1 ; "c:6" | @ 2 ]
                lyrics [ "c:6" | @ 1 ; "c:6_2" | @ 2 ]
            }{
                chord [ "E" 1 ]
                guitar [ "E" 1 ; "c:7" | ]
                vocal [ "c:7" | ]
                lyrics [ "c:7" | @ 1 ; "c:7_2" | @ 2 ]
            }{
                chord [ "A,A7" | ]
                guitar [ "A,A7" | ; "c:8" | ]
                vocal [ "c:8" | ]
                lyrics [ "c:8" | @ 1 ; "c:8_2" | @ 2 ]
            }{
                chord [ "A7" | ]
                guitar [ "A7" | ; "c:9" | ]
                vocal [ "c:9" | ]
                lyrics [ "c:9" | @ 1 ; "c:9_2" | @ 2 ]
            }]
        ]
        Form: intro verse verse chorus verse verse intro chorus verse verse intro
    }
}