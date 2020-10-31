/// Messages for task communication
#[derive(Debug)]
pub enum Events {
    /// A new tag is in the field of the tag reader
    NewTag(Card),
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Card {
    cookie: u32,
    version: u8,
    folder: u8,
    mode: u8,
    special1: u8,
    special2: u8,
}

impl core::convert::TryFrom<[u8; 16]> for Card {
    type Error = ();

    fn try_from(value: [u8; 16]) -> Result<Self, Self::Error> {
        // Check the cookie
        let cookie = u32::from_be_bytes([value[0], value[1], value[2], value[3]]);
        if cookie != 0x1337B347 {
            return Err(());
        }

        // Encode the rest
        let version = value[4];
        let folder = value[5];
        let mode = value[6];
        let special1 = value[7];
        let special2 = value[8];

        Ok(Self {
            cookie,
            version,
            folder,
            mode,
            special1,
            special2,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Modus {
    /// Random track from folder
    RandomSingle(u8),
    /// Full Folder in normal order
    AlbumNormal(u8),
    /// Full Folder in random order
    AlbumShuffel(u8),
    /// Single Track from folder
    Single(u8, u8),
    /// Full Folder Safe progress
    /// TODO: Safe not implemented
    AlbumSave(u8),
    /// Play random single Track start to end
    RandomStartToEndSingle(u8, u8, u8),
    /// Play all Tracks from start to end
    StartToEndAlbum(u8, u8, u8),
    /// Play random tracks from start to end
    RandomStartToEnd(u8, u8, u8),
}

impl core::convert::TryFrom<Card> for Modus {
    type Error = ();

    fn try_from(value: Card) -> Result<Self, Self::Error> {
        Modus::try_from((value.mode, value.folder, value.special1, value.special2))
    }
}

impl core::convert::TryFrom<(u8, u8, u8, u8)> for Modus {
    type Error = ();

    fn try_from(value: (u8, u8, u8, u8)) -> Result<Self, Self::Error> {
        match value {
            (0x01, folder, _, _) => Ok(Modus::RandomSingle(folder)),
            (0x02, folder, _, _) => Ok(Modus::AlbumNormal(folder)),
            (0x03, folder, _, _) => Ok(Modus::AlbumShuffel(folder)),
            (0x04, folder, track, _) => Ok(Modus::Single(folder, track)),
            (0x05, folder, _, _) => Ok(Modus::AlbumSave(folder)),
            (0x07, folder, start, end) => Ok(Modus::RandomStartToEndSingle(folder, start, end)),
            (0x08, folder, start, end) => Ok(Modus::StartToEndAlbum(folder, start, end)),
            (0x09, folder, start, end) => Ok(Modus::RandomStartToEnd(folder, start, end)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Modifyer {
    None,
    AdminMenu,
    SleepTimer(u8),
    FreezeDance,
    Locked,
    Toddler,
    Kindergarden,
    RepeatSingle,
}

impl core::convert::TryFrom<Card> for Modifyer {
    type Error = ();

    fn try_from(value: Card) -> Result<Self, Self::Error> {
        Modifyer::try_from((value.mode, value.folder, value.special1, value.special2))
    }
}

impl core::convert::TryFrom<(u8, u8, u8, u8)> for Modifyer {
    type Error = ();

    fn try_from(value: (u8, u8, u8, u8)) -> Result<Self, Self::Error> {
        match value {
            (0x00, _, _, _) => Ok(Modifyer::None),
            (0xff, _, _, _) => Ok(Modifyer::AdminMenu),
            (0x01, _, min, _) => Ok(Modifyer::SleepTimer(min)),
            (0x02, _, _, _) => Ok(Modifyer::FreezeDance),
            (0x03, _, _, _) => Ok(Modifyer::Locked),
            (0x04, _, _, _) => Ok(Modifyer::Toddler),
            (0x05, _, _, _) => Ok(Modifyer::Kindergarden),
            (0x06, _, _, _) => Ok(Modifyer::RepeatSingle),
            _ => Err(()),
        }
    }
}
