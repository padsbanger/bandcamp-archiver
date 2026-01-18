# bandcamp-archiver
Cli tool to archive Bandcamp albums


## Installation

```bash

cargo run -- --url album_url --destination ./downloads

```

Example:

```bash
cargo run -- --url https://padsbanger.bandcamp.com/album/eleventh-dimension --destination ./downloads

```


Destination is optional, defaults to current working directory.


## TODO

- ~~Create scrapper struct~~
- ~~Create album struct~~
- ~~Create song struct~~
- ~~Get Bandcamp schema and deserialize response into structs~~
- ~~Download whole album into a directory~~
- ~~Add destination folder option~~
- ~~Download using threads~~
- ~~Download whole artist discography~~
- ~~Implement some kind of CLI UI Interface with progress bars~~

#### Nice to have:
- Better UI, especially for discography downloads, it gets very messy right now
- Error handling improvements