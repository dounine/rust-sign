add_rules("mode.debug", "mode.release")

target("hello_shared")
    set_kind("shared")
    add_files("*.c")
    set_languages("c99")