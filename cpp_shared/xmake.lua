add_rules("mode.debug", "mode.release")

target("cpp_shared")
    set_kind("shared")
    add_files("*.cpp")
    set_languages("c99", "c++11")