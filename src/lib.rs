use nalgebra::{UnitQuaternion, Vector3};
use snafu::Snafu;
pub mod sys;

#[cfg(target_os = "linux")]
/// Implementations of [`HasViconHardware`]
/// for use with a real Vicon system.
pub mod sys;

/// A thing that can read from a Vicon data stream.
pub trait HasViconHardware {
    /// Returns a list of all identified [`ViconSubject`]s
    /// in the next available frame from the system.
    fn read_frame_subjects(
        &mut self,
        rotation_type: RotationType,
    ) -> Result<Vec<ViconSubject>, ViconError>;
}

/// A single subject identified in a frame
/// read by a thing that [`HasViconHardware`].
#[derive(Debug)]
pub struct ViconSubject {
    /// The subject's name.
    pub name: String,

    /// The subject's position in meters
    /// relative to the origin of the
    /// motion capture volume.
    pub origin: Vector3<f64>,

    /// The subject's rotation (euler angles)
    /// in radians.
    pub rotation: RotationType,
}

#[derive(Debug)]
pub enum RotationType {
    Euler(Vector3<f64>),
    Quaternion(UnitQuaternion<f64>),
}

/// Enumeration of errors returned by a
/// thing that [HasViconHardware].
#[derive(Debug, Snafu)]
pub enum ViconError {
    /// An error from the Vicon SDK.
    SdkError {
        source: ViconSdkStatus,
    },
    OtherError {
        message: String,
    },
}
/// Implementation of [`TryFrom`] which
/// returns `Ok` for _successful_
/// [`ViconSdkStatus`] codes, and `Err`
/// for all other status codes.
impl TryFrom<i32> for ViconError {
    type Error = ViconError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let status = ViconSdkStatus::from(value);

        if status.is_success() {
            Ok(ViconError::SdkError { source: status })
        } else {
            Err(ViconError::SdkError { source: status })
        }
    }
}

/// Enumeration of status codes returned
/// by Vicon data stream SDK.
///
/// These status codes are derived from
/// the codes listed in the Vicon SDK's
/// `CTypeDefs.h` file.
#[derive(Debug, Snafu)]
pub enum ViconSdkStatus {
    Unknown { code: i32 },
    Unimplemented,
    Success,
    InvalidHostname,
    InvalidMulticastIp,
    ClientAlreadyConnected,
    ClientConnectionFailed,
    ServerAlreadyTransmittingMulticast,
    ServerNotTransmittingMulticast,
    NotConnected,
    NoDataFrame,
    InvalidIndex,
    InvalidCameraName,
    InvalidSubjectName,
    InvalidSegmentName,
    InvalidMarkerName,
    InvalidDeviceName,
    InvalidDeviceOutputName,
    InvalidLatencySampleRate,
    InvalidCoLinearAxes,
    LeftHandedAxes,
    HapticAlreadySet,
    EarlyDataRequested,
    LateDataRequested,
    InvalidOperation,
    Unsupported,
    ConfigurationFailed,
    NotPresent,
}

impl ViconSdkStatus {
    /// Returns `true` iff this status
    /// represents a success.
    pub fn is_success(&self) -> bool {
        matches!(self, ViconSdkStatus::Success)
    }
}

impl From<i32> for ViconSdkStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Unknown { code: 0 },
            1 => Self::Unimplemented,
            2 => Self::Success,
            3 => Self::InvalidHostname,
            4 => Self::InvalidMulticastIp,
            5 => Self::ClientAlreadyConnected,
            6 => Self::ClientConnectionFailed,
            7 => Self::ServerAlreadyTransmittingMulticast,
            8 => Self::ServerNotTransmittingMulticast,
            9 => Self::NotConnected,
            10 => Self::NoDataFrame,
            11 => Self::InvalidIndex,
            12 => Self::InvalidCameraName,
            13 => Self::InvalidSubjectName,
            14 => Self::InvalidSegmentName,
            15 => Self::InvalidMarkerName,
            16 => Self::InvalidDeviceName,
            17 => Self::InvalidDeviceOutputName,
            18 => Self::InvalidLatencySampleRate,
            19 => Self::InvalidCoLinearAxes,
            20 => Self::LeftHandedAxes,
            21 => Self::HapticAlreadySet,
            22 => Self::EarlyDataRequested,
            23 => Self::LateDataRequested,
            24 => Self::InvalidOperation,
            25 => Self::Unsupported,
            26 => Self::ConfigurationFailed,
            27 => Self::NotPresent,
            _ => Self::Unknown { code: value },
        }
    }
}
