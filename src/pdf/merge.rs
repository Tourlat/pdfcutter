use super::utils::{copy_page_with_resources, create_pages_structure, finalize_document};
use anyhow::{Context, Result};
use lopdf::{Document, ObjectId};

/**
 * Merge a list of PDFs into a single output file
 *
 * This function copies each page and all its referenced objects (like fonts, images, etc.)
 * from the input PDFs into a new target PDF document. It ensures that all object references
 * are properly maintained across the merge operation.
 *
 * @param inputs List of input PDF file paths
 * @param output Output PDF file path
 */
pub fn merge_pdfs(inputs: &[String], output: &str) -> Result<()> {
    let mut target = Document::with_version("1.5");
    let mut page_objects: Vec<ObjectId> = Vec::new();

    for path in inputs {
        let doc = Document::load(path).with_context(|| format!("Failed to load PDF '{}'", path))?;

        // Get pages from this document
        let pages = doc.get_pages();

        // For each page, copy it and all its referenced objects
        for (_page_no, page_id) in pages {
            let new_page_id = copy_page_with_resources(&doc, page_id, &mut target)?;
            page_objects.push(new_page_id);
        }
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
    fn test_merge_pdfs() {
        // Test file paths
        let input_a = "tests/tests_pdf/a.pdf";
        let input_b = "tests/tests_pdf/b.pdf";
        let output = "test_merged_output.pdf";

        // Check if test files exist
        if !Path::new(input_a).exists() {
            panic!("Test file {} does not exist", input_a);
        }
        if !Path::new(input_b).exists() {
            panic!("Test file {} does not exist", input_b);
        }

        // Test the merge functionality
        let inputs = vec![input_a.to_string(), input_b.to_string()];
        let result = merge_pdfs(&inputs, output);

        // Assert that the merge was successful
        assert!(result.is_ok(), "Merge should succeed: {:?}", result.err());

        // Check that the output file was created
        assert!(Path::new(output).exists(), "Output file should be created");

        // Load the merged PDF to verify it's valid
        let merged_doc = Document::load(output);
        assert!(
            merged_doc.is_ok(),
            "Merged PDF should be loadable: {:?}",
            merged_doc.err()
        );

        let doc = merged_doc.unwrap();
        let pages = doc.get_pages();

        // Verify that we have pages from both input files
        // Assuming a.pdf and b.pdf each have at least 1 page
        assert!(
            pages.len() >= 2,
            "Merged PDF should have at least 2 pages, got {}",
            pages.len()
        );

        // Clean up test file
        if Path::new(output).exists() {
            std::fs::remove_file(output).unwrap_or_else(|e| {
                eprintln!("Warning: Could not remove test file {}: {}", output, e);
            });
        }
    }

    #[test]
    fn test_merge_same_pdf_multiple_times() {
        let input_a = "tests/tests_pdf/a.pdf";
        let output = "test_merged_duplicate.pdf";

        if !Path::new(input_a).exists() {
            panic!("Test file {} does not exist", input_a);
        }

        // Test merging the same PDF multiple times
        let inputs = vec![
            input_a.to_string(),
            input_a.to_string(),
            input_a.to_string(),
        ];
        let result = merge_pdfs(&inputs, output);

        assert!(
            result.is_ok(),
            "Merge with duplicates should succeed: {:?}",
            result.err()
        );
        assert!(Path::new(output).exists(), "Output file should be created");

        // Verify the merged document
        let merged_doc = Document::load(output).unwrap();
        let pages = merged_doc.get_pages();

        // Should have 3 times the pages of the original (assuming a.pdf has at least 1 page)
        assert!(
            pages.len() >= 3,
            "Merged PDF should have at least 3 pages, got {}",
            pages.len()
        );

        // Clean up
        if Path::new(output).exists() {
            std::fs::remove_file(output).unwrap_or_else(|e| {
                eprintln!("Warning: Could not remove test file {}: {}", output, e);
            });
        }
    }

    #[test]
    fn test_merge_nonexistent_file() {
        let inputs = vec!["nonexistent.pdf".to_string()];
        let output = "test_output.pdf";

        let result = merge_pdfs(&inputs, output);

        // Should fail when trying to load a nonexistent file
        assert!(
            result.is_err(),
            "Merge should fail with nonexistent input file"
        );

        // Output file should not be created
        assert!(
            !Path::new(output).exists(),
            "Output file should not be created on failure"
        );
    }

    #[test]
    fn test_merge_empty_input() {
        let inputs: Vec<String> = vec![];
        let output = "test_empty_output.pdf";

        let result = merge_pdfs(&inputs, output);

        // Should handle empty input gracefully
        assert!(result.is_ok(), "Merge with empty input should succeed");

        if Path::new(output).exists() {
            let merged_doc = Document::load(output);
            if let Ok(doc) = merged_doc {
                let pages = doc.get_pages();
                assert_eq!(
                    pages.len(),
                    0,
                    "Merged PDF from empty input should have 0 pages"
                );
            }

            std::fs::remove_file(output).unwrap_or_else(|e| {
                eprintln!("Warning: Could not remove test file {}: {}", output, e);
            });
        }
    }
}
