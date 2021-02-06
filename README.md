# N a Day
[![Build Status](https://travis-ci.org/creinig/naday.svg?branch=master)](https://travis-ci.org/creinig/naday)

This is a little tool for tracking (physical) excercise of the "N repetitions a day" variant -
100 pushups per day, 10'000 steps per day etc.

Since I not only needed such a tracker, but also needed some playground project for learning [rust](https://www.rust-lang.org/),
this is implemented as CLI tool. Plus, I suck at GUIs. I personally run it in [termux](https://termux.com/) on my android phone.


## Installation

For now you have to compile & install it yourself, but proper release builds are planned, starting with milestone 0.1.0.

Manual builds:

```
git clone https://github.com/creinig/naday.git
cd naday
cargo build --release
cp target/release/naday ~/.local/bin/
```

## Usage

`naday system` prints configuration settings:

```
Storage directory: /home/creinig/.naday
Known Categories:
  Burpees         (weight 1.5  ), aliases bu
  PlankSeconds    (weight 0.33 ), aliases pl
  Pushups         (weight 1    ), aliases pu, push
  Situps          (weight 1    ), aliases si
```

This shows some key points that are central to the tool's usage:

1. All data is stored in plain text files under your home directory. Each of these files
   contains a description of its format at the top, so feel free to look at them and edit them manually if needed.

2. Different types of excercises are called "categories", and the tool comes with a few of them preinstalled
   (in `~/.naday/categories.txt`). Each category has a display name, optionally a few aliases and a weight 
   allowing a kind of "comparison" between logged repetitions. This allows for displaying a "weighted total"
   if you mix different excercises over the day.


`naday 18pu` logs a set of reps:

```
Added 18 Pushups

Stats for today:
  Pushups: 18 reps
```

This uses the alias "pu" for Pushups defined in `~/.naday/category.txt` to specify what you did.
The general pattern for this argument is `<repetitions><name_or_alias>`, with case insensitive
`name_or_alias`. So the same could have been
written as "18Pushups", "18pushups", "18Push" etc.


`naday report --day` will print a little report of today's activities (the same as the info printed
when logging an activity):

```
Stats for today:
  Burpees        : 15 reps
  Pushups        : 33 reps (16 + 17)
  PlankSeconds   : 60 reps
  Weighted total : 75
```

`naday report --month` will print an overview for the past month. For now this only lists the weighted
total per day for the past 31 days, but additional options are being worked on:

```
Wed:     0 total
Thu:     0 total
Fri:     0 total
Sat:     0 total
Sun:     0 total
Mon:     0 total
Tue:     0 total
Wed:    36 total
Thu:    16 total
Fri:     0 total
Sat:    92 total
Sun:     0 total
Mon:    87 total
Tue:     0 total
Wed:     0 total
Thu:     0 total
Fri:     0 total
Sat:     0 total
Sun:     0 total
Mon:     0 total
Tue:     0 total
Wed:     0 total
Thu:     0 total
Fri:     0 total
Sat:     0 total
Sun:     0 total
Mon:     0 total
Tue:     0 total
Wed:     0 total
Thu:     0 total
Fri:     0 total
```

Additional and better reports are planned. You can also directly load the save files into a 
spreadsheet (they are basically plain CSV) and generate your own custom reports.
