use chrono::{DateTime, FixedOffset, TimeZone};
use exif::{Exif, In, Tag, Value};
use std::fmt;
use std::fs;
use std::io;
use std::os::unix::prelude::MetadataExt;
use std::{error::Error, path::Path};
use std::fmt::Display;
use std::fs::metadata;

#[derive(Debug)]
pub struct Metadata {
    pub date_time_created: DateTime<FixedOffset>,
    pub date_time_taken: Option<DateTime<FixedOffset>>,
}

#[derive(Debug)]
pub struct MetadataError {
    message: String,
}

impl MetadataError {
    fn from_str(message: &str) -> Self {
        MetadataError {
            message: message.to_string(),
        }
    }

    fn from_string(message: String) -> Self {
        MetadataError { message }
    }
}

// Display implementation is required for std::error::Error.
impl Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Metadata error: {}", self.message)
    }
}

impl Error for MetadataError {}

impl From<exif::Error> for MetadataError {
    fn from(item: exif::Error) -> Self {
        MetadataError::from_string(item.to_string())
    }
}

impl From<io::Error> for MetadataError {
    fn from(item: io::Error) -> Self {
        MetadataError::from_string(item.to_string())
    }
}

fn result_to_option<T, E: Error>(res: Result<T, E>, checkpoint: String) -> Option<T> {
    match res {
        Ok(t) => Some(t),
        Err(e) => {
            eprint!("Error occurred during {}: {}", checkpoint, e)
        }
    }
}

fn open_file(path: &Path) -> Option<fs::File> {
    result_to_option(fs::File::open(path), "opening file " + path.display())
}

fn get_date_time_created(path: &Path) -> Result<DateTime<FixedOffset>, MetadataError> {
    Ok(FixedOffset::west(0).timestamp(metadata(path)?.ctime(), 0))
}

fn get_exif_metadata(file: fs::File, file_name: &String) -> Option<Exif> {
    let mut bufreader = io::BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    result_to_option(
        exif_reader.read_from_container(&mut bufreader),
        "extracting exif metadata for " + file_name
    )
}

fn get_exif_date_time_field(exif: Exif, tag: Tag, file_name: &String)
                            -> Option<exif::DateTime> {
   exif.get_field(tag, In::PRIMARY)
        .map(|field| {
            match field.value {
                Value::Ascii(ref v) => result_to_option(
                    exif::DateTime::from_ascii(&v[0]),
                    "converting exif field " + tag + " to date time"
                ),
                _ => {
                    eprintln!("Exif field {} is not ascii in file {}", tag, file_name);
                    None
                }
            }
        }).flatten()
}

fn from_exif_to_chrono_date_time(exif_fate_time: exif::DateTime, file_name: &String)
    -> Option<DateTime<FixedOffset>> {

    let offset = match exif_fate_time.offset {
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
        chrono::LocalResult::Single(date) => Some(date),
        chrono::LocalResult::Ambiguous(_, _) => {
            eprintln!("Date time {} in {} was ambiguous", file_name, exif_fate_time);
            None
        },
        chrono::LocalResult::None => {
            eprintln!("Date time {} in {} was invalid", file_name, exif_fate_time);
            None
        }
    }
}

fn get_date_time_taken(path: &Path) -> Option<DateTime<FixedOffset>> {
    let file_name = path.display().to_string();
    open_file(path)
        .map(|file| get_exif_metadata(file, &file_name)).flatten()
        .map(|exif|
            get_exif_date_time_field(exif, Tag::DateTimeOriginal, &file_name)
        ).flatten()
        .map(|exif_date_time|
            from_exif_to_chrono_date_time(exif_date_time, &file_name)
        ).flatten()
}

pub fn read_metadata(path: &Path) -> Result<Metadata, MetadataError> {
    let date_time_created = get_date_time_created(path)?;
    let date_time_taken = get_date_time_taken(path);
    Ok(Metadata {
        date_time_created,
        date_time_taken,
    })
}
