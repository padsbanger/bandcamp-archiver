pub struct Album {
    songs: Vec<string>,
}

impl Album {
    fn new() -> Self {
        Album { songs: Vec::new() }
    }
}
