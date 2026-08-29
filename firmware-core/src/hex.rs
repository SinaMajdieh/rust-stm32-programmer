use std::{fmt, fs, io, path::Path};

const DATA: u8 = 0x00;
const EOF: u8 = 0x01;
const EXTENDED_SEGMENT_ADDRESS: u8 = 0x02;
const START_SEGMENT_ADDRESS: u8 = 0x03;
const EXTENDED_LINEAR_ADDRESS: u8 = 0x04;
const START_LINEAR_ADDRESS: u8 = 0x05;

const TYPE_INDEX: usize = 3;

// length + address high + address low + type + checksum
const RECORD_OVERHEAD: usize = 5;

#[derive(Debug)]
pub enum HexError {
    Read(io::Error),
    Invalid { line: usize, message: &'static str },
    MissingEof,
}

impl fmt::Display for HexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read(error) => write!(f, "Could not read HEX file: {error}"),
            Self::Invalid { line, message } => {
                write!(f, "Invalid record on line {line}: {message}")
            }
            Self::MissingEof => write!(f, "Missing end-of-file record"),
        }
    }
}

impl std::error::Error for HexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read(error) => Some(error),
            _ => None,
        }
    }
}

pub fn validate_file(path: impl AsRef<Path>) -> Result<(), HexError> {
    let text = fs::read_to_string(path).map_err(HexError::Read)?;
    validate(&text)
}

fn validate(text: &str) -> Result<(), HexError> {
    let mut found_record = false;
    let mut found_eof = false;

    for (index, line) in text.lines().enumerate() {
        let line_number = index + 1;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if found_eof {
            return invalid(line_number, "data appears after the EOF record");
        }

        found_record = true;

        let record = validate_record(line).map_err(|message| HexError::Invalid {
            line: line_number,
            message,
        })?;

        found_eof = is_record_eof(&record);
    }

    if !found_record {
        return invalid(0, "file contains no records");
    }

    if !found_eof {
        return Err(HexError::MissingEof);
    }

    Ok(())
}

fn validate_record(line: &str) -> Result<Vec<u8>, &'static str> {
    let hex = line.strip_prefix(':').ok_or("record must start with ':'")?;

    if hex.len() % 2 != 0 {
        return Err("record has an odd number of hexadecimal digits");
    }

    let record = hex
        .as_bytes()
        .chunks_exact(2)
        .map(parse_byte)
        .collect::<Result<Vec<_>, _>>()?;

    if record.len() < RECORD_OVERHEAD {
        return Err("record is too short");
    }

    let data_length = record[0] as usize;

    if record.len() != data_length + RECORD_OVERHEAD {
        return Err("byte count does not match record length");
    }

    if record.iter().copied().fold(0, u8::wrapping_add) != 0 {
        return Err("checksum does not match");
    }

    let address = u16::from_be_bytes([record[1], record[2]]);
    let kind = record[TYPE_INDEX];

    validate_record_type(kind, data_length, address)?;

    Ok(record)
}

fn validate_record_type(kind: u8, length: usize, address: u16) -> Result<(), &'static str> {
    match kind {
        DATA => Ok(()),

        EOF if length == 0 && address == 0 => Ok(()),

        EXTENDED_SEGMENT_ADDRESS | EXTENDED_LINEAR_ADDRESS if length == 2 && address == 0 => Ok(()),

        START_SEGMENT_ADDRESS | START_LINEAR_ADDRESS if length == 4 && address == 0 => Ok(()),

        EOF => Err("EOF record must have address 0000 and no data"),

        EXTENDED_SEGMENT_ADDRESS | EXTENDED_LINEAR_ADDRESS => {
            Err("address record must have address 0000 and two data bytes")
        }

        START_SEGMENT_ADDRESS | START_LINEAR_ADDRESS => {
            Err("start-address record must have address 0000 and four data bytes")
        }

        _ => Err("unknown record type"),
    }
}

fn parse_byte(pair: &[u8]) -> Result<u8, &'static str> {
    let pair = std::str::from_utf8(pair).map_err(|_| "record contains invalid hexadecimal data")?;

    u8::from_str_radix(pair, 16).map_err(|_| "record contains invalid hexadecimal data")
}

fn is_record_eof(record: &[u8]) -> bool {
    record[TYPE_INDEX] == EOF
}

fn invalid<T>(line: usize, message: &'static str) -> Result<T, HexError> {
    Err(HexError::Invalid { line, message })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_hex() {
        let hex = "\
            :020000040800F2
            :0400000001020304F2
            :00000001FF
        ";

        assert!(validate(hex).is_ok());
    }

    #[test]
    fn rejects_bad_checksum() {
        let hex = "\
            :020000040800F2
            :0400000001020304F3
            :00000001FF
        ";

        assert!(matches!(
            validate(hex),
            Err(HexError::Invalid {
                message: "checksum does not match",
                ..
            })
        ));
    }

    #[test]
    fn requires_eof() {
        let hex = ":0400000001020304F2";

        assert!(matches!(validate(hex), Err(HexError::MissingEof)));
    }

    #[test]
    fn rejects_data_after_eof() {
        let hex = "\
            :00000001FF
            :0400000001020304F2
        ";

        assert!(matches!(
            validate(hex),
            Err(HexError::Invalid {
                message: "data appears after the EOF record",
                ..
            })
        ));
    }
}
