use notation_tab::prelude::*;

pub mod test;
pub mod songs;

pub fn main() {
    write_tab(&test::new_tab(), "tabs/test.ron");
    //songs
    write_tab(&songs::pu_shu::bai_hua_lin::new_tab(), "tabs/songs/pu_shu/bai_hua_lin.ron");
    write_tab(&songs::jay::long_juan_feng::new_tab(), "tabs/songs/jay/long_juan_feng.ron");
}