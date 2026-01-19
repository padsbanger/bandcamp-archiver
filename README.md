# bandcamp-archiver
Cli tool to archive Bandcamp albums


## Installation

Clone this repo, in root directory run:

```bash

bandcamp-archiver --url album_url --destination ./downloads

```

Example:

```bash
bandcamp-archiver --url https://padsbanger.bandcamp.com/album/eleventh-dimension --destination ./downloads

```
Destination is optional, defaults to current working directory.

## Limitations

- Only mp3 128kbps quality is supported, no lossless formats
- Only public albums are supported, no private/shared links
- Progress bars are sometimes broken, thats because Bandcamp sometimes blocks HEAD requests, causing wrong content length to be reported
## Legality

This tools uses Bandcamp Player html/js to look for audio files, which are publicly available on the album page anyway.

Apperantly Bandcamp is aware of this and doesnt care about patching it: https://bandcamp.com/help/audio_basics#steal <br />

I am not a lawyer, but downloading copyrighted material without permission is probably illegal in your country. Use this tool at your own risk.

Always support the artists you like by buying their music!

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
- Better UI, especially for discography downloads, it gets very messy right now with multiple albums downloading at once
- Error handling
