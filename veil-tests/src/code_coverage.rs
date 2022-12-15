#[test]
fn test_code_coverage_for_errors() {
    // Build a report which shows test code coverage for the
    // various syn::Error::new() calls throughout the macro's code.

    use std::{
        ffi::OsStr,
        num::NonZeroUsize,
        ops::Range,
        path::{Path, PathBuf},
    };

    struct ErrorMessage<'a, 'b> {
        message: String,
        file: &'a Path,
        line: NonZeroUsize,
        column: NonZeroUsize,
        code_coverage: Vec<&'b OsStr>,
    }
    impl std::fmt::Debug for ErrorMessage<'_, '_> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            fmt.debug_struct("ErrorMessage")
                .field(
                    "message",
                    &self
                        .message
                        .strip_prefix("error: ")
                        .unwrap()
                        .strip_suffix('\n')
                        .unwrap(),
                )
                .field("file", &self.file)
                .field("line", &self.line)
                .field("column", &self.column)
                .field("code_coverage", &self.code_coverage)
                .finish()
        }
    }

    struct SourceFile {
        path: PathBuf,
        contents: String,
        line_map: Vec<Range<usize>>,
    }
    impl SourceFile {
        fn new(path: PathBuf) -> Result<Self, std::io::Error> {
            let contents = std::fs::read_to_string(&path)?;

            let mut cursor = 0;
            let line_map = contents
                .split_inclusive('\n')
                .map(|line| {
                    let range = cursor..(cursor + line.len());
                    cursor += line.len();
                    range
                })
                .collect();

            Ok(Self {
                path,
                contents,
                line_map,
            })
        }

        fn line(&self, byte_offset: usize) -> Option<(NonZeroUsize, NonZeroUsize)> {
            let line_range = &self.line_map[self
                .line_map
                .binary_search_by(|line_number| {
                    if line_number.contains(&byte_offset) {
                        std::cmp::Ordering::Equal
                    } else if line_number.start > byte_offset {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                })
                .ok()?];

            let line_number = NonZeroUsize::new(line_range.start + 1).unwrap();
            let column_number = NonZeroUsize::new(byte_offset - line_range.start + 1).unwrap();

            Some((line_number, column_number))
        }
    }

    let mut error_msgs = Vec::new();
    let mut macro_src_files = Vec::new();

    for path in walkdir::WalkDir::new("../veil-macros/src")
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.expect("Failed to walk veil-macros directory");
            if entry.file_type().is_file() && entry.path().extension() == Some(OsStr::new("rs")) {
                return Some(entry.into_path());
            }
            None
        })
    {
        macro_src_files.push(SourceFile::new(path).expect("Failed to read veil-macros src file"));
    }

    let re_syn_error = regex::Regex::new(r#"syn::Error::new(?:_spanned)?\(\s*.+?,\s*"(.+?)",?\s*\)"#).unwrap();
    for src in macro_src_files.iter() {
        for cap in re_syn_error.captures_iter(&src.contents) {
            let error_msg = cap.get(1).unwrap();

            let (line, column) = src
                .line(error_msg.start())
                .expect("Failed to find line number for error message");

            error_msgs.push(ErrorMessage {
                message: format!("error: {}\n", error_msg.as_str()),
                file: src.path.as_path(),
                line,
                column,
                code_coverage: Vec::new(),
            });
        }
    }

    let compile_fail_tests = std::fs::read_dir("../veil-tests/src/compile_tests/fail")
        .expect("Failed to read veil-tests/src/compile_tests/fail")
        .filter_map(|entry| {
            let entry = entry.expect("Failed to read entry in veil-tests/src/compile_tests/fail");
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                let path = entry.path();
                if path.extension() == Some(OsStr::new("rs")) {
                    return Some(path);
                }
            }
            None
        })
        .collect::<Vec<_>>();

    for path in compile_fail_tests.iter().map(|p| p.as_path()) {
        let stderr = path.with_extension("stderr");
        if !stderr.is_file() {
            panic!(
                "The stderr file for test {:?} hasn't been added yet",
                path.file_name().unwrap()
            );
        }

        let stderr = std::fs::read_to_string(stderr).expect("Failed to read stderr file for compile fail test");
        for error_msg in error_msgs.iter_mut() {
            if stderr.contains(&error_msg.message) {
                error_msg.code_coverage.push(path.file_name().unwrap());
            }
        }
    }

    println!("{error_msgs:#?}");

    assert!(
        error_msgs.is_empty() == compile_fail_tests.is_empty(),
        "There are {} compile fail tests, but {} error messages were found",
        compile_fail_tests.len(),
        error_msgs.len()
    );
    assert!(
        error_msgs.iter().any(|error_msg| !error_msg.code_coverage.is_empty()),
        "No error messages were covered by any compile fail tests!"
    );

    for error_msg in error_msgs {
        assert!(
            !error_msg.code_coverage.is_empty(),
            "This error message is not covered by any compile fail tests: {error_msg:#?}"
        );
    }
}
