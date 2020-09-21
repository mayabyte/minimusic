# minimusic
A little tool to bulk-transcode your music library as quickly and easily as possible.
```bash
minimusic -i ./Music -o ./OpusMusic -e ".ogg" -c libopus -b 192k
```

## Features
* Parallel execution, so it's probably a lot faster than an equivalent bash script you could hack together
* A cute little progress indicator :)
* Just wraps ffmpeg, so minimusic supports any codec ffmpeg does

## Installation
```bash
git clone git@github.com:mayabyte/minimusic.git
cd minimusic
cargo install --path .
```
Requirements:
* Rust and Cargo (tested working on 1.46 stable, idk about other versions)
* ffmpeg
* mediainfo

## Why?
You probably don't have a billion gigabytes of free space on your phone, but you probably want to keep a bunch of music on there anyway. This is an easy way to crunch down all your 400Mb flacs into something a bit more manageable - and let's be honest, your $30 pair of earbuds won't do those flacs justice anyway. And if you *are* rocking some really nice audio gear, you can still get some big space reductions without much quality loss using like 320kbps Opus or something.

"But [other program] already does this for you!" Yes, but I don't use [other program] and writing this was less effort than setting it up. Besides, I like the simplicity of this approach.

## Stuff
Q: Can I contribute to minimusic? <br>
A: Sure, if you want! This is just a spare time project of mine but if you find it useful and you want to add a feature or something, that's always welcome.

Q: Can I use minimusic at my imperial corporate workplace that disenfranchises/spies upon/enslaves/imprisons/kills minorities, or enables such actions? <br>
A: If you ask nicely :)


... Just kidding, no you cannot.
