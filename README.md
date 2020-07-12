# RustSnitch
Language agnostic tool to manage patterns (e.g. TODO) in code comments

## Motivation for the project
Firstly, I liked [this project](https://github.com/tsoding/snitch) - it was very simple yet quite useful.
One problem: it's written in GO, installation can be tricky and I got a segfault when working on my resume project which left my code with a bunch of useless TODO marks, which actually were implemented.

Secondly, I want to learn Rust. At least on some minimal level, but still.

Thirdly, I'm sure there could be other uses, other "hooks" for comments. Thus I want some more modularity on that aspect. Even if I am not sure what those could be.

Lastly, I want it to be usefull for stuff like CI. I wasted a day on old project, trying to mary it to Github Actions, with no success. Sure, I'm just dumb and have not enought experience, but still. This one is more like a feature requirement.
