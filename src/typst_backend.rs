use std::sync::OnceLock;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Dict};
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, World};

// 1. EMBED FONTS PERMANENTLY
static FONTS: OnceLock<(FontBook, Vec<Font>)> = OnceLock::new();

fn get_fonts() -> &'static (FontBook, Vec<Font>) {
    FONTS.get_or_init(|| {
        // Adjust these filenames to match exactly what you put in assets/fonts/
        let font_payloads = vec![
            include_bytes!("../assets/fonts/LiberationSans-Regular.ttf").as_slice(),
            include_bytes!("../assets/fonts/LiberationSans-Bold.ttf").as_slice(),
            include_bytes!("../assets/fonts/LiberationSans-Italic.ttf").as_slice(),
            include_bytes!("../assets/fonts/LiberationSans-BoldItalic.ttf").as_slice(),
        ];

        let mut book = FontBook::new();
        let mut fonts = Vec::new();

        for data in font_payloads {
            let buffer = Bytes::from_static(data);
            for font in Font::iter(buffer) {
                book.push(font.info().clone());
                fonts.push(font);
            }
        }
        (book, fonts)
    })
}

// 2. THE WORLD STRUCT
pub struct ResumeWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>, // Wrapped in LazyHash for 0.12 compatibility
    fonts: &'static [Font],
    source: Source,
    current_time: time::OffsetDateTime,
}

impl ResumeWorld {
    pub fn new(template_source: String, inputs: Dict) -> Self {
        let (book, fonts) = get_fonts();

        // FIX: Use builder pattern to inject inputs (typst 0.12+)
        let library = Library::builder().with_inputs(inputs).build();

        Self {
            library: LazyHash::new(library),
            // FIX: Clone the book from static and wrap in LazyHash
            book: LazyHash::new(book.clone()),
            fonts,
            source: Source::detached(template_source),
            current_time: time::OffsetDateTime::now_utc(),
        }
    }
}

// 3. IMPLEMENT THE TRAIT
impl World for ResumeWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    // FIX: Return type must match &LazyHash<FontBook>
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
            Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let date = self.current_time.date();
        // FIX: Datetime::from_ymd returns Option<Datetime>, no need to wrap in Some() again
        Datetime::from_ymd(date.year(), date.month() as u8, date.day())
    }
}
