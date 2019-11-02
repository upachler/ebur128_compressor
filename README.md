# lv2-signal-gen
Test Plugin for audio software in [LV2 format](http://lv2plug.in/]). The plugin generates a sine wave with variable frequency. 
Users can change the frequency at rumtime.

Use an LV2 plugin host like [Carla](https://kx.studio/Applications:Carla) as a plugin host and (optionally) 
an oscilloscope plugin (like the [x42 Oscilloscpe](https://x42-plugins.com/x42/x42-scope)) for testing.

# Building
This is rough around the edges: 
* make sure you have the Rust toolchain ([-> Rustup.rs](https://rustup.rs))
* get this source and run ./build.sh (this will run cargo and copy stuff to your ~/.lv2 directory). Note that on Windows, you'll need a MinGW shell to run this; the easiest (and tested) way is to install [Git for Windows](https://git-scm.com/download/win) and start the 'Git Bash' - then run ./build.sh from there.

Batteries not included.
