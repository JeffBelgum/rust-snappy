extern crate gcc;

fn build_snappy() {
    let mut config = gcc::Config::new();
    config.include("snappy/");
    config.include(".");

    config.define("NDEBUG", Some("1"));

    if cfg!(target_env = "msvc") {
        config.flag("-EHsc");
    } else {
        config.flag("-std=c++11");
    }

    config.file("snappy/snappy.cc");
    config.file("snappy/snappy-sinksource.cc");
    config.file("snappy/snappy-c.cc");
    config.cpp(true);
    config.compile("libsnappy.a");
}

fn main() {
    build_snappy();
}
