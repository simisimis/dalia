use chrono::{DateTime, FixedOffset, TimeZone};
use exif::{Exif, In, Tag, Value};
use std::os::unix::prelude::MetadataExt;
use std::path::Path;
use std::{fmt, fs, io};

#[derive(Debug)]
pub struct Metadata {
    pub date_time_created: DateTime<FixedOffset>,
    pub date_time_taken: Option<DateTime<FixedOffset>>,
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Date time created: {}", self.date_time_created)?;
        match self.date_time_taken {
            Some(taken) => write!(f, "Date time taken: {}", taken),
            None => write!(f, "Date time taken: unknown"),
        }
    }
}

/// MetadataError enumerates all errors returned by metadata module.
#[derive(thiserror::Error, Debug)]
pub enum MetadataError {
    /// Representation has multiple results and thus ambiguous.
    #[error("Ambiguous date")]
    AmbiguousDate,

    /// Representation has invalid date.
    #[error("Invalid date")]
    InvalidDate,

    /// Catch all errors when trying to parse date from exif.
    #[error("Exif date field is not Ascii")]
    ExifDateNotAscii,

    /// Forward all errors returned by exif crate.
    #[error(transparent)]
    ExifError(#[from] exif::Error),

    /// Forward underlying `std::io` errors.
    #[error(transparent)]
    IOError(#[from] io::Error),
}

fn get_date_time_created(path: &Path) -> Result<chrono::DateTime<FixedOffset>, MetadataError> {
    Ok(FixedOffset::west(0).timestamp(fs::metadata(path)?.ctime(), 0))
}

fn convert_exif_date_time_to_chrono_date_time_fixed_offset(
    exif_date_time: exif::DateTime,
) -> Result<chrono::DateTime<FixedOffset>, MetadataError> {
    let offset = match exif_date_time.offset {
        Some(offset_minutes) => FixedOffset::west((offset_minutes * 60).into()),
        None => FixedOffset::west(0),
    };

    let maybe_date = offset
        .ymd_opt(
            exif_date_time.year.into(),
            exif_date_time.month.into(),
            exif_date_time.day.into(),
        )
        .and_hms_nano_opt(
            exif_date_time.hour.into(),
            exif_date_time.minute.into(),
            exif_date_time.second.into(),
            exif_date_time.nanosecond.unwrap_or_default(),
        );

    match maybe_date {
        chrono::LocalResult::Single(date) => Ok(date),
        chrono::LocalResult::Ambiguous(_, _) => Err(MetadataError::AmbiguousDate),
        chrono::LocalResult::None => Err(MetadataError::InvalidDate),
    }
}

fn extract_date_time_exif_field(
    exif: &Exif,
    tag: Tag,
) -> Result<Option<chrono::DateTime<FixedOffset>>, MetadataError> {
    match exif.get_field(tag, In::PRIMARY) {
        Some(field) => match field.value {
            Value::Ascii(ref v) => match convert_exif_date_time_to_chrono_date_time_fixed_offset(
                exif::DateTime::from_ascii(&v[0])?,
            ) {
                Ok(date_time) => Ok(Some(date_time)),
                Err(err) => Err(err),
            },
            _ => Err(MetadataError::ExifDateNotAscii),
        },
        None => Ok(None),
    }
}

pub fn read_metadata(path: &Path) -> Result<Metadata, MetadataError> {
    let date_time_created = get_date_time_created(path)?;

    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();

    let date_time_taken = match exifreader.read_from_container(&mut bufreader) {
        Ok(exif) => extract_date_time_exif_field(&exif, Tag::DateTimeOriginal)?,
        Err(err) => {
            log::debug!("Could not read EXIF data: {}", err);
            None
        }
    };

    Ok(Metadata {
        date_time_created,
        date_time_taken,
    })
}
