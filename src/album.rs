pub struct Album {
    title: String,
    artist: String,
    songs: Vec<Song>,
}

pub struct Song {
    file: HasMap<String, String>,
    title: String,
    artist: String,
}

impl Album {
    fn new() -> Self {
        Album { songs: Vec::new() }
    }
    fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }
}

impl Song {
    fn new(file: HashMap, title: String, artist: String) -> Self {
        Song {
            file: HashMap::new(),
            title: String::new(),
            artist: String::new(),
        }
    }
}