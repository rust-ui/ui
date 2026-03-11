use std::{fmt::Display, sync::atomic::AtomicUsize};

const BYTES_LEN: usize = 64;
pub type Bytes = [u8; BYTES_LEN];

/// A ToastId is basically a simple wrapper around a [u8; 64] which makes it easy to use strings and integers as Id's
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ToastId(Bytes);

fn usize_to_u8_array(mut number: usize) -> Bytes {
    let mut result = [0u8; BYTES_LEN];

    for i in (0..BYTES_LEN).rev() {
        result[i] = (number & 0xFF) as u8;
        number >>= 8;
    }

    result
}

impl ToastId {
    pub fn to_decodable_string(&self) -> String {
        self.0.map(|b| b.to_string()).join(",")
    }

    pub fn decode_string(s: &str) -> Self {
        let mut bytes = [0; BYTES_LEN];
        for (index, split) in s.split(',').enumerate() {
            if index >= BYTES_LEN {
                break;
            }
            bytes[index] = split.parse::<u8>().unwrap();
        }

        ToastId(bytes)
    }
}

static TOAST_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub enum ToastIdFromStrError {
    StrTooLong,
}

impl Display for ToastIdFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToastIdFromStrError::StrTooLong => write!(f, "Could not parse the str to a ToastId since the string was too long. The str should have at most {} bytes", BYTES_LEN),
        }
    }
}

impl std::error::Error for ToastIdFromStrError {}

impl ToastId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let id = TOAST_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        ToastId(usize_to_u8_array(id))
    }

    pub fn from_usize(number: usize) -> Self {
        ToastId(usize_to_u8_array(number))
    }

    /// Tries to parse the string to a ToastId
    /// This can fail if the string is too long: The string should be at most 64 long
    pub fn try_from_str(s: &str) -> Result<Self, ToastIdFromStrError> {
        if s.bytes().len() > BYTES_LEN {
            return Err(ToastIdFromStrError::StrTooLong);
        }

        Ok(Self::from_str_truncated(s))
    }

    /// Parses the str to a ToastId but truncates it if it is too long
    pub fn from_str_truncated(s: &str) -> Self {
        let bytes = s.bytes();
        let mut toast_id_bytes: Bytes = [0; BYTES_LEN];
        for (index, byte) in bytes.enumerate() {
            if index >= BYTES_LEN {
                break;
            }
            // Fill in from the back
            toast_id_bytes[BYTES_LEN - index - 1] = byte;
        }

        ToastId(toast_id_bytes)
    }
}
