use vicon_sys::HasViconHardware;
use vicon_sys::ViconSubject;

pub struct MockVicon {
    pub subjects: Vec<ViconSubject>,
}

impl MockVicon {
    pub fn new() -> Self {
        MockVicon {
            subjects: vec![ViconSubject {
                name: "mob_6".to_string(),
                origin: nalgebra::Vector3::new(0.0, 0.0, 0.0),
                rotation: vicon_sys::RotationType::Quaternion(nalgebra::UnitQuaternion::identity()),
                occluded: false,
            }],
        }
    }
}

impl HasViconHardware for MockVicon {
    fn read_frame_subjects(
        &mut self,
        rotation_type: vicon_sys::OutputRotation,
    ) -> Result<Vec<ViconSubject>, vicon_sys::ViconError> {
        Ok(
            self.subjects
                .clone()
                .into_iter()
                .map(|mut s| {
                    s.rotation = match s.rotation {
                        vicon_sys::RotationType::Euler(euler) => match rotation_type {
                            vicon_sys::OutputRotation::Euler => {
                                vicon_sys::RotationType::Euler(euler)
                            }
                            vicon_sys::OutputRotation::Quaternion => {
                                vicon_sys::RotationType::Quaternion(
                                    nalgebra::UnitQuaternion::identity(),
                                )
                            }
                        },
                        vicon_sys::RotationType::Quaternion(quat) => match rotation_type {
                            vicon_sys::OutputRotation::Euler => {
                                let euler = quat.euler_angles();
                                vicon_sys::RotationType::Euler(nalgebra::Vector3::new(
                                    euler.0, euler.1, euler.2,
                                ))
                            }
                            vicon_sys::OutputRotation::Quaternion => {
                                vicon_sys::RotationType::Quaternion(quat)
                            }
                        },
                    };
                    s
                })
                .collect(),
        )
    }
}

fn main() {
    let mut mock_vicon = MockVicon::new();
    let subjects = mock_vicon.read_frame_subjects(vicon_sys::OutputRotation::Quaternion);
    println!("{:?}", subjects);
}
