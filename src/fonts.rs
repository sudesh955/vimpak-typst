use typst::foundations::Bytes;
use typst::text::{Font, FontBook};

pub(crate) struct Fonts {
    pub(crate) book: FontBook,
    pub(crate) items: Vec<Font>,
}

pub(crate) fn get_fonts() -> Fonts {
    let mut fonts = Fonts {
        items: vec![],
        book: FontBook::new(),
    };
    for data in typst_assets::fonts() {
        let buffer = Bytes::new(data);
        for font in Font::iter(buffer) {
            fonts.book.push(font.info().clone());
            fonts.items.push(font);
        }
    }
    fonts
}
