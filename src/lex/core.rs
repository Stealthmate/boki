#[derive(Clone, Debug)]
pub struct StringScanner {
    content: std::sync::Arc<str>,
    offset: usize,
    limit: usize,
}

impl StringScanner {
    pub fn as_str(&self) -> &str {
        &self.content[self.offset..self.limit]
    }

    pub fn is_empty(&self) -> bool {
        if self.limit == 0 {
            return true;
        }
        self.offset == self.limit
    }

    pub fn location(&self) -> usize {
        self.offset
    }

    pub fn eof_idx(&self) -> usize {
        self.content.len()
    }
}

impl From<&str> for StringScanner {
    fn from(value: &str) -> Self {
        let limit = value.len();
        StringScanner {
            content: std::sync::Arc::from(value),
            offset: 0,
            limit,
        }
    }
}

impl nom::Compare<&str> for StringScanner {
    fn compare(&self, t: &str) -> nom::CompareResult {
        self.as_str().compare(t)
    }
    fn compare_no_case(&self, t: &str) -> nom::CompareResult {
        self.as_str().compare_no_case(t)
    }
}

impl nom::FindSubstring<&str> for StringScanner {
    fn find_substring(&self, substr: &str) -> Option<usize> {
        self.as_str().find_substring(substr)
    }
}

impl nom::Offset for StringScanner {
    fn offset(&self, second: &Self) -> usize {
        self.as_str().offset(second.as_str())
    }
}

struct StringScannerIteratorBase {
    chars: Vec<(usize, char)>,
    index: usize,
}

impl From<&StringScanner> for StringScannerIteratorBase {
    fn from(value: &StringScanner) -> Self {
        Self {
            chars: value.content.chars().enumerate().collect(),
            index: 0,
        }
    }
}

impl std::iter::Iterator for StringScannerIteratorBase {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.chars.len() {
            return None;
        }
        self.index += 1;
        Some(self.index - 1)
    }
}

pub struct StringScannerIterator(StringScannerIteratorBase);

impl std::iter::Iterator for StringScannerIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| self.0.chars[i].1)
    }
}

impl From<StringScannerIteratorBase> for StringScannerIterator {
    fn from(value: StringScannerIteratorBase) -> Self {
        Self(value)
    }
}

pub struct StringScannerIdxIterator(StringScannerIteratorBase);

impl std::iter::Iterator for StringScannerIdxIterator {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|i| self.0.chars[i])
    }
}
impl From<StringScannerIteratorBase> for StringScannerIdxIterator {
    fn from(value: StringScannerIteratorBase) -> Self {
        Self(value)
    }
}

impl nom::Input for StringScanner {
    type Item = char;
    type Iter = StringScannerIterator;
    type IterIndices = StringScannerIdxIterator;

    fn input_len(&self) -> usize {
        self.limit - self.offset
    }

    fn take(&self, index: usize) -> Self {
        let limit = self.offset + index;
        if limit > self.limit {
            panic!("Tried to take outside bounds.");
        }
        Self {
            content: self.content.clone(),
            offset: self.offset,
            limit,
        }
    }

    fn take_from(&self, index: usize) -> Self {
        let offset = self.offset + index;
        if offset > self.limit {
            panic!("Tried to take outside bounds.");
        }
        Self {
            content: self.content.clone(),
            offset,
            limit: self.limit,
        }
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let adj_index = self.offset + index;

        if index > self.limit - self.offset {
            panic!("Tried to take outside bounds.");
        }

        let first = Self {
            content: self.content.clone(),
            offset: self.offset,
            limit: adj_index,
        };
        let second = Self {
            content: self.content.clone(),
            offset: adj_index,
            limit: self.limit,
        };

        (second, first)
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.as_str().find(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        StringScannerIterator::from(StringScannerIteratorBase::from(self))
    }

    fn iter_indices(&self) -> Self::IterIndices {
        StringScannerIdxIterator::from(StringScannerIteratorBase::from(self))
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        let n = self.limit - self.offset;
        if n >= count {
            Ok(count)
        } else {
            Err(nom::Needed::new(count - n))
        }
    }
}

#[derive(Clone, Debug)]
pub enum LexerErrorDetails {
    NothingMatched,
    InternalError(String),
}

/// The main error type for the parser stage.
#[derive(Debug)]
pub struct LexerError {
    /// The location of the token at which the error occurred.
    pub location: usize,
    /// Details about the exact error.
    pub details: LexerErrorDetails,
    pub previous_tokens: Vec<super::DecoratedToken>,
}

impl nom::error::ParseError<StringScanner> for LexerError {
    fn from_error_kind(input: StringScanner, kind: nom::error::ErrorKind) -> Self {
        Self {
            location: input.offset,
            details: LexerErrorDetails::InternalError(format!("{:#?}", kind)),
            previous_tokens: vec![],
        }
    }

    fn append(input: StringScanner, kind: nom::error::ErrorKind, _: Self) -> Self {
        Self {
            location: input.offset,
            details: LexerErrorDetails::InternalError(format!("{:#?}", kind)),
            previous_tokens: vec![],
        }
    }
}

impl From<nom::Err<LexerError>> for LexerError {
    fn from(value: nom::Err<LexerError>) -> Self {
        match value {
            nom::Err::Incomplete(_) => LexerError {
                location: 0,
                details: LexerErrorDetails::InternalError("Incomplete".to_string()),
                previous_tokens: vec![],
            },
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    }
}

impl From<nom::error::Error<LexerError>> for LexerError {
    fn from(_: nom::error::Error<LexerError>) -> Self {
        LexerError {
            location: 0,
            details: LexerErrorDetails::InternalError("Incomplete".to_string()),
            previous_tokens: vec![],
        }
    }
}

pub type NomResult<T> = nom::IResult<StringScanner, T, LexerError>;

pub type LexerResult<T> = Result<T, LexerError>;

pub(crate) fn error_at(location: usize, details: LexerErrorDetails) -> nom::Err<LexerError> {
    nom::Err::Failure(LexerError {
        location,
        details,
        previous_tokens: vec![],
    })
}
