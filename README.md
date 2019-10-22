# lv2-signal-gen
Test Plugin for audio software in [LV2 format](http://lv2plug.in/]). The plugin generates a sine wave with variable frequency. 
Users can change the frequency at rumtime.

Use an LV2 plugin host like [Carla](https://kx.studio/Applications:Carla) as a plugin host and (optionally) 
an oscilloscope plugin (like the [x42 Oscilloscpe](https://x42-plugins.com/x42/x42-scope)) for testing.

# Building
This is rough around the edges: 
* make sure you have the Rust toolchain ([-> Rustup.rs](https://rustup.rs))
* get this source and run ./signal_generator_publish.sh (this will run cargo and copy stuff to your lv2 directory)

If you're not on mac, you'll have to adjust manifest.ttl to fix the executable name (currently it is 'libsignal_generator.dylib' for Mac, 
make sure this is the proper .so or .dll name for Linux/Windows).

Batteries not included.
