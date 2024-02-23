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

/// Error enumerates all errors returned by metadata module.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Representation has multiple results and thus ambiguous.
    #[error("Ambiguous date. Received: {0} and {1}")]
    AmbiguousDate(String, String),

    /// Representation has invalid date.
    #[error("Could not parse the date: {0}")]
    InvalidDate(String),

    /// Catch all non ascii Value variants.
    #[error("Exif date field is not Ascii")]
    ExifDateNotAscii(String),

    /// Failed to convert exif DateTime to one from chrono.
    #[error("Failed to convert to chrono DateTime: {err} found in this path: {path}")]
    ChronoConvert { path: String, err: String },

    /// Forward all errors returned by exif crate.
    #[error(transparent)]
    ExifError(#[from] exif::Error),

    /// Forward underlying `std::io` errors.
    #[error(transparent)]
    IOError(#[from] io::Error),
}

fn get_date_time_created(path: &Path) -> Result<chrono::DateTime<FixedOffset>, Error> {
    Ok(FixedOffset::west(0).timestamp(fs::metadata(path)?.ctime(), 0))
}

fn convert_exif_date_time_to_chrono_date_time_fixed_offset(
    exif_date_time: exif::DateTime,
) -> Result<chrono::DateTime<FixedOffset>, Error> {
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
        chrono::LocalResult::Ambiguous(date1, date2) => {
            Err(Error::AmbiguousDate(date1.to_string(), date2.to_string()))
        }
        chrono::LocalResult::None => Err(Error::InvalidDate(exif_date_time.to_string())),
    }
}

fn extract_date_time_exif_field(
    path: &Path,
    exif: &Exif,
    tag: Tag,
) -> Result<Option<chrono::DateTime<FixedOffset>>, Error> {
    match exif.get_field(tag, In::PRIMARY) {
        Some(field) => match field.value {
            Value::Ascii(ref v) => match convert_exif_date_time_to_chrono_date_time_fixed_offset(
                exif::DateTime::from_ascii(&v[0])?,
            ) {
                Ok(date_time) => Ok(Some(date_time)),
                Err(err) => Err(Error::ChronoConvert {
                    err: err.to_string(),
                    path: String::from(path.to_string_lossy()),
                }),
            },
            _ => Err(Error::ExifDateNotAscii(String::from(
                path.to_string_lossy(),
            ))),
        },
        None => Ok(None),
    }
}

pub fn read_metadata(path: &Path) -> Result<Metadata, Error> {
    let date_time_created = get_date_time_created(path)?;

    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();

    let date_time_taken = match exifreader.read_from_container(&mut bufreader) {
        Ok(exif) => extract_date_time_exif_field(path, &exif, Tag::DateTimeOriginal)?,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_date_time_convert() -> Result<(), Error> {
        let exif_date_time = exif::DateTime {
            year: 2019,
            month: 2,
            day: 10,
            hour: 13,
            minute: 11,
            second: 51,
            nanosecond: None,
            offset: None,
        };
        let chrono_datetime =
            convert_exif_date_time_to_chrono_date_time_fixed_offset(exif_date_time);
        assert_eq!(
            chrono_datetime?.to_string(),
            "2019-02-10 13:11:51 +00:00".to_string()
        );
        Ok(())
    }
}
