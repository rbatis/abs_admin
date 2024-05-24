use fast_log::{appender::{Command, FastLogRecord, RecordFormat}, TimeType};
use log::LevelFilter;


pub struct FastLogFormat {
    // show line level
    pub display_line_level: LevelFilter,
    pub time_type: TimeType,
}

impl RecordFormat for FastLogFormat {
    fn do_format(&self, arg: &mut FastLogRecord) {
        match &arg.command {
            Command::CommandRecord => {
                let now = match self.time_type {
                    TimeType::Local => fastdate::DateTime::from(arg.now)
                        .set_offset(fastdate::offset_sec())
                        .display_stand(),
                    TimeType::Utc => fastdate::DateTime::from(arg.now).display_stand(),
                };
                // 调整line的输出位置
                if arg.level.to_level_filter() <= self.display_line_level {
                    arg.formated = format!(
                        "{:27} [{}] {}\n [{}:{}]\n",
                        &now,
                        arg.level,
                        arg.args,
                        arg.file,
                        arg.line.unwrap_or_default(),
                    );
                } else {
                    // 增加输出target
                    arg.formated = format!("{:27} [{}] [{}] {}\n", &now, arg.level, arg.target, arg.args);
                }
            }
            Command::CommandExit => {}
            Command::CommandFlush(_) => {}
        }
    }
}
#[allow(dead_code)]
impl FastLogFormat {
    pub fn new() -> FastLogFormat {
        Self {
            display_line_level: LevelFilter::Warn,
            time_type: TimeType::default(),
        }
    }

    ///show line level
    pub fn set_display_line_level(mut self, level: LevelFilter) -> Self {
        self.display_line_level = level;
        self
    }

    /// set time_type
    pub fn set_time_type(mut self, time_type: TimeType) -> Self {
        self.time_type = time_type;
        self
    }
}