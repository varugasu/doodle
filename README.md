# Doodle

This is a personal CLI tht allows me to quickly start a project based in template and how to create it

This project uses TOML to define the templates and how to create them. Currently, it only supports executing shell commands (totally vulnerable for now):

```toml
[raylib]
run = [
    "git clone --depth 1 https://github.com/varugasu/raylib-template",
    "rm -rf raylib-template/.git",
    "cd raylib-template",
    "git init",
]
```
