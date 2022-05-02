#pragma once

#include "rust/cxx.h"
#include <ruckig/ruckig.hpp>

struct InitState;
struct Output;

class RuckigClient {
public:
    RuckigClient();

    void set_inputs(InitState &initState) const;
    bool update() const;
    Output get_outputs() const;

private:
    class impl;

    std::shared_ptr <impl> impl;

    ruckig::Ruckig<6> otg;
    ruckig::InputParameter<6> input;
    ruckig::OutputParameter<6> output;
};

std::unique_ptr <RuckigClient> new_ruckig_client();

void hello_world();
