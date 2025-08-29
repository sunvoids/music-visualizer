Little program done in a day for fun. Idea from http://nifty.stanford.edu/2025/wayne-music-visualizer/specification.html.
Samples included come from the above site except for sine.wav, which is a file created from hound's example code in documentation (https://crates.io/crates/hound).

To use, run `cargo run [wav file] [# of groups, above 0]`. For songs, I've noticed 10.000 -> 100.000 is a nice looking range of groups. 
Space pauses playback. Left seeks 5 seconds back, right seeks 5 seconds forward.
