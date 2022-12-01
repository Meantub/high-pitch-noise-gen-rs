# High Pitch Noise Generator

This is a solution to play a super low volume high frequency noise (unplayable by most speakers), to keep the Phonak TV Connector V2 from stuttering when inactive for more than a few seconds.

The python version that I had created at [https://github.com/Meantub/high-pitch-noise-gen](https://github.com/Meantub/high-pitch-noise-gen) was annoying to package into a binary due to Python annoyances on Windows. Particularly it used numpy which notoriously needs `conda` to even use. So automating a build with it was more work than it was worth. So I ended up just rewriting in Rust which I knew would be easier to compile on both Linux and Windows.

It creates a Tauri (alternative to Electron) application just for the system tray that you can quit by right-clicking on it.# high-pitch-noise-gen-rs
