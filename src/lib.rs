//! A LocalTime implementation to set timezone manually.

use std::fmt;
use std::io;
use time::{format_description::well_known, formatting::Formattable, OffsetDateTime, UtcOffset};
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

/// Formats the current [local time] using a [formatter] from the [`time` crate].
///
/// To format the current [UTC time] instead, use the [`UtcTime`] type.
///
/// [local time]: https://docs.rs/time/0.3/time/struct.OffsetDateTime.html#method.now_local
/// [UTC time]: https://docs.rs/time/0.3/time/struct.OffsetDateTime.html#method.now_utc
/// [formatter]: https://docs.rs/time/0.3/time/formatting/trait.Formattable.html
/// [`time` crate]: https://docs.rs/time/0.3/time/
#[derive(Clone, Debug)]
// #[cfg_attr(docsrs, doc(cfg(all(feature = "time", feature = "local-time"))))]
// #[cfg(feature = "local-time")]
pub struct LocalTime<F> {
    format: F,
    tz_hours: i8,
    tz_minutes: i8,
    tz_seconds: i8,
}

// === impl LocalTime ===

// #[cfg(feature = "local-time")]
impl LocalTime<well_known::Rfc3339> {
    /// Returns a formatter that formats the current [local time] in the
    /// [RFC 3339] format (a subset of the [ISO 8601] timestamp format).
    ///
    /// # Examples
    ///
    /// ```
    /// use tracing_subscriber::fmt::{self, time};
    ///
    /// let collector = tracing_subscriber::fmt()
    ///     .with_timer(time::LocalTime::rfc_3339());
    /// # drop(collector);
    /// ```
    ///
    /// [local time]: https://docs.rs/time/0.3/time/struct.OffsetDateTime.html#method.now_local
    /// [RFC 3339]: https://datatracker.ietf.org/doc/html/rfc3339
    /// [ISO 8601]: https://en.wikipedia.org/wiki/ISO_8601
    pub fn rfc_3339() -> Self {
        Self::new(well_known::Rfc3339)
    }
}

// #[cfg(feature = "local-time")]
impl<F: Formattable> LocalTime<F> {
    /// Returns a formatter that formats the current [local time] using the
    /// [`time` crate] with the provided provided format. The format may be any
    /// type that implements the [`Formattable`] trait.
    ///
    /// This default use UTC.
    ///
    /// Typically, the format will be a format description string, or one of the
    /// `time` crate's [well-known formats].
    ///
    /// If the format description is statically known, then the
    /// [`format_description!`] macro should be used. This is identical to the
    /// [`time::format_description::parse`] method, but runs at compile-time,
    /// throwing an error if the format description is invalid. If the desired format
    /// is not known statically (e.g., a user is providing a format string), then the
    /// [`time::format_description::parse`] method should be used. Note that this
    /// method is fallible.
    ///
    /// See the [`time` book] for details on the format description syntax.
    ///
    /// # Examples
    ///
    /// Using the [`format_description!`] macro:
    ///
    /// ```
    /// use tracing_subscriber::fmt::{self, time::LocalTime};
    /// use time::macros::format_description;
    ///
    /// let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));
    /// let collector = tracing_subscriber::fmt()
    ///     .with_timer(timer);
    /// # drop(collector);
    /// ```
    ///
    /// Using [`time::format_description::parse`]:
    ///
    /// ```
    /// use tracing_subscriber::fmt::{self, time::LocalTime};
    ///
    /// let time_format = time::format_description::parse("[hour]:[minute]:[second]")
    ///     .expect("format string should be valid!");
    /// let timer = LocalTime::new(time_format);
    /// let collector = tracing_subscriber::fmt()
    ///     .with_timer(timer);
    /// # drop(collector);
    /// ```
    ///
    /// Using the [`format_description!`] macro requires enabling the `time`
    /// crate's "macros" feature flag.
    ///
    /// Using a [well-known format][well-known formats] (this is equivalent to
    /// [`LocalTime::rfc_3339`]):
    ///
    /// ```
    /// use tracing_subscriber::fmt::{self, time::LocalTime};
    ///
    /// let timer = LocalTime::new(time::format_description::well_known::Rfc3339);
    /// let collector = tracing_subscriber::fmt()
    ///     .with_timer(timer);
    /// # drop(collector);
    /// ```
    ///
    /// [local time]: https://docs.rs/time/latest/time/struct.OffsetDateTime.html#method.now_local
    /// [`time` crate]: https://docs.rs/time/0.3/time/
    /// [`Formattable`]: https://docs.rs/time/0.3/time/formatting/trait.Formattable.html
    /// [well-known formats]: https://docs.rs/time/0.3/time/format_description/well_known/index.html
    /// [`format_description!`]: https://docs.rs/time/0.3/time/macros/macro.format_description.html
    /// [`time::format_description::parse`]: https://docs.rs/time/0.3/time/format_description/fn.parse.html
    /// [`time` book]: https://time-rs.github.io/book/api/format-description.html
    pub fn new(format: F) -> Self {
        Self {
            format,
            tz_hours: 0,
            tz_minutes: 0,
            tz_seconds: 0,
        }
    }

    /// New with a format and timezone setting.
    /// 
    /// Timezone format: (tz_hours, tz_minutes, tz_seconds)
    /// 
    /// # Examples:
    /// 
    /// ```
    ///     (8, 0, 0)
    ///     (-2, 30, 0)
    /// ```
    /// 
    pub fn with_timezone(format: F, tz_hms: (i8, i8, i8)) -> Self {
        Self {
            format,
            tz_hours: tz_hms.0,
            tz_minutes: tz_hms.1,
            tz_seconds: tz_hms.2,
        }
    }
}

// #[cfg(feature = "local-time")]
impl<F> FormatTime for LocalTime<F>
where
    F: Formattable,
{
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        //
        //
        //
        // Fix here:
        //
        //
        //

        // let now = OffsetDateTime::now_local().map_err(|_| fmt::Error)?;
        let offset = UtcOffset::from_hms(self.tz_hours, self.tz_minutes, self.tz_seconds)
            .unwrap_or(UtcOffset::UTC);
        let now = OffsetDateTime::now_utc().to_offset(offset);
        format_datetime(now, w, &self.format)
    }
}

// #[cfg(feature = "local-time")]
impl<F> Default for LocalTime<F>
where
    F: Formattable + Default,
{
    fn default() -> Self {
        Self::new(F::default())
    }
}

fn format_datetime(
    now: OffsetDateTime,
    into: &mut Writer<'_>,
    fmt: &impl Formattable,
) -> fmt::Result {
    let mut into = WriteAdaptor::new(into);
    now.format_into(&mut into, fmt)
        .map_err(|_| fmt::Error)
        .map(|_| ())
}

/// A bridge between `fmt::Write` and `io::Write`.
///
/// This is used by the timestamp formatting implementation for the `time`
/// crate and by the JSON formatter. In both cases, this is needed because
/// `tracing-subscriber`'s `FormatEvent`/`FormatTime` traits expect a
/// `fmt::Write` implementation, while `serde_json::Serializer` and `time`'s
/// `format_into` methods expect an `io::Write`.
// #[cfg(any(feature = "json", feature = "time"))]
pub(crate) struct WriteAdaptor<'a> {
    fmt_write: &'a mut dyn fmt::Write,
}

// === impl WriteAdaptor ===

// #[cfg(any(feature = "json", feature = "time"))]
impl<'a> WriteAdaptor<'a> {
    pub fn new(fmt_write: &'a mut dyn fmt::Write) -> Self {
        Self { fmt_write }
    }
}
// #[cfg(any(feature = "json", feature = "time"))]
impl<'a> io::Write for WriteAdaptor<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s =
            std::str::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        self.fmt_write
            .write_str(s)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(s.as_bytes().len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// #[cfg(any(feature = "json", feature = "time"))]
impl<'a> fmt::Debug for WriteAdaptor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("WriteAdaptor { .. }")
    }
}
// === blanket impls ===

#[cfg(test)]
mod tests {
    use super::LocalTime;
    use time::macros::format_description;

    #[test]
    fn test_init_tracing() {
        let timer = LocalTime::with_timezone(
            format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
            ),
            (8, 0, 0),
        );
        tracing_subscriber::fmt().with_timer(timer).init();
    }
}
