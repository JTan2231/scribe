use std::io::Write;
use std::sync::Once;

// TODO: thread safety?

static mut WORKSPACE_INSTANCE: Option<Workspace> = None;
static WORKSPACE_INIT: Once = Once::new();

pub struct Workspace {
    root_dir: std::path::PathBuf,
}

impl Workspace {
    pub fn new(root_dir: &str) -> &'static Workspace {
        unsafe {
            WORKSPACE_INIT.call_once(|| {
                let root_dir = std::path::PathBuf::from(root_dir);
                WORKSPACE_INSTANCE = Some(Workspace { root_dir });
            });

            WORKSPACE_INSTANCE.as_ref().unwrap()
        }
    }

    pub fn config_dir(&self) -> std::path::PathBuf {
        self.root_dir.join("config")
    }

    pub fn local_dir(&self) -> std::path::PathBuf {
        self.root_dir.join("state")
    }

    pub fn data_dir(&self) -> std::path::PathBuf {
        self.local_dir().join("data")
    }

    pub fn root_dir(&self) -> &std::path::PathBuf {
        &self.root_dir
    }
}

pub fn get_root_dir() -> std::path::PathBuf {
    Workspace::new("").root_dir().clone()
}

pub fn get_config_dir() -> std::path::PathBuf {
    Workspace::new("").config_dir()
}

pub fn get_local_dir() -> std::path::PathBuf {
    Workspace::new("").local_dir()
}

pub fn get_data_dir() -> std::path::PathBuf {
    Workspace::new("").data_dir()
}

pub struct Logger {
    file: Box<dyn std::io::Write>,
}

static mut INSTANCE: Option<Logger> = None;
static INIT: Once = Once::new();

#[allow(static_mut_refs)]
impl Logger {
    pub fn init(filename: Option<&str>) -> &'static Logger {
        unsafe {
            INIT.call_once(|| {
                INSTANCE = Some(Logger {
                    file: match filename {
                        Some(fname) => {
                            println!("Initializing logging to {}", fname);
                            Box::new(
                                std::fs::OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(fname)
                                    .expect("Failed to open log file"),
                            )
                        }
                        None => {
                            println!("No log file given; logging to stdout");
                            Box::new(std::io::stdout())
                        }
                    },
                });
            });

            INSTANCE.as_ref().unwrap()
        }
    }

    fn write(message: String) {
        unsafe {
            if INSTANCE.is_none() {
                panic!("Logger not initialized");
            }

            let file = &mut INSTANCE.as_mut().unwrap().file;
            writeln!(file, "{}", message).expect("Failed to write to log");
            file.flush().expect("Failed to flush file");
        }
    }

    pub fn info(message: String) {
        let message = format!("{} [INFO]: {}", chrono::Local::now(), message);
        Logger::write(message);
    }

    pub fn error(message: String) {
        let message = format!("{} [ERROR]: {}", chrono::Local::now(), message);
        Logger::write(message);
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        Logger::info(format!($($arg)*));
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        Logger::error(format!($($arg)*));
    }
}

#[macro_export]
macro_rules! lprint {
    ($method:tt, $($arg:tt)*) => {
        println!("{}", format!($($arg)*));
        Logger::$method(format!($($arg)*));
    }
}
