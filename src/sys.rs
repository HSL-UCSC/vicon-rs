#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{thread, time::Duration};

use crate::{
    HasViconHardware, OutputRotation, RotationType, ViconError, ViconSdkStatus, ViconSubject,
};
use nalgebra::{Quaternion, UnitQuaternion, Vector3};

include!(concat!(env!("OUT_DIR"), "/libvicon.rs"));

/// Maximum number of times [`ViconSystem::new`]
/// will attempt to connect to a Vicon data stream.
pub const MAX_CONNECT_RETRIES: usize = 3;

/// Maximim timeout [`ViconSystem::new`] will
/// use when connecting to a vicon data stream.
pub const MAX_CONNECT_TIMEOUT: u32 = 1000;

/// An active connection to
/// a real Vicon data stream.
pub struct ViconSystem {
    vicon_handle: *mut std::ffi::c_void,
}

impl ViconSystem {
    /// Returns a new system connected
    /// to a Vicon data stream at `hostname`.
    ///
    /// The provided `hostname` may optionally
    /// include a port suffix (e.g., `192.168.1.1:810`).
    pub fn new(hostname: &str) -> Result<Self, ViconError> {
        let vicon_handle = unsafe { Client_Create() };

        // Try connecting to the Vicon.
        let host_and_port = std::ffi::CString::new(hostname).unwrap();
        let mut attempts = 0;
        loop {
            let status: ViconSdkStatus = unsafe {
                Client_SetConnectionTimeout(vicon_handle, MAX_CONNECT_TIMEOUT);
                Client_Connect(vicon_handle, host_and_port.as_ptr()).into()
            };

            if status.is_success() {
                break;
            }

            if attempts > MAX_CONNECT_RETRIES {
                return Err(ViconError::SdkError { source: status });
            }

            attempts += 1;
        }

        // Configure SDK client data stream.
        unsafe {
            Client_SetStreamMode(vicon_handle, CStreamMode_ClientPull.try_into().unwrap());
            Client_SetAxisMapping(
                vicon_handle,
                CDirection_Forward.try_into().unwrap(),
                CDirection_Left.try_into().unwrap(),
                CDirection_Up.try_into().unwrap(),
            );
        }

        // TODO: The reference usage by the NEST Lab
        //       questions if these steps are needed
        //       --copy-pasta for completeness.
        unsafe {
            Client_EnableSegmentData(vicon_handle);
            Client_EnableMarkerData(vicon_handle);
        }
        thread::sleep(Duration::from_millis(1000));

        Ok(Self { vicon_handle })
    }
}

impl HasViconHardware for ViconSystem {
    fn read_frame_subjects(
        &mut self,
        rotation: OutputRotation,
    ) -> Result<Vec<ViconSubject>, ViconError> {
        // Get a new frame.
        let _: ViconError = unsafe { Client_GetFrame(self.vicon_handle).try_into()? };

        // Count the subjects in the frame.
        let mut subject_count = COutput_GetSubjectCount {
            Result: CResult_UnknownResult as i32,
            SubjectCount: 0,
        };
        unsafe {
            Client_GetSubjectCount(self.vicon_handle, &mut subject_count);
        }
        let _: ViconError = subject_count.Result.try_into()?;
        let subject_count = subject_count.SubjectCount;

        // Visit all subjects in the frame.
        let mut subjects = Vec::with_capacity(subject_count.try_into().unwrap());
        for i in 0..subject_count {
            // Get the subject's name.
            let mut buffer = vec![0; 1024];
            let subject_name = unsafe {
                let _: ViconError = Client_GetSubjectName(
                    self.vicon_handle,
                    i,
                    buffer.capacity() as i32,
                    buffer.as_mut_ptr(),
                )
                .try_into()?;
                buffer_to_cstring(buffer)
            };

            // Get the subject's segment count.
            let mut segment_count = COutput_GetSegmentCount {
                Result: CResult_UnknownResult as i32,
                SegmentCount: 0,
            };
            unsafe {
                Client_GetSegmentCount(
                    self.vicon_handle,
                    subject_name.as_ptr(),
                    &mut segment_count,
                );
            }
            let _: ViconError = segment_count.Result.try_into()?;
            let segment_count = segment_count.SegmentCount;

            // Skip subjects with no segments.
            if segment_count == 0 {
                continue;
            }

            // Get the _zeroth_ segment's name.
            let mut buffer = vec![0; 1024];
            let segment_name = unsafe {
                let _: ViconError = Client_GetSegmentName(
                    self.vicon_handle,
                    subject_name.as_ptr(),
                    0,
                    buffer.capacity() as i32,
                    buffer.as_mut_ptr(),
                )
                .try_into()?;
                buffer_to_cstring(buffer)
            };

            // Get the segment's translation.
            let mut segment_translation = COutput_GetSegmentGlobalTranslation {
                Result: CResult_UnknownResult as i32,
                Translation: [0.0f64; 3],
                Occluded: -1,
            };
            unsafe {
                Client_GetSegmentGlobalTranslation(
                    self.vicon_handle,
                    subject_name.as_ptr(),
                    segment_name.as_ptr(),
                    &mut segment_translation,
                );
            }
            let _: ViconError = segment_translation.Result.try_into()?;

            // Skip occluded segments.
            if segment_translation.Occluded != 0 {
                continue;
            }

            // Get the segment's rotation.
            let segment_rotation: ViconRotationType = match rotation {
                OutputRotation::Quaternion => unsafe {
                    let mut segment_rotation = COutput_GetSegmentGlobalRotationQuaternion {
                        Result: CResult_UnknownResult as i32,
                        Rotation: [0.0f64; 4],
                        Occluded: -1,
                    };
                    Client_GetSegmentGlobalRotationQuaternion(
                        self.vicon_handle,
                        subject_name.as_ptr(),
                        segment_name.as_ptr(),
                        &mut segment_rotation,
                    );
                    // Reorder quaternion components to match nalgebra's convention.
                    segment_rotation.Rotation = [
                        segment_rotation.Rotation[3],
                        segment_rotation.Rotation[0],
                        segment_rotation.Rotation[1],
                        segment_rotation.Rotation[2],
                    ];
                    ViconRotationType::Quaternion(segment_rotation)
                },
                OutputRotation::Euler => {
                    let mut segment_rotation = COutput_GetSegmentGlobalRotationEulerXYZ {
                        Result: CResult_UnknownResult as i32,
                        Rotation: [0.0f64; 3],
                        Occluded: -1,
                    };
                    unsafe {
                        Client_GetSegmentGlobalRotationEulerXYZ(
                            self.vicon_handle,
                            subject_name.as_ptr(),
                            segment_name.as_ptr(),
                            &mut segment_rotation,
                        );
                    }
                    ViconRotationType::Euler(segment_rotation)
                }
            };

            let _: ViconError = segment_rotation.result().try_into()?;

            // Skip occluded segments.
            if segment_rotation.occluded() {
                continue;
            }

            subjects.push(ViconSubject::from_vicon_frame(
                subject_name.to_str().unwrap().to_owned(),
                segment_translation,
                segment_rotation,
            )?);
        }
        Ok(subjects)
    }
}

unsafe impl Send for ViconSystem {}

impl ViconSubject {
    /// Converts raw segment data from a Vicon
    /// to a [`ViconSubject`].
    fn from_vicon_frame(
        name: String,
        translation: COutput_GetSegmentGlobalTranslation,
        vicon_rotation: ViconRotationType,
    ) -> Result<Self, ViconError> {
        // Calculate origins, converting from
        // millimeters to meters.
        let origin_x = translation.Translation[0] / 1000.0;
        let origin_y = translation.Translation[1] / 1000.0;
        let origin_z = translation.Translation[2] / 1000.0;

        Ok(Self {
            name,
            origin: Vector3::<f64>::new(origin_x, origin_y, origin_z),
            rotation: RotationType::try_from(vicon_rotation).map_err(|e| {
                ViconError::OtherError {
                    message: e.to_string(),
                }
            })?,
        })
    }
}

pub enum ViconRotationType {
    Euler(COutput_GetSegmentGlobalRotationEulerXYZ),
    Quaternion(COutput_GetSegmentGlobalRotationQuaternion),
}

impl ViconRotationType {
    pub fn occluded(&self) -> bool {
        match self {
            ViconRotationType::Euler(euler) => euler.Occluded != 0,
            ViconRotationType::Quaternion(quaternion) => quaternion.Occluded != 0,
        }
    }

    pub fn result(&self) -> i32 {
        match self {
            ViconRotationType::Euler(euler) => euler.Result,
            ViconRotationType::Quaternion(quaternion) => quaternion.Result,
        }
    }
}

impl TryFrom<ViconRotationType> for RotationType {
    type Error = String;
    fn try_from(value: ViconRotationType) -> Result<Self, Self::Error> {
        match value {
            ViconRotationType::Euler(euler) => {
                let euler = Vector3::new(euler.Rotation[0], euler.Rotation[1], euler.Rotation[2]);
                return Ok(Self::Euler(euler));
            }
            ViconRotationType::Quaternion(quaternion) => {
                let quaternion = UnitQuaternion::from_quaternion(Quaternion::new(
                    quaternion.Rotation[0],
                    quaternion.Rotation[1],
                    quaternion.Rotation[2],
                    quaternion.Rotation[3],
                ));
                return Ok(Self::Quaternion(quaternion));
            }
        }
    }
}

/// Utility which converts a `buffer`
/// of raw C characters into a valid
/// C string, stripping all `0`s from
/// the buffer.
unsafe fn buffer_to_cstring(buffer: Vec<std::os::raw::c_char>) -> std::ffi::CString {
    // Remove all null bytes,
    // and add _one_ null byte
    // to the end.
    let buffer = buffer
        .into_iter()
        .filter(|b| b != &0)
        .map(|b| b as u8)
        .collect::<Vec<u8>>();

    std::ffi::CString::new(buffer).unwrap()
}
