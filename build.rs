fn main() {
    cxx_build::bridge("src/ruckig.rs")
        .file("src/ruckig.cc")
        .include("third-party/ruckig/include")
        .file("third-party/ruckig/src/brake.cpp")
        .file("third-party/ruckig/src/position-step1.cpp")
        .file("third-party/ruckig/src/position-step2.cpp")
        .file("third-party/ruckig/src/velocity-step1.cpp")
        .file("third-party/ruckig/src/velocity-step2.cpp")
        .flag_if_supported("-std=c++17")
        .compile("ruckig-rust-demo");
}