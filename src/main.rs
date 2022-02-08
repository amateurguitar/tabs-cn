use notation_tab::prelude::*;

pub mod songs;
pub mod drafts;

pub fn main() {
    //drafts
    write_tab(&drafts::song_bie::new_tab(), "tabs/drafts/song_bie.ron");
    write_tab(&drafts::xiao_cheng_gu_shi::new_tab(), "tabs/drafts/xiao_cheng_gu_shi.ron");
    write_tab(&drafts::xia_ri_li_zui_hou_de_mei_gui::new_tab(), "tabs/drafts/song_bie.ron");
    //songs
    write_tab(&songs::pu_shu::bai_hua_lin::new_tab(), "tabs/songs/pu_shu/bai_hua_lin.ron");
    write_tab(&songs::jay::long_juan_feng::new_tab(), "tabs/songs/jay/long_juan_feng.ron");
    write_tab(&songs::wang_jie::anne::new_tab(), "tabs/songs/wang_jie/anne.ron");
    write_tab(&songs::luo_da_you::ge::new_tab(), "tabs/songs/luo_da_you/ge.ron");
}
