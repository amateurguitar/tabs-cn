use notation_tab::prelude::*;

pub mod songs;

pub fn main() {
    //songs
    write_tab(&songs::pu_shu::bai_hua_lin::new_tab(), "tabs/songs/pu_shu/bai_hua_lin.ron");
    write_tab(&songs::jay::long_juan_feng::new_tab(), "tabs/songs/jay/long_juan_feng.ron");
    write_tab(&songs::wang_jie::anne::new_tab(), "tabs/songs/wang_jie/anne.ron");
    write_tab(&songs::luo_da_you::ge::new_tab(), "tabs/songs/luo_da_you/ge.ron");
}
