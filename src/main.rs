mod ruckig;

fn main() {
    let otg = ruckig::ffi::new_ruckig_client();

    let mut init = ruckig::ffi::InitState {
        current_position: vec![0.0, 0.0, 0.5, 0.0, 0.0, 0.0],
        current_velocity: vec![0.0, -2.2, -0.5, 0.0, 0.0, 0.0],
        current_acceleration: vec![0.0, 2.5, -0.5, 0.0, 0.0, 0.0],

        target_position: vec![5.0, -2.0, -3.5, 0.0, 0.0, 0.0],
        target_velocity: vec![0.0, -0.5, -2.0, 0.0, 0.0, 0.0],
        target_acceleration: vec![0.0, 0.0, 0.5, 0.0, 0.0, 0.0],

        max_velocity: vec![3.0, 1.0, 3.0, 1.0, 1.0, 1.0],
        max_acceleration: vec![3.0, 2.0, 1.0, 1.0, 1.0, 1.0],
        max_jerk: vec![4.0, 3.0, 2.0, 1.0, 1.0, 1.0],
    };

    otg.set_inputs(&mut init);

    loop {
        let more_work_to_do = otg.update();
        let out = otg.get_outputs();

        for angle in out.new_position {
            print!("{}, ", angle);
        }

        println!();

        if !more_work_to_do { break; }
    }
}
