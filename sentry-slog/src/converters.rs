use sentry_core::protocol::{Breadcrumb, Event, Exception, Frame, Level, Map, Stacktrace, Value};
use slog::{Key, OwnedKVList, Record, Serializer, KV};
use std::fmt;

/// Converts a [`slog::Level`] to a Sentry [`Level`]
pub fn convert_log_level(level: slog::Level) -> Level {
    match level {
        slog::Level::Trace | slog::Level::Debug => Level::Debug,
        slog::Level::Info => Level::Info,
        slog::Level::Warning => Level::Warning,
        slog::Level::Error | slog::Level::Critical => Level::Error,
    }
}

struct MapSerializer<'a>(&'a mut Map<String, Value>);

macro_rules! impl_into {
    ($t:ty => $f:ident) => {
        fn $f(&mut self, key: Key, val: $t) -> slog::Result {
            self.0.insert(key.into(), val.into());
            Ok(())
        }
    };
}
impl<'a> Serializer for MapSerializer<'a> {
    fn emit_arguments(&mut self, key: Key, val: &fmt::Arguments) -> slog::Result {
        self.0.insert(key.into(), Value::String(format!("{}", val)));
        Ok(())
    }

    impl_into! { usize => emit_usize }
    impl_into! { isize => emit_isize }
    impl_into! { bool  => emit_bool  }
    impl_into! { u8    => emit_u8    }
    impl_into! { i8    => emit_i8    }
    impl_into! { u16   => emit_u16   }
    impl_into! { i16   => emit_i16   }
    impl_into! { u32   => emit_u32   }
    impl_into! { i32   => emit_i32   }
    impl_into! { f32   => emit_f32   }
    impl_into! { u64   => emit_u64   }
    impl_into! { i64   => emit_i64   }
    impl_into! { f64   => emit_f64   }
    impl_into! { &str  => emit_str   }
}

/// Adds the data from a [`slog::KV`] into a Sentry [`Map`].
fn add_kv_to_map(map: &mut Map<String, Value>, record: &Record, kv: &impl KV) {
    // TODO: Do something with these errors?
    let _ = record.kv().serialize(record, &mut MapSerializer(map));
    let _ = kv.serialize(record, &mut MapSerializer(map));
}

/// Creates a Sentry [`Breadcrumb`] from the [`Record`].
pub fn breadcrumb_from_record(record: &Record, values: &OwnedKVList) -> Breadcrumb {
    let mut data = Map::new();
    add_kv_to_map(&mut data, record, values);

    Breadcrumb {
        ty: "log".into(),
        message: Some(record.msg().to_string()),
        level: convert_log_level(record.level()),
        data,
        ..Default::default()
    }
}

/// Creates a simple message [`Event`] from the [`Record`].
pub fn event_from_record(record: &Record, values: &OwnedKVList) -> Event<'static> {
    let mut extra = Map::new();
    add_kv_to_map(&mut extra, record, values);
    Event {
        message: Some(record.msg().to_string()),
        level: convert_log_level(record.level()),
        ..Default::default()
    }
}

/// Creates an exception [`Event`] from the [`Record`].
///
/// The exception will have a stacktrace that corresponds to the location
/// information contained in the [`Record`].
///
/// # Examples
///
/// ```
/// let args = format_args!("");
/// let record = slog::record!(slog::Level::Error, "", &args, slog::b!());
/// let kv = slog::o!().into();
/// let event = sentry_slog::exception_from_record(&record, &kv);
///
/// let frame = &event.exception.as_ref()[0]
///     .stacktrace
///     .as_ref()
///     .unwrap()
///     .frames[0];
/// assert!(frame.lineno.unwrap() > 0);
/// ```
pub fn exception_from_record(record: &Record, values: &OwnedKVList) -> Event<'static> {
    let mut event = event_from_record(record, values);
    let frame = Frame {
        function: Some(record.function().into()),
        module: Some(record.module().into()),
        filename: Some(record.file().into()),
        lineno: Some(record.line().into()),
        colno: Some(record.column().into()),
        ..Default::default()
    };
    let exception = Exception {
        ty: "slog::Record".into(),
        stacktrace: Some(Stacktrace {
            frames: vec![frame],
            ..Default::default()
        }),
        ..Default::default()
    };
    event.exception = vec![exception].into();
    event
}
