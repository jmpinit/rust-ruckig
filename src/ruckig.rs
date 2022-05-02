#[cxx::bridge]
pub mod ffi {
    pub struct InitState {
        pub current_position: Vec<f64>,
        pub current_velocity: Vec<f64>,
        pub current_acceleration: Vec<f64>,

        pub target_position: Vec<f64>,
        pub target_velocity: Vec<f64>,
        pub target_acceleration: Vec<f64>,

        pub max_velocity: Vec<f64>,
        pub max_acceleration: Vec<f64>,
        pub max_jerk: Vec<f64>,
    }

    pub struct Output {
        pub new_position: Vec<f64>,
        pub new_velocity: Vec<f64>,
        pub new_acceleration: Vec<f64>,
        pub time: f64,
    }

    unsafe extern "C++" {
        include!("rust-ruckig/include/ruckig.h");

        type RuckigClient;

        pub fn new_ruckig_client() -> UniquePtr<RuckigClient>;
        pub fn set_inputs(&self, angles: &mut InitState);
        pub fn update(&self) -> bool;
        pub fn get_outputs(&self) -> Output;
    }
}
