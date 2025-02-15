use gtk::gio::FileType;

pub trait Display {
    fn as_str(&self) -> &str;
}

impl Display for FileType {
    fn as_str(&self) -> &str {
        match self {
            FileType::Unknown => "Unknown",
            FileType::Regular => "File",
            FileType::Directory => "Directory",
            FileType::SymbolicLink => "SymbolicLink",
            FileType::Special => "Special",
            FileType::Shortcut => "Shortcut",
            FileType::Mountable => "Mountable",
            _ => "Undefined",
        }
    }
}
