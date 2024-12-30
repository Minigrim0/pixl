# Pixel Flut


# Images
```shell
# Convert image to commands
uv run test.py input.png output.txt

# Send commands to server
cargo run --bin image --release
```

# Video
```shell
# Convert image to commands
ffmpeg -i InputFile.mp4 -s hd480 -c:v libx264 -crf 23 -c:a aac -strict -2 output_file.mp4

# You might need to change the filename in the src/bin/video.rs file

# Send commands to server
cargo run --bin video --release [--features report]
```
