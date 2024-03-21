let project_name = variable::get("project-name");
file::delete("./simple/cargo-generate.toml");
file::rename("./simple", "./" + project_name);
