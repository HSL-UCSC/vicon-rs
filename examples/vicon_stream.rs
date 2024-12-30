// use rerun;
use vicon_sys::{sys::ViconSystem, HasViconHardware, OutputRotation, RotationType};

// include!(concat!(env!("OUT_DIR"), "/libvicon.rs"));

fn main() {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_minimal").spawn().unwrap();

    let mut vicon = ViconSystem::new("localhost").unwrap();
    loop {
        let subjects = vicon
            .read_frame_subjects(OutputRotation::Quaternion)
            .unwrap();
        // println!("{:?}", subjects);
        for subject in subjects {
            println!("Subject: {}", subject.name);
            println!("Position: {:?}", subject.origin);
            println!("Rotation: {:?}", subject.rotation);

            // Log position
            // MsgSender::new("object/position")
            // .with_component(&[subject.origin])?
            // .send(&session)?;

            // Log rotation
            let rotation = match subject.rotation {
                // OutputRotation::Euler() => {
                //     rerun::Quaternion::from_euler_angles(euler.x, euler.y, euler.z)
                // }
                RotationType::Quaternion(quat) => {
                    rerun::Quaternion::from_xyzw([
                        quat.i as f32,
                        quat.j as f32,
                        quat.k as f32,
                        quat.w as f32,
                    ])
                }
                _ => panic!("Unsupported rotation type"),
            };
            let translation = rerun::Vec3D::new(
                subject.origin[0] as f32,
                subject.origin[1] as f32,
                subject.origin[2] as f32,
            );
            rec.log(
                "world/quad/base_link",
                &rerun::Transform3D::from_translation_rotation(
                    translation,
                    rotation,
                )
                .with_axis_length(0.7),
            ).unwrap();
        }
    }
}
