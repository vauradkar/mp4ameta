use std::convert::TryFrom;
use std::fmt;
use std::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::ErrorKind;

// iTunes media type indices
/// A media type code stored in the `stik` atom.
const MOVIE: u8 = 0;
/// A media type code stored in the `stik` atom.
const NORMAL: u8 = 1;
/// A media type code stored in the `stik` atom.
const AUDIOBOOK: u8 = 2;
/// A media type code stored in the `stik` atom.
const WHACKED_BOOKMARK: u8 = 5;
/// A media type code stored in the `stik` atom.
const MUSIC_VIDEO: u8 = 6;
/// A media type code stored in the `stik` atom.
const SHORT_FILM: u8 = 9;
/// A media type code stored in the `stik` atom.
const TV_SHOW: u8 = 10;
/// A media type code stored in the `stik` atom.
const BOOKLET: u8 = 11;

// iTunes advisory rating indices
/// An advisory rating code stored in the `rtng` atom.
const CLEAN: u8 = 2;
/// An advisory rating code stored in the `rtng` atom.
const INOFFENSIVE: u8 = 0;
/// An advisory rating code stored in the `rtng` atom.
const EXPLICIT: u8 = 4;

// channnel configuration indices
/// Mono
const MONO: u8 = 1;
/// Stereo
const STEREO: u8 = 2;
/// 3.0
const THREE: u8 = 3;
/// 4.0
const FOUR: u8 = 4;
/// 5.0
const FIVE: u8 = 5;
/// 5.1
const FIVE_ONE: u8 = 6;
/// 7.1
const SEVEN_ONE: u8 = 7;

// sample rate indices
/// Sample rate index for 96000Hz.
const HZ_96000: u8 = 0;
/// Sample rate index for 882000Hz.
const HZ_88200: u8 = 1;
/// Sample rate index for 640000Hz.
const HZ_64000: u8 = 2;
/// Sample rate index for 480000Hz.
const HZ_48000: u8 = 3;
/// Sample rate index for 44100Hz.
const HZ_44100: u8 = 4;
/// Sample rate index for 32000Hz.
const HZ_32000: u8 = 5;
/// Sample rate index for 242000Hz.
const HZ_24000: u8 = 6;
/// Sample rate index for 22050Hz.
const HZ_22050: u8 = 7;
/// Sample rate index for 16000Hz.
const HZ_16000: u8 = 8;
/// Sample rate index for 12000Hz.
const HZ_12000: u8 = 9;
/// Sample rate index for 11025Hz.
const HZ_11025: u8 = 10;
/// Sample rate index for 8000Hz.
const HZ_8000: u8 = 11;
/// Sample rate index for 7350Hz.
const HZ_7350: u8 = 12;

/// An enum describing the media type of a file stored in the `stik` atom.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MediaType {
    /// A media type stored as 0 in the `stik` atom.
    Movie,
    /// A media type stored as 1 in the `stik` atom.
    Normal,
    /// A media type stored as 2 in the `stik` atom.
    AudioBook,
    /// A media type stored as 5 in the `stik` atom.
    WhackedBookmark,
    /// A media type stored as 6 in the `stik` atom.
    MusicVideo,
    /// A media type stored as 9 in the `stik` atom.
    ShortFilm,
    /// A media type stored as 10 in the `stik` atom.
    TvShow,
    /// A media type stored as 11 in the `stik` atom.
    Booklet,
}

impl MediaType {
    /// Returns the media type code.
    pub(crate) fn code(&self) -> u8 {
        match self {
            Self::Movie => MOVIE,
            Self::Normal => NORMAL,
            Self::AudioBook => AUDIOBOOK,
            Self::WhackedBookmark => WHACKED_BOOKMARK,
            Self::MusicVideo => MUSIC_VIDEO,
            Self::ShortFilm => SHORT_FILM,
            Self::TvShow => TV_SHOW,
            Self::Booklet => BOOKLET,
        }
    }
}

impl TryFrom<u8> for MediaType {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            MOVIE => Ok(Self::Movie),
            NORMAL => Ok(Self::Normal),
            AUDIOBOOK => Ok(Self::AudioBook),
            WHACKED_BOOKMARK => Ok(Self::WhackedBookmark),
            MUSIC_VIDEO => Ok(Self::MusicVideo),
            SHORT_FILM => Ok(Self::ShortFilm),
            TV_SHOW => Ok(Self::TvShow),
            BOOKLET => Ok(Self::Booklet),
            _ => Err(Self::Error::new(
                ErrorKind::UnknownMediaType(value),
                "Unknown media type".to_owned(),
            )),
        }
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Movie => write!(f, "Movie"),
            Self::Normal => write!(f, "Normal"),
            Self::AudioBook => write!(f, "Audiobook"),
            Self::WhackedBookmark => write!(f, "Whacked Bookmark"),
            Self::MusicVideo => write!(f, "Music Video"),
            Self::ShortFilm => write!(f, "Short Film"),
            Self::TvShow => write!(f, "TV-Show"),
            Self::Booklet => write!(f, "Booklet"),
        }
    }
}

/// An enum describing the rating of a file stored in the `rtng` atom.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdvisoryRating {
    /// An advisory rating stored as 2 in the `rtng` atom.
    Clean,
    /// An advisory rating stored as 0 in the `rtng` atom.
    Inoffensive,
    /// An advisory rating indicated by any other value than 0 or 2 in the `rtng` atom, containing
    /// the value.
    Explicit,
}

impl AdvisoryRating {
    /// Returns the advisory rating code.
    pub(crate) fn code(&self) -> u8 {
        match self {
            Self::Clean => CLEAN,
            Self::Inoffensive => INOFFENSIVE,
            Self::Explicit => EXPLICIT,
        }
    }
}

impl From<u8> for AdvisoryRating {
    fn from(rating: u8) -> Self {
        match rating {
            CLEAN => Self::Clean,
            INOFFENSIVE => Self::Inoffensive,
            _ => Self::Explicit,
        }
    }
}

impl fmt::Display for AdvisoryRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clean => write!(f, "Clean"),
            Self::Inoffensive => write!(f, "Inoffensive"),
            Self::Explicit => write!(f, "Explicit"),
        }
    }
}

/// An enum representing the channel configuration of an MPEG-4 audio track.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelConfig {
    /// Mono
    Mono,
    /// Stereo
    Stereo,
    /// 3.0
    Three,
    /// 4.0
    Four,
    /// 5.0
    Five,
    /// 5.1
    FiveOne,
    /// 7.1
    SevenOne,
}

impl TryFrom<u8> for ChannelConfig {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            MONO => Ok(Self::Mono),
            STEREO => Ok(Self::Stereo),
            THREE => Ok(Self::Three),
            FOUR => Ok(Self::Four),
            FIVE => Ok(Self::Five),
            FIVE_ONE => Ok(Self::FiveOne),
            SEVEN_ONE => Ok(Self::SevenOne),
            _ => Err(Self::Error::new(
                crate::ErrorKind::UnknownChannelConfig(value),
                "Unknown channel config index".to_owned(),
            )),
        }
    }
}

impl ChannelConfig {
    /// Returns the number of channels.
    pub const fn channel_count(&self) -> u8 {
        match self {
            Self::Mono => 1,
            Self::Stereo => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::FiveOne => 6,
            Self::SevenOne => 8,
        }
    }
}

impl fmt::Display for ChannelConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mono => write!(f, "Mono"),
            Self::Stereo => write!(f, "Stereo"),
            Self::Three => write!(f, "3.0"),
            Self::Four => write!(f, "4.0"),
            Self::Five => write!(f, "5.0"),
            Self::FiveOne => write!(f, "5.1"),
            Self::SevenOne => write!(f, "7.1"),
        }
    }
}

/// An enum representing the sample rate of an MPEG-4 audio track.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SampleRate {
    /// A Sample rate of 96000Hz
    Hz96000,
    /// A Sample rate of 88200Hz
    Hz88200,
    /// A Sample rate of 64000Hz
    Hz64000,
    /// A Sample rate of 48000Hz
    Hz48000,
    /// A Sample rate of 44100Hz
    Hz44100,
    /// A Sample rate of 32000Hz
    Hz32000,
    /// A Sample rate of 24000Hz
    Hz24000,
    /// A Sample rate of 24050Hz
    Hz22050,
    /// A Sample rate of 16000Hz
    Hz16000,
    /// A Sample rate of 12000Hz
    Hz12000,
    /// A Sample rate of 11050Hz
    Hz11025,
    /// A Sample rate of 8000Hz
    Hz8000,
    /// A Sample rate of 7350Hz
    Hz7350,
}

impl TryFrom<u8> for SampleRate {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            HZ_96000 => Ok(Self::Hz96000),
            HZ_88200 => Ok(Self::Hz88200),
            HZ_64000 => Ok(Self::Hz64000),
            HZ_48000 => Ok(Self::Hz48000),
            HZ_44100 => Ok(Self::Hz44100),
            HZ_32000 => Ok(Self::Hz32000),
            HZ_24000 => Ok(Self::Hz24000),
            HZ_22050 => Ok(Self::Hz22050),
            HZ_16000 => Ok(Self::Hz16000),
            HZ_12000 => Ok(Self::Hz12000),
            HZ_11025 => Ok(Self::Hz11025),
            HZ_8000 => Ok(Self::Hz8000),
            HZ_7350 => Ok(Self::Hz7350),
            _ => Err(Self::Error::new(
                crate::ErrorKind::UnknownChannelConfig(value),
                "Unknown sample rate index".to_owned(),
            )),
        }
    }
}

impl fmt::Display for SampleRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}Hz", self.hz())
    }
}

impl SampleRate {
    /// Returns the sample rate in Hz.
    pub const fn hz(&self) -> u32 {
        match self {
            Self::Hz96000 => 96000,
            Self::Hz88200 => 88200,
            Self::Hz64000 => 64000,
            Self::Hz48000 => 48000,
            Self::Hz44100 => 44100,
            Self::Hz32000 => 32000,
            Self::Hz24000 => 24000,
            Self::Hz22050 => 22050,
            Self::Hz16000 => 16000,
            Self::Hz12000 => 12000,
            Self::Hz11025 => 11025,
            Self::Hz8000 => 8000,
            Self::Hz7350 => 7350,
        }
    }
}

/// A struct containing information about a mp4 track.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AudioInfo {
    /// The duration of the track.
    pub duration: Option<Duration>,
    /// The channel configuration of the track.
    pub channel_config: Option<ChannelConfig>,
    /// The sample rate of the track.
    pub sample_rate: Option<SampleRate>,
    /// The maximum bitrate of the track.
    pub max_bitrate: Option<u32>,
    /// The average bitrate of the track.
    pub avg_bitrate: Option<u32>,
}

/// An alias for an image reference.
pub type ImgRef<'a> = Img<&'a [u8]>;
/// An alias for a mutable image reference.
pub type ImgMut<'a> = Img<&'a mut Vec<u8>>;
/// An alias for an owned image buffer.
pub type ImgBuf = Img<Vec<u8>>;

/// A struct representing an image.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Img<T> {
    /// The image format.
    pub fmt: ImgFmt,
    /// The image data.
    pub data: T,
}

impl<T> Img<T> {
    /// Creates a new image.
    pub const fn new(fmt: ImgFmt, data: T) -> Self {
        Self { fmt, data }
    }

    /// Creates a new image with the bmp format.
    pub const fn bmp(data: T) -> Self {
        Self::new(ImgFmt::Bmp, data)
    }

    /// Creates a new image with the jpeg format.
    pub const fn jpeg(data: T) -> Self {
        Self::new(ImgFmt::Jpeg, data)
    }

    /// Creates a new image with the png format.
    pub const fn png(data: T) -> Self {
        Self::new(ImgFmt::Png, data)
    }
}

/// An enum representing image formats.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImgFmt {
    /// Bmp.
    Bmp,
    /// Jpeg.
    Jpeg,
    /// Png.
    Png,
}

impl ImgFmt {
    /// Returns true if `self` is of type [`Self::Bmp`] false otherwise.
    pub fn is_bmp(&self) -> bool {
        matches!(self, Self::Bmp)
    }

    /// Returns true if `self` is of type [`Self::Jpeg`] false otherwise.
    pub fn is_jpeg(&self) -> bool {
        matches!(self, Self::Jpeg)
    }

    /// Returns true if `self` is of type [`Self::Png`] false otherwise.
    pub fn is_png(&self) -> bool {
        matches!(self, Self::Png)
    }
}
