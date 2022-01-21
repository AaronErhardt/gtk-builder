pub struct Project {
    app_name: String,
    project_name: String,
    app_id: String,
}

impl Project {
    pub fn new(app_name: &str, project_name: &str, app_id: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            project_name: project_name.to_string(),
            app_id: app_id.to_string(),
        }
    }

    pub fn write_flatpak_manifest(&self) {
        let Project {
            app_id,
            project_name,
            ..
        } = self;

        let manifest = format!(
            r#"
{{
    "app-id": "{app_id}",
    "runtime": "org.gnome.Platform",
    "runtime-version": "41",
    "sdk": "org.gnome.Sdk",
    "sdk-extensions": ["org.freedesktop.Sdk.Extension.rust-stable"],
    "command": "{project_name}",
    "finish-args" : [
        "--socket=fallback-x11",
        "--socket=wayland",
        "--device=dri",
        "--env=RUST_LOG=test=debug",
        "--env=G_MESSAGES_DEBUG=none",
        "--env=RUST_BACKTRACE=1"
    ],
    "build-options" : {{
        "append-path" : "/usr/lib/sdk/rust-stable/bin",
        "build-args" : [
            "--share=network"
        ],
        "test-args": [
            "--socket=x11",
            "--share=network"
        ]
    }},
    "modules": [
        {{
            "name": "{project_name}",
            "buildsystem": "simple",
            "build-commands": [
                "cargo build",
                "install -Dm755 ./target/debug/{project_name} -t /app/bin/"
            ],
            "run-tests": true,
            "config-opts": ["-Dprofile=development"],
            "sources": [
                {{
                    "type": "dir",
                    "path": "../"
                }}
            ]
        }}
    ]
}}
"#
        );

        // Create build-aux first!
        std::fs::create_dir("build-aux").unwrap();
        std::fs::write("build-aux/test.test.Devel.json", manifest).unwrap();
    }
}
