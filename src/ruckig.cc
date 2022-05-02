#include "rust-ruckig/include/ruckig.h"
#include "rust-ruckig/src/ruckig.rs.h"
#include <ruckig/ruckig.hpp>
#include <iostream>

using namespace ruckig;

class RuckigClient::impl {
    friend RuckigClient;

    Ruckig<6> otg{0.012};
    InputParameter<6> input;
    OutputParameter<6> output;
};

RuckigClient::RuckigClient() : impl(new class RuckigClient::impl) {}

void RuckigClient::set_inputs(InitState &initState) const {
    for (int i = 0; i < 6; i++) {
        impl->input.current_position[i] = initState.current_position[i];
        impl->input.current_velocity[i] = initState.current_velocity[i];
        impl->input.current_acceleration[i] = initState.current_acceleration[i];

        impl->input.target_position[i] = initState.target_position[i];
        impl->input.target_velocity[i] = initState.target_velocity[i];
        impl->input.target_acceleration[i] = initState.target_acceleration[i];

        impl->input.max_velocity[i] = initState.max_velocity[i];
        impl->input.max_acceleration[i] = initState.max_acceleration[i];
        impl->input.max_jerk[i] = initState.max_jerk[i];
    }
}

bool RuckigClient::update() const {
    auto res = impl->otg.update(impl->input, impl->output);

    if (res != Result::Working) {
        return false;
    }

    impl->output.pass_to_input(impl->input);

    return true;
}

Output RuckigClient::get_outputs() const {
    Output out {};

    for (int i = 0; i < 6; i++) {
        out.new_position.emplace_back(impl->output.new_position[i]);
        out.new_velocity.emplace_back(impl->output.new_velocity[i]);
        out.new_acceleration.emplace_back(impl->output.new_acceleration[i]);
    }

    out.time = impl->output.time;

    return out;
}

std::unique_ptr <RuckigClient> new_ruckig_client() {
    return std::make_unique<RuckigClient>();
}
