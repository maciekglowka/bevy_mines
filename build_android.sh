docker run --rm -v $(pwd):/app -v cargo_index_android:/usr/local/cargo -t bevy_android cargo apk build
