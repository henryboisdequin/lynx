#![allow(clippy::too_many_lines, clippy::struct_excessive_bools)]

pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    #[must_use]
    pub fn highlighting_options(&self) -> &HighlightingOptions {
        &self.hl_opts
    }
    #[must_use]
    pub fn from(file_name: &str) -> Self {
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "as".to_string(),
                        "break".to_string(),
                        "const".to_string(),
                        "continue".to_string(),
                        "crate".to_string(),
                        "else".to_string(),
                        "enum".to_string(),
                        "extern".to_string(),
                        "false".to_string(),
                        "fn".to_string(),
                        "for".to_string(),
                        "if".to_string(),
                        "impl".to_string(),
                        "in".to_string(),
                        "let".to_string(),
                        "loop".to_string(),
                        "match".to_string(),
                        "mod".to_string(),
                        "move".to_string(),
                        "mut".to_string(),
                        "pub".to_string(),
                        "ref".to_string(),
                        "return".to_string(),
                        "self".to_string(),
                        "Self".to_string(),
                        "static".to_string(),
                        "struct".to_string(),
                        "super".to_string(),
                        "trait".to_string(),
                        "true".to_string(),
                        "type".to_string(),
                        "unsafe".to_string(),
                        "use".to_string(),
                        "where".to_string(),
                        "while".to_string(),
                        "dyn".to_string(),
                        "abstract".to_string(),
                        "become".to_string(),
                        "box".to_string(),
                        "do".to_string(),
                        "final".to_string(),
                        "macro".to_string(),
                        "override".to_string(),
                        "priv".to_string(),
                        "typeof".to_string(),
                        "unsized".to_string(),
                        "virtual".to_string(),
                        "yield".to_string(),
                        "async".to_string(),
                        "await".to_string(),
                        "try".to_string(),
                    ],
                    secondary_keywords: vec![
                        "bool".to_string(),
                        "char".to_string(),
                        "i8".to_string(),
                        "i16".to_string(),
                        "i32".to_string(),
                        "i64".to_string(),
                        "isize".to_string(),
                        "u8".to_string(),
                        "u16".to_string(),
                        "u32".to_string(),
                        "u64".to_string(),
                        "usize".to_string(),
                        "f32".to_string(),
                        "f64".to_string(),
                    ],
                },
            };
        }
        if file_name.ends_with(".py") {
            return Self {
                name: String::from("Python"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "and".to_string(),
                        "as".to_string(),
                        "assert".to_string(),
                        "break".to_string(),
                        "class".to_string(),
                        "continue".to_string(),
                        "def".to_string(),
                        "del".to_string(),
                        "elif".to_string(),
                        "else".to_string(),
                        "except".to_string(),
                        "False".to_string(),
                        "finally".to_string(),
                        "for".to_string(),
                        "from".to_string(),
                        "global".to_string(),
                        "if".to_string(),
                        "import".to_string(),
                        "in".to_string(),
                        "is".to_string(),
                        "lambda".to_string(),
                        "None".to_string(),
                        "nonlocal".to_string(),
                        "not".to_string(),
                        "or".to_string(),
                        "pass".to_string(),
                        "raise".to_string(),
                        "return".to_string(),
                        "True".to_string(),
                        "try".to_string(),
                        "while".to_string(),
                        "with".to_string(),
                        "yield".to_string(),
                    ],
                    secondary_keywords: vec![
                        "str".to_string(),
                        "int".to_string(),
                        "float".to_string(),
                        "complex".to_string(),
                        "list".to_string(),
                        "tuple".to_string(),
                        "range".to_string(),
                        "dict".to_string(),
                        "set".to_string(),
                        "frozenset".to_string(),
                        "bool".to_string(),
                        "bytes".to_string(),
                        "bytearray".to_string(),
                        "memoryview".to_string(),
                    ],
                },
            };
        }
        Self::default()
    }
}

impl HighlightingOptions {
    #[must_use]
    pub fn numbers(&self) -> bool {
        self.numbers
    }
    #[must_use]
    pub fn strings(&self) -> bool {
        self.strings
    }
    #[must_use]
    pub fn characters(&self) -> bool {
        self.characters
    }
    #[must_use]
    pub fn comments(&self) -> bool {
        self.comments
    }
    #[must_use]
    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }
    #[must_use]
    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }
    #[must_use]
    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }
}
