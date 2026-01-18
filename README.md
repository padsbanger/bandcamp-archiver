# bandcamp-archiver
Cli tool to archive Bandcamp albums


## Installation

```bash

cargo run -- --url album_url --destination ./downloads

```

Destination is optional, defaults to current working directory.


## TODO

- ~~Create scapper struct~~
- ~~Create album struct~~
- ~~Create song struct~~
- ~~Get Bandcamp schema and deserialize response into structs~~
- ~~Download whole album into a directory~~
- ~~Add destination folder option~~

#### Nice to have:
- Download using threads
- Download whole artist discography
- ~~Implement some kind of CLI UI Interface with progress bars~~