# THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT.

{
    "target_defaults": {
        "default_configuration": "<!(python -c 'import os, json; print(\"Debug\" if u\"--debug\" in json.loads(os.getenv(\"npm_config_argv\"))[\"cooked\"] else \"Release\")')"
    },

    "variables": {
        "cargo_config": "<!(python -c 'import os, json; print(\"debug\" if u\"--debug\" in json.loads(os.getenv(\"npm_config_argv\"))[\"cooked\"] else \"release\")')",
        "cargo_flags": "<!(python -c 'import os, json; print(\"\" if u\"--debug\" in json.loads(os.getenv(\"npm_config_argv\"))[\"cooked\"] else \"--release\")')"
    },

    "targets": [{
        "target_name": "wc_demo",

        "variables": {
            "rust_inputs": "<!(python -c 'import os, os.path, sys; [sys.stdout.write(os.path.join(sub, f) + \"\\n\") for sub, _, files in os.walk(\"src\") for f in files if f.endswith(\".rs\")]')",
            "static_lib": "target/<(cargo_config)/<(STATIC_LIB_PREFIX)wc_demo<(STATIC_LIB_SUFFIX)"
        },

        "sources": ["src/binding.cc"],

        "include_dirs": ["<!(node -e \"require('neon-bridge').headers()\")"],

        "libraries": ["../<(static_lib)"],

        "conditions": [
            ["OS=='mac'", {
                "xcode_settings": {
                    "MACOSX_DEPLOYMENT_TARGET": "10.7"
                }
            }]
        ],

        "actions": [{
            "action_name": "cargo",
            "inputs": ["<@(rust_inputs)"],
            "outputs": ["../<(static_lib)"],
            "action": ["cargo", "rustc", "<@(cargo_flags)" , "--", "--crate-type", "staticlib"]
        }]
    }]

}
