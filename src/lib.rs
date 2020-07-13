#[cfg(feature = "grapheme-clusters")]
use unicode_segmentation::UnicodeSegmentation;

pub struct LineColLookup<'source> {
    src: &'source str,
    line_heads: Vec<usize>,
    line_count: usize
}

impl<'source> LineColLookup<'source> {
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

    /// Gets the 1-based line and column numbers of the specified char index.
    ///
    /// Returns a tuple with the line number first, then column number. 
    pub fn get(&self, index: usize) -> (usize, usize) {
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
        let col = index - UnicodeSegmentation::grapheme_indices(&self.src[line_start_index..index], true).count() + 1;

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
    fn line_col_iter() {
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
    }
}