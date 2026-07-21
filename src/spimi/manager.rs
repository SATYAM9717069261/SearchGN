pub struct SPIMIManager {
    current_index: InvertedIndex,
    block_id: usize,
    words_processed: usize,
    flush_limit: usize,
}
