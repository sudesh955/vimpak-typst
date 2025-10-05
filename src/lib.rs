mod fonts;
mod utils;

use std::convert::TryInto;

use typst::{
    diag::FileResult,
    foundations::{Bytes, Datetime},
    layout::Abs,
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
    Library,
};
use wasm_bindgen::prelude::*;

use crate::fonts::get_fonts;

pub struct VimpakTypstWorld {
    source: Source,
    fonts: Vec<Font>,
    book: LazyHash<FontBook>,
    time: time::OffsetDateTime,
    library: LazyHash<Library>,
}

impl VimpakTypstWorld {
    pub fn new(source: String) -> Self {
        let fonts = get_fonts();
        let t = (now() / 1000.0) as i64;
        Self {
            fonts: fonts.items,
            book: LazyHash::new(fonts.book),
            source: Source::detached(source),
            library: LazyHash::new(Library::default()),
            time: time::OffsetDateTime::from_unix_timestamp(t).expect("problem"),
        }
    }
}

impl typst::World for VimpakTypstWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.source.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(typst::diag::FileError::AccessDenied)
        }
    }

    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(typst::diag::FileError::AccessDenied)
    }

    fn font(&self, id: usize) -> Option<Font> {
        self.fonts.get(id).cloned()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let offset = offset.unwrap_or(0);
        let offset = time::UtcOffset::from_hms(offset.try_into().ok()?, 0, 0).ok()?;
        let time = self.time.checked_to_offset(offset)?;
        Some(Datetime::Date(time.date()))
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = Date)]
    fn now() -> f64;

}

#[wasm_bindgen]
pub unsafe fn compile(source: String) -> Option<String> {
    utils::set_panic_hook();
    let world = VimpakTypstWorld::new(source);
    let result = typst::compile(&world);
    let document = result.output.ok()?;
    Some(typst_svg::svg_merged(&document, Abs::zero()))
}
