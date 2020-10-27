/// Messages for task communication
#[derive(Debug)]
pub enum Events {
    /// A new tag is in the field of the tag reader
    NewTag(mfrc522::Uid),
    /// A button has been pressed long (>1s)
    ButtonPressedShort(Button),
    /// A button has been pressed short
    ButtonPressedLong(Button),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
/// Buttons that can be pressed
pub enum Button {
    PlayPause,
    Up,
    Down,
}
