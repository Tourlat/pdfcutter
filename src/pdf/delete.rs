use super::utils::{copy_page_with_resources, create_pages_structure, finalize_document};
use anyhow::{Context, Result};
use lopdf::{Document, ObjectId};

/// Delete specified pages from a PDF and save the result
pub fn delete_pages(input: &str, output: &str, pages_to_delete: &[u32]) -> Result<()> {
    let doc = Document::load(input).with_context(|| format!("Failed to load PDF '{}'", input))?;

    let all_pages = doc.get_pages();
    let total_pages = all_pages.len();

    // Validate page numbers
    for &page_num in pages_to_delete {
        if page_num == 0 || page_num > total_pages as u32 {
            return Err(anyhow::anyhow!(
                "Invalid page number: {}. PDF has {} pages (1-{})",
                page_num,
                total_pages,
                total_pages
            ));
        }
    }

    // Create a set for faster lookup of pages to delete (convert to 0-based indexing)
    let delete_set: std::collections::HashSet<usize> =
        pages_to_delete.iter().map(|&p| (p - 1) as usize).collect();

    // Collect pages to keep
    let mut pages_to_keep = Vec::new();
    for (index, (_page_no, page_id)) in all_pages.iter().enumerate() {
        if !delete_set.contains(&index) {
            pages_to_keep.push(*page_id);
        }
    }

    if pages_to_keep.is_empty() {
        return Err(anyhow::anyhow!("Cannot delete all pages from PDF"));
    }

    // Create a new document with only the pages we want to keep
    let mut target = Document::with_version("1.5");
    let mut page_objects: Vec<ObjectId> = Vec::new();

    // Copy each page we want to keep
    for page_id in pages_to_keep {
        let new_page_id = copy_page_with_resources(&doc, page_id, &mut target)?;
        page_objects.push(new_page_id);
    }

    // Create the document structure and save
    create_pages_structure(&mut target, &page_objects)?;
    finalize_document(&mut target, output)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_delete_pages() {
        let input = "tests/tests_pdf/a.pdf";
        let output = "test_delete_output.pdf";

        if !Path::new(input).exists() {
            panic!("Test file {} does not exist", input);
        }

        // First, check how many pages the input has
        let original_doc = Document::load(input).unwrap();
        let original_pages = original_doc.get_pages();
        let original_count = original_pages.len();

        if original_count < 2 {
            // Skip test if PDF doesn't have enough pages
            return;
        }

        // Delete the first page
        let pages_to_delete = vec![1];
        let result = delete_pages(input, output, &pages_to_delete);

        assert!(
            result.is_ok(),
            "Delete pages should succeed: {:?}",
            result.err()
        );
        assert!(Path::new(output).exists(), "Output file should be created");

        // Verify the result
        let result_doc = Document::load(output).unwrap();
        let result_pages = result_doc.get_pages();

        assert_eq!(
            result_pages.len(),
            original_count - 1,
            "Result should have one less page"
        );

        // Clean up
        if Path::new(output).exists() {
            std::fs::remove_file(output).unwrap_or_else(|e| {
                eprintln!("Warning: Could not remove test file {}: {}", output, e);
            });
        }
    }

    #[test]
    fn test_delete_invalid_page() {
        let input = "tests/tests_pdf/a.pdf";
        let output = "test_delete_invalid.pdf";

        if !Path::new(input).exists() {
            panic!("Test file {} does not exist", input);
        }

        // Try to delete a page that doesn't exist (page 999)
        let pages_to_delete = vec![999];
        let result = delete_pages(input, output, &pages_to_delete);

        assert!(
            result.is_err(),
            "Delete should fail for invalid page number"
        );
        assert!(
            !Path::new(output).exists(),
            "Output file should not be created on failure"
        );
    }

    #[test]
    fn test_delete_all_pages() {
        let input = "tests/tests_pdf/a.pdf";
        let output = "test_delete_all.pdf";

        if !Path::new(input).exists() {
            panic!("Test file {} does not exist", input);
        }

        let original_doc = Document::load(input).unwrap();
        let original_pages = original_doc.get_pages();
        let page_count = original_pages.len() as u32;

        // Try to delete all pages
        let pages_to_delete: Vec<u32> = (1..=page_count).collect();
        let result = delete_pages(input, output, &pages_to_delete);

        assert!(
            result.is_err(),
            "Delete should fail when trying to delete all pages"
        );
        assert!(
            !Path::new(output).exists(),
            "Output file should not be created on failure"
        );
    }
}
