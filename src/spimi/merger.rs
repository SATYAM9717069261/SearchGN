struct MergeCursor {
    block_id: usize,
    current_word: String,
    postings: Vec<Posting>,
}
