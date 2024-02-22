add_rules("mode.debug", "mode.release")

target("hello_static")
    set_kind("static")
    add_files("*.c")
    set_languages("c99")