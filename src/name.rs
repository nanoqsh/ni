use core::{error, fmt, num::NonZeroU128, slice, str};

const _: () = {
    assert!(
        size_of::<Name>() == size_of::<&str>(),
        "`Name` and `&str` must have the same size",
    );

    assert!(
        size_of::<Name>() == size_of::<Option<Name>>(),
        "`Name` and `Option<Name>` must have the same size",
    );

    assert!(
        align_of::<Name>() == align_of::<u64>(),
        "`Name` and `u64` must have the same alignment",
    );
};

#[repr(Rust, packed)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InnerU128(NonZeroU128);

/// Small limited alloc-free named identifier.
///
/// The name has the same layout as `&str` and also has a niche:
/// * `size_of::<Name>() == size_of::<&str>()`
/// * `size_of::<Name>() == size_of::<Option<Name>>()`
///
/// Names are also `Copy`.
///
/// # Invariants
///
/// * Non empty.
/// * Maximum length 24.
/// * May only contain:
///   - Latin lowercased letters `'a'..='z'`
///   - Digits `'0'..='9'`
///   - Underscores `'_'`
/// * Cannot start with an underscore.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(InnerU128, [u64; 0]);

impl Name {
    /// The maximum possible length of a name.
    pub const MAXLEN: usize = 24;

    /// Encodes the name from a slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the source string violates name invariants.
    /// See [`Error`] for more details.
    #[inline]
    pub const fn encode(s: &[u8]) -> Result<Self, Error> {
        if s.is_empty() {
            return Err(Error::Empty);
        }

        if s.len() > Self::MAXLEN {
            return Err(Error::TooLong);
        }

        if let [b'_', ..] = s {
            return Err(Error::LeadingUnderscore);
        }

        let mut mul = 1;
        let mut sum = 0;
        let mut idx = s.len() - 1;
        loop {
            let u = match s[idx] {
                b'_' => 0,
                n @ b'a'..=b'z' => n - b'a' + 1,
                n @ b'0'..=b'9' => n - b'0' + 27,
                _ => return Err(Error::InvalidChar),
            };

            sum += u as u128 * mul;
            mul *= 37;
            if idx == 0 {
                break;
            }

            idx -= 1;
        }

        // SAFETY:
        // * Sum is 0 only when
        //   - all chars are `_` but this is prohibited because the first character is not `_`.
        //   - the string is empty but this is also prohibited.
        let num = unsafe { NonZeroU128::new_unchecked(sum) };
        Ok(Self(InnerU128(num), []))
    }

    /// Encodes the name from a char.
    ///
    /// # Errors
    ///
    /// Returns an error if the source char is invalid for a name.
    #[inline]
    pub const fn encode_char(c: char) -> Result<Self, Error> {
        match (c as u32).to_le_bytes() {
            [v, 0, 0, 0] => Self::encode(slice::from_ref(&v)),
            _ => Err(Error::InvalidChar),
        }
    }

    /// Decodes the name into a buffer.
    #[inline]
    pub const fn decode(self) -> DecodedName {
        let mut num = self.0.0.get();
        let mut buf = [0; Self::MAXLEN];
        let mut idx = buf.len() - 1;
        loop {
            buf[idx] = match (num % 37) as u8 {
                0 => b'_',
                n @ 1..27 => n + b'a' - 1,
                n @ 27..37 => n + b'0' - 27,
                _ => unreachable!(),
            };

            num /= 37;
            if num == 0 || idx == 0 {
                break;
            }

            idx -= 1;
        }

        DecodedName { buf, idx }
    }

    /// Returns the name length.
    #[inline]
    pub const fn len(self) -> usize {
        u128::ilog(self.0.0.get(), 37) as usize + 1
    }

    /// Names are always non-empty.
    #[inline]
    pub const fn is_empty(self) -> bool {
        false
    }
}

impl fmt::Display for Name {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.decode().as_str())
    }
}

impl fmt::Debug for Name {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.decode().as_str())
    }
}

impl TryFrom<&[u8]> for Name {
    type Error = Error;

    #[inline]
    fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
        Self::encode(s)
    }
}

impl TryFrom<&str> for Name {
    type Error = Error;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::encode(s.as_bytes())
    }
}

/// Return type from the [`decode`](Name::decode) method
/// used to obtain a slice or string.
#[derive(Clone, Copy, Debug)]
pub struct DecodedName {
    buf: [u8; Name::MAXLEN],
    idx: usize,
}

impl DecodedName {
    /// Returns a slice containing the decoded string.
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        let len = self.buf.len() - self.idx;

        // SAFETY:
        // * Invariant `idx <= buf.len()`, so `buf.as_ptr() + idx`
        //   in bounds or one element past of the buffer.
        let ptr = unsafe { self.buf.as_ptr().add(self.idx) };

        // SAFETY:
        // * All `buf` bytes are initialized.
        // * `ptr` in bounds of `buf`.
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// Returns the decoded string.
    #[inline]
    pub const fn as_str(&self) -> &str {
        // SAFETY:
        // * The invariant of the `Name` type is allowed to contain only
        //   letters, numbers and underscores, so all chars are valid utf8
        unsafe { str::from_utf8_unchecked(self.as_slice()) }
    }
}

impl AsRef<[u8]> for DecodedName {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsRef<str> for DecodedName {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// The [name](Name) creation error.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// The name is empty.
    Empty,

    /// The name's length is too long.
    TooLong,

    /// The name cannot contain a leading `_` char.
    LeadingUnderscore,

    /// The name contains an invalid char.
    InvalidChar,
}

impl Error {
    #[doc(hidden)]
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Empty => "the name is empty",
            Self::TooLong => "the name is too long",
            Self::LeadingUnderscore => "the name starts with `_`",
            Self::InvalidChar => "the name contains an invalid char",
        }
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl error::Error for Error {}

#[cfg(feature = "std")]
impl From<Error> for std::io::Error {
    #[inline]
    fn from(e: Error) -> Self {
        Self::new(std::io::ErrorKind::InvalidData, e.as_str())
    }
}

/// Creates a [name](Name) from the string.
/// Panics when name is [invalid](Error).
///
/// # Examples
///
/// This constructor can be called in a const context.
///
/// ```
/// let hello = const { ni::name!("hello_world") };
/// assert_eq!(hello.decode().as_str(), "hello_world");
/// ```
#[macro_export]
macro_rules! name {
    ($s:expr) => {{
        let s: &str = $s;
        match $crate::Name::encode(s.as_bytes()) {
            ::core::result::Result::Ok(name) => name,
            ::core::result::Result::Err(e) => ::core::panic!("{}", e.as_str()),
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code() -> Result<(), Error> {
        let s = b"hello";
        let name = Name::encode(s)?;
        assert_eq!(name.decode().as_slice(), s);
        assert_eq!(name.len(), 5);
        Ok(())
    }

    #[test]
    fn code_const() {
        let (string, orig) = const {
            let orig = "hellowo";
            let name = name!(orig);
            let string = name.decode();
            (string, orig)
        };

        assert_eq!(string.as_str(), orig);
    }

    #[test]
    fn cases() {
        let tests = [
            "a",
            "z",
            "0",
            "9",
            "a_",
            "99",
            "999",
            "999_",
            "hi",
            "some_key",
            "lol________",
            "0123456789",
            "abcdefxyz",
            "a1b2c3d_e_f9",
            "small_strings_rule",
            "123__qwe__456_xyz__098__",
        ];

        for test in tests {
            let name = name!(test);
            assert_eq!(name.decode().as_str(), test);
            assert_eq!(name.len(), test.len());
        }
    }

    #[test]
    fn code_long() -> Result<(), Error> {
        let s = b"999999999999999999999999";
        let name = Name::encode(s)?;
        assert_eq!(name.decode().as_slice(), s);
        assert_eq!(name.len(), 24);
        Ok(())
    }

    #[test]
    fn encode_char() -> Result<(), Error> {
        let name = Name::encode_char('a')?;
        assert_eq!(name.decode().as_str(), "a");
        assert_eq!(name.len(), 1);
        Ok(())
    }

    #[test]
    fn encode_invalid_char() {
        assert_eq!(Name::encode_char('ю'), Err(Error::InvalidChar));
    }

    #[test]
    fn zero_len() {
        let empty = b"";
        assert_eq!(Name::encode(empty), Err(Error::Empty));
    }

    #[test]
    fn too_long() {
        let long = b"9999999999999999999999999";
        assert_eq!(Name::encode(long), Err(Error::TooLong));
    }

    #[test]
    fn leading_underscore() {
        let underscored = b"_hello";
        assert_eq!(Name::encode(underscored), Err(Error::LeadingUnderscore));
    }

    #[test]
    fn invalid_char() {
        let s = b"HELLO";
        assert_eq!(Name::encode(s), Err(Error::InvalidChar));
    }
}
