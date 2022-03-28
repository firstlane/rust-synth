~~I need to rethink the way the code blocks. It's subtly different compared to the C++ code, where I lock on every call to the audio callback, but I only lock on *receiving* a midi message. Currently in the Rust code, I lock on every call to Update and check for midi messages. This is due to me having to wrap the synth in an Arc<Mutex> to get it across threads. The synth owns the midi receiver, but I have to lock in order to call Update, regardless of whether or not I have data.~~

~~Instead, I should pull the midi receiver out of the synth so that I can block only on receiving a key press, instead of *every* iteration of the Update loop. When I receive a midi message, I can lock on the synth and call a function to update its internal note data.~~

Try to get just the volume working on the Synth for now. I don't care too much about how I get it working, even if it's hacky. But while I'm working on it, think about how I'll make this work when I have many different parameters that need to be controlled.

I want the synth to be able to create a list of components it has and then pass those to the GUI to be displayed.
Each component should specify it's type (bool, float, etc.) and the GUI would determine how to display that, unless the component requested a specific display type.
