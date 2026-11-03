//! Shared Excel helpers (calamine).

use std::collections::HashMap;
use std::io::Cursor;

use calamine::{Data, DataType, Reader, Xlsx, open_workbook_from_rs};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExcelError {
    #[error("failed to open Excel workbook: {0}")]
    Open(String),
    #[error("Excel workbook has no worksheets")]
    EmptyWorkbook,
    #[error("failed to read worksheet: {0}")]
    Sheet(String),
    #[error("{0}")]
    Message(String),
}

pub type RowMap = HashMap<String, Data>;

/// Read A1 (first cell of first row) as a string when present.
pub fn read_a1_string(data: &[u8]) -> Result<Option<String>, ExcelError> {
    let mut workbook = open_xlsx(data)?;
    let first = first_sheet_name(&workbook)?;
    let range = workbook
        .worksheet_range(&first)
        .map_err(|e| ExcelError::Sheet(e.to_string()))?;
    let Some(row) = range.rows().next() else {
        return Ok(None);
    };
    Ok(cell_as_string(row.first().unwrap_or(&Data::Empty)))
}

/// Read rows after skipping `skip_rows`, using the next row as headers.
pub fn read_table(data: &[u8], skip_rows: usize) -> Result<(Vec<String>, Vec<RowMap>), ExcelError> {
    let mut workbook = open_xlsx(data)?;
    let first = first_sheet_name(&workbook)?;
    let range = workbook
        .worksheet_range(&first)
        .map_err(|e| ExcelError::Sheet(e.to_string()))?;

    let mut rows = range.rows();
    for _ in 0..skip_rows {
        rows.next();
    }
    let header_row = rows
        .next()
        .ok_or_else(|| ExcelError::Message("Excel has no header row after skip_rows".into()))?;
    let headers: Vec<String> = header_row
        .iter()
        .map(|cell| normalize_header(&cell_as_string(cell).unwrap_or_default()))
        .collect();

    let mut out = Vec::new();
    for row in rows {
        if row.iter().all(DataType::is_empty) {
            continue;
        }
        let mut map = HashMap::new();
        for (idx, header) in headers.iter().enumerate() {
            if header.is_empty() {
                continue;
            }
            let value = row.get(idx).cloned().unwrap_or(Data::Empty);
            map.insert(header.clone(), value);
        }
        out.push(map);
    }
    Ok((headers, out))
}

fn open_xlsx(data: &[u8]) -> Result<Xlsx<Cursor<&[u8]>>, ExcelError> {
    open_workbook_from_rs::<Xlsx<_>, _>(Cursor::new(data))
        .map_err(|e| ExcelError::Open(e.to_string()))
}

fn first_sheet_name(workbook: &Xlsx<Cursor<&[u8]>>) -> Result<String, ExcelError> {
    workbook
        .sheet_names()
        .first()
        .cloned()
        .ok_or(ExcelError::EmptyWorkbook)
}

pub fn normalize_header(value: &str) -> String {
    value
        .trim()
        .replace('\u{00a0}', " ")
        .replace('\t', "")
        .to_lowercase()
        .replace(' ', "_")
}

pub fn cell_as_string(cell: &Data) -> Option<String> {
    match cell {
        Data::Empty => None,
        Data::String(s) => {
            let t = s.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_string())
            }
        }
        Data::Int(v) => Some(v.to_string()),
        Data::Float(v) => Some(format_number(*v)),
        Data::Bool(v) => Some(v.to_string()),
        Data::DateTime(dt) => dt
            .as_datetime()
            .map(|ndt| ndt.format("%Y-%m-%d %H:%M:%S").to_string()),
        Data::DateTimeIso(s) | Data::DurationIso(s) => Some(s.clone()),
        Data::Error(err) => Some(format!("{err:?}")),
    }
}

pub fn cell_as_f64(cell: &Data) -> Option<f64> {
    cell.as_f64().or_else(|| match cell {
        Data::String(s) => {
            let cleaned = s.trim().replace(',', "");
            cleaned.parse::<f64>().ok()
        }
        _ => None,
    })
}

pub fn cell_as_i64(cell: &Data) -> Option<i64> {
    cell.as_i64().or_else(|| {
        cell_as_f64(cell).and_then(|v| {
            if v.is_finite() {
                Some(v.round() as i64)
            } else {
                None
            }
        })
    })
}

pub fn cell_as_datetime(cell: &Data) -> Option<NaiveDateTime> {
    cell.as_datetime().or_else(|| match cell {
        Data::String(s) => parse_flexible_datetime(s),
        Data::Float(v) => excel_serial_to_datetime(*v),
        Data::Int(v) => excel_serial_to_datetime(*v as f64),
        Data::DateTimeIso(s) => parse_flexible_datetime(s),
        _ => None,
    })
}

fn format_number(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{}", v as i64)
    } else {
        v.to_string()
    }
}

fn excel_serial_to_datetime(serial: f64) -> Option<NaiveDateTime> {
    let days = serial.floor() as i64;
    let day =
        NaiveDate::from_ymd_opt(1899, 12, 30)?.checked_add_signed(chrono::Duration::days(days))?;
    let frac = serial - serial.floor();
    let secs = (frac * 86400.0).round() as i64;
    let time = NaiveTime::from_num_seconds_from_midnight_opt((secs.rem_euclid(86400)) as u32, 0)?;
    Some(NaiveDateTime::new(day, time))
}

fn parse_flexible_datetime(raw: &str) -> Option<NaiveDateTime> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    const FORMATS: &[&str] = &[
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d",
        "%d-%m-%Y %H:%M:%S",
        "%d/%m/%Y %H:%M:%S",
        "%m/%d/%Y %H:%M:%S",
        "%d-%m-%Y",
        "%d/%m/%Y",
    ];
    for fmt in FORMATS {
        if let Ok(dt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(dt);
        }
        if let Ok(d) = NaiveDate::parse_from_str(s, fmt) {
            return Some(d.and_hms_opt(0, 0, 0)?);
        }
    }
    None
}

pub fn get_cell<'a>(row: &'a RowMap, key: &str) -> Option<&'a Data> {
    row.get(key)
}
