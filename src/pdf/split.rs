use super::utils::{copy_page_with_resources, create_pages_structure, finalize_document};
use anyhow::{Context, Result};
use lopdf::{Document, ObjectId};

fn check_pages_to_split_not_empty(pages_to_split: &[i32]) -> Result<()> {
    if pages_to_split.is_empty() {
        return Err(anyhow::anyhow!("No pages specified for splitting"));
    }
    Ok(())
}

fn check_page_range_validity(pages_to_split: &[i32], total_pages: usize) -> Result<()> {
    for &page in pages_to_split {
        if page == 0 || page.abs() as usize > total_pages {
            return Err(anyhow::anyhow!(
                "Invalid page number: {}. PDF has {} pages (1-{})",
                page,
                total_pages,
                total_pages
            ));
        }
    }
    Ok(())
}
pub fn split_pdfs(
    input: &str,
    output_prefix: &str,
    pages_to_split: &[i32],
    nb_pdfs_to_generate: usize,
) -> Result<()> {
    todo!();
}
