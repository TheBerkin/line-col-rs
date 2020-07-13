#[cfg(feature = "grapheme-clusters")]
use unicode_segmentation::UnicodeSegmentation;

/// Pre-cached line/column lookup table for a string slice.
pub struct LineColLookup<'source> {
    src: &'source str,
    line_heads: Vec<usize>,
    line_count: usize
}

impl<'source> LineColLookup<'source> {
    /// Creates a new line/col lookup table. The `src` parameter provides the input string used to calculate lines and columns.
    ///
    /// Internally, this scans `src` and caches the starting positions of all lines. This means this is an O(n) operation.
    pub fn new(src: &'source str) -> Self {
        let line_heads: Vec<usize> = std::iter::once(0)
            .chain(src
                .char_indices()
                .filter_map(|(i, c)| Some(i + 1).filter(|_| c == '\n')))
            .collect();

        let line_count = line_heads.len();

        Self {
            src,
            line_heads,
            line_count
        }
    }

    /// Looks up the 1-based line and column numbers of the specified char index.
    /// If the `grapheme-clusters` feature is enabled, the column represents the nearest grapheme cluster rather than character.
    ///
    /// Returns a tuple with the line number first, then column number. 
    ///
    /// # Example
    /// ```rust
    /// use line_col::*;
    /// let text = "One\nTwo";
    /// let lookup = LineColLookup::new(text);
    /// assert_eq!(lookup.get(0), (1, 1)); // 'O' (line 1, col 1)
    /// assert_eq!(lookup.get(1), (1, 2)); // 'n' (line 1, col 2)
    /// assert_eq!(lookup.get(2), (1, 3)); // 'e' (line 1, col 3)
    /// assert_eq!(lookup.get(4), (2, 1)); // 'T' (line 2, col 1)
    /// assert_eq!(lookup.get(5), (2, 2)); // 'w' (line 2, col 2)
    /// assert_eq!(lookup.get(6), (2, 3)); // 'o' (line 2, col 3)
    /// assert_eq!(lookup.get(7), (2, 4)); // <end> (line 2, col 4)
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than the length of the input `&str`.
    ///
    /// # Notes
    /// This function uses a binary search to locate the line on which `index` resides.
    /// This means that it runs in approximately O(log n) time.
    pub fn get(&self, index: usize) -> (usize, usize) {
        if index > self.src.len() {
            panic!("Index cannot be greater than the length of the input slice.");
        }

        // Perform a binary search to locate the line on which `index` resides
        let mut line_range = 0 .. self.line_count;
        while line_range.end - line_range.start > 1 {
            let range_middle = line_range.start + (line_range.end - line_range.start) / 2;
            let (left, right) = (line_range.start..range_middle, range_middle..line_range.end);
            // Check which line window contains our character index
            if (self.line_heads[left.start] .. self.line_heads[left.end]).contains(&index) {
                line_range = left;
            } else {
                line_range = right;
            }
        }

        let line_start_index = self.line_heads[line_range.start];
        let line = line_range.start + 1; 

        #[cfg(feature = "grapheme-clusters")]
        let col = UnicodeSegmentation::graphemes(&self.src[line_start_index..index], true).count() + 1;

        #[cfg(not(feature = "grapheme-clusters"))]
        let col = index - line_start_index + 1;

        (line, col)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn empty_str() {
        let text = "";
        let lookup = LineColLookup::new(text);
        assert_eq!(lookup.get(0), (1, 1));
    }

    #[test]
    #[cfg(not(feature = "grapheme-clusters"))]
    fn line_col_iter_by_codepoints() {
        let text = "a\nab\nabc";
        let lookup = LineColLookup::new(text);
        assert_eq!(lookup.get(0), (1, 1));
        assert_eq!(lookup.get(1), (1, 2));
        assert_eq!(lookup.get(2), (2, 1));
        assert_eq!(lookup.get(3), (2, 2));
        assert_eq!(lookup.get(4), (2, 3));
        assert_eq!(lookup.get(5), (3, 1));
        assert_eq!(lookup.get(6), (3, 2));
        assert_eq!(lookup.get(7), (3, 3));
        assert_eq!(lookup.get(8), (3, 4));
    }

    #[test]
    #[cfg(feature = "grapheme-clusters")]
    fn emoji_text_by_grapheme_clusters() {
        let text = "The 👨‍👩‍👦 emoji is made of 5 code points and 18 bytes in UTF-8.";
        let lookup = LineColLookup::new(text);
        assert_eq!(lookup.get(4), (1, 5));
        assert_eq!(lookup.get(22), (1, 6));
    }

    #[test]
    #[cfg(not(feature = "grapheme-clusters"))]
    fn emoji_text_by_codepoints() {
        let text = "The 👨‍👩‍👦 emoji is made of 5 code points and 18 bytes in UTF-8.";
        let lookup = LineColLookup::new(text);
        assert_eq!(lookup.get(4), (1, 5));
        assert_eq!(lookup.get(22), (1, 23));
    }
}