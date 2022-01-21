const FLATPAK: &str = r#"
{
    "app-id": "test.test.Devel",
    "runtime": "org.gnome.Platform",
    "runtime-version": "41",
    "sdk": "org.gnome.Sdk",
    "sdk-extensions": ["org.freedesktop.Sdk.Extension.rust-stable"],
    "command": "test",
    "finish-args" : [
        "--socket=fallback-x11",
        "--socket=wayland",
        "--device=dri",
        "--env=RUST_LOG=test=debug",
        "--env=G_MESSAGES_DEBUG=none",
        "--env=RUST_BACKTRACE=1"
    ],
    "build-options" : {
        "append-path" : "/usr/lib/sdk/rust-stable/bin",
        "build-args" : [
            "--share=network"
        ],
        "test-args": [
            "--socket=x11",
            "--share=network"
        ]
    },
    "modules": [
        {
            "name": "test",
            "buildsystem": "meson",
            "buildsystem": "simple",
            "build-commands": [
                "cargo --offline fetch --manifest-path Cargo.toml --verbose",
                "cargo --offline build --release --verbose",
                "install -Dm755 ./target/debug/gtk-builder -t /app/bin/"
            ],
            "run-tests": true,
            "config-opts": ["-Dprofile=development"],
            "sources": [
                {
                    "type": "dir",
                    "path": "../"
                }
            ]
        }
    ]
}
"#;

pub fn generate_output() {
    // Create build-aux first!
    std::fs::write("build-aux/test.test.Devel.json", FLATPAK).unwrap();
}

