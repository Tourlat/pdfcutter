use crate::tui::errors::{TuiError, TuiResult};
use lopdf::Document;
use std::path::Path;

/**
 * Validate if the given file path exists and is a valid PDF file.
 * @param input The file path to validate.
 * @returns Ok(()) if valid, Err(TuiError) if invalid.
 * @throws TuiError if the file does not exist or is not a valid PDF.
 */
pub fn validate_file_input(input: &str) -> TuiResult<()> {
    if input.is_empty() {
        return Err(TuiError::FileNotFound {
            path: "empty path".to_string(),
        });
    }

    if !Path::new(input).exists() {
        return Err(TuiError::FileNotFound {
            path: input.to_string(),
        });
    }

    if !is_pdf_file(input) {
        return Err(TuiError::InvalidPdf {
            path: input.to_string(),
        });
    }

    Ok(())
}

/**
 * Check if the required number of files are provided for a merge operation (min two files).
 * @param files The list of file paths to validate.
 * @returns Ok(()) if valid, Err(TuiError) if invalid.
 * @throws TuiError if there are not enough files for merging.
 */
pub fn validate_merge_requirements(files: &[String]) -> TuiResult<()> {
    if files.len() < 2 {
        return Err(TuiError::InsufficientFiles { count: files.len() });
    }
    Ok(())
}

/**
 * Check if exactly one file is provided for a delete operation.
 * @param files The list of file paths to validate.
 * @returns Ok(()) if valid, Err(TuiError) if invalid.
 * @throws TuiError if there are too many files for deletion.
 */
pub fn validate_delete_requirements(files: &[String]) -> TuiResult<()> {
    if files.len() != 1 {
        return Err(TuiError::TooManyFiles { count: files.len() });
    }
    Ok(())
}

/**
 * Check if the given file path points to a valid PDF file by attempting to load it.
 * @param path The file path to check.
 * @returns true if the file is a valid PDF, false otherwise.
 */
fn is_pdf_file(path: &str) -> bool {
    let path = Path::new(path);
    if !path.exists() || path.extension().map(|e| e != "pdf").unwrap_or(true) {
        return false;
    }
    match Document::load(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/**
 * Parse a single page number from a string.
 * @param page_str The string representing a page number.
 * @returns The parsed page number.
 * @throws TuiError if the page number is invalid or zero.
 */
fn parse_single_page(page_str: &str) -> TuiResult<u32> {
    let page: u32 = page_str
        .trim()
        .parse()
        .map_err(|_| TuiError::InvalidPageRange {
            input: format!("Invalid page: {}", page_str),
        })?;

    if page == 0 {
        return Err(TuiError::InvalidPageRange {
            input: "Page numbers must be greater than 0".to_string(),
        });
    }

    Ok(page)
}

/**
 * Parse a page range from a string (e.g., "3-7").
 * @param range_str The string representing a page range.
 * @returns A vector of page numbers in the range.
 * @throws TuiError if the range format is invalid.
 */
fn parse_page_range(range_str: &str) -> TuiResult<Vec<u32>> {
    let range_parts: Vec<&str> = range_str.split('-').collect();

    if range_parts.len() != 2 {
        return Err(TuiError::InvalidPageRange {
            input: format!("Invalid range format: {}", range_str),
        });
    }

    let start = parse_single_page(range_parts[0])?;
    let end = parse_single_page(range_parts[1])?;

    if start > end {
        return Err(TuiError::InvalidPageRange {
            input: format!("Invalid range: {} > {} (start must be <= end)", start, end),
        });
    }

    Ok((start..=end).collect())
}

/**
 * Parse a single part of a page specification (either a single page or a range).
 * @param part The string part to parse.
 * @returns A vector of page numbers.
 * @throws TuiError if the part is invalid.
 */
fn parse_page_part(part: &str) -> TuiResult<Vec<u32>> {
    let part = part.trim();

    if part.is_empty() {
        return Ok(Vec::new());
    }

    if part.contains('-') {
        parse_page_range(part)
    } else {
        let page = parse_single_page(part)?;
        Ok(vec![page])
    }
}

/**
 * Normalize a vector of pages by sorting and removing duplicates.
 * @param pages The vector of page numbers to normalize.
 * @returns The normalized vector.
 * @throws TuiError if no valid pages are provided.
 */
fn normalize_pages(mut pages: Vec<u32>) -> TuiResult<Vec<u32>> {
    if pages.is_empty() {
        return Err(TuiError::InvalidPageRange {
            input: "No valid pages specified".to_string(),
        });
    }

    pages.sort();
    pages.dedup();
    Ok(pages)
}

/**
 * Validate and parse a string representing page ranges (e.g., "1-3,5,7-9").
 * @param pages_str The string representing page ranges.
 * @returns A vector of unique page numbers if valid, Err(TuiError) if invalid.
 * @throws TuiError if the page range string is invalid.
 */
pub fn validate_page_ranges(pages_str: &str) -> TuiResult<Vec<u32>> {
    let mut all_pages = Vec::new();

    for part in pages_str.split(',') {
        let part_pages = parse_page_part(part)?;
        all_pages.extend(part_pages);
    }

    normalize_pages(all_pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_page() {
        assert_eq!(parse_single_page("5").unwrap(), 5);
        assert_eq!(parse_single_page(" 10 ").unwrap(), 10);

        // Error cases
        assert!(parse_single_page("0").is_err());
        assert!(parse_single_page("abc").is_err());
        assert!(parse_single_page("").is_err());
    }

    #[test]
    fn test_parse_page_range() {
        assert_eq!(parse_page_range("3-5").unwrap(), vec![3, 4, 5]);
        assert_eq!(parse_page_range("1-1").unwrap(), vec![1]);
        assert_eq!(parse_page_range(" 2 - 4 ").unwrap(), vec![2, 3, 4]);

        // Error cases
        assert!(parse_page_range("5-3").is_err()); // start > end
        assert!(parse_page_range("1-0").is_err()); // zero page
        assert!(parse_page_range("a-b").is_err()); // invalid numbers
        assert!(parse_page_range("1-2-3").is_err()); // too many parts
    }

    #[test]
    fn test_parse_page_part() {
        assert_eq!(parse_page_part("5").unwrap(), vec![5]);
        assert_eq!(parse_page_part("3-5").unwrap(), vec![3, 4, 5]);
        assert_eq!(parse_page_part("").unwrap(), vec![]); // empty part
        assert_eq!(parse_page_part("  ").unwrap(), vec![]); // whitespace only
    }

    #[test]
    fn test_normalize_pages() {
        assert_eq!(normalize_pages(vec![3, 1, 2, 1]).unwrap(), vec![1, 2, 3]);
        assert_eq!(normalize_pages(vec![5, 5, 5]).unwrap(), vec![5]);

        // Error case
        assert!(normalize_pages(vec![]).is_err());
    }

    #[test]
    fn test_validate_page_ranges() {
        // Valid cases
        assert_eq!(
            validate_page_ranges("1,3,5-7").unwrap(),
            vec![1, 3, 5, 6, 7]
        );
        assert_eq!(validate_page_ranges("2-4,6").unwrap(), vec![2, 3, 4, 6]);
        assert_eq!(validate_page_ranges("10").unwrap(), vec![10]);
        assert_eq!(
            validate_page_ranges("1-3,5,7-9").unwrap(),
            vec![1, 2, 3, 5, 7, 8, 9]
        );
        assert_eq!(
            validate_page_ranges(" 1 , 2 - 3 , 5 ").unwrap(),
            vec![1, 2, 3, 5]
        );

        // Invalid cases
        assert!(validate_page_ranges("3-1").is_err());
        assert!(validate_page_ranges("a,b,c").is_err());
        assert!(validate_page_ranges("0,2-4").is_err());
        assert!(validate_page_ranges("").is_err());
        assert!(validate_page_ranges(",,,").is_err());
    }
}
