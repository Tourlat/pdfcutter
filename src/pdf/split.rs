use super::utils::{copy_page_with_resources, create_pages_structure, finalize_document};
use anyhow::{Context, Result, bail};
use lopdf::{Document, ObjectId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageSegment {
    pub start: u32,
    pub end: Option<u32>,
    pub name: Option<String>,
}

impl PageSegment {
    pub fn single(page: u32) -> Self {
        PageSegment {
            start: page,
            end: None,
            name: None,
        }
    }

    pub fn range(start: u32, end: u32) -> Self {
        Self {
            start,
            end: Some(end),
            name: None,
        }
    }

    pub fn named(start: u32, end: Option<u32>, name: String) -> Self {
        Self {
            start,
            end,
            name: Some(name),
        }
    }

    pub fn get_pages(&self) -> Vec<u32> {
        match self.end {
            Some(end) => (self.start..=end).collect(),
            None => vec![self.start],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.start > 0 && self.end.map_or(true, |end| end >= self.start)
    }

    pub fn generate_filename(&self, base_prefix: &str) -> String {
        if let Some(ref name) = self.name {
            format!("{}_{}.pdf", base_prefix, name)
        } else {
            match self.end {
                Some(end) if end != self.start => {
                    format!("{}_pages_{}_{}.pdf", base_prefix, self.start, end)
                }
                _ => {
                    format!("{}_page_{}.pdf", base_prefix, self.start)
                }
            }
        }
    }
}

/**
 * Parse the input string into a vector of PageSegment.
 * @param input The input string (e.g., "1,3-5,(7,9),11")
 * @returns A vector of PageSegment if valid, Err(anyhow::Error) if invalid.
 * @throws anyhow::Error if the input is invalid.
 */
pub fn parse_page_segments(input: &str) -> Result<Vec<PageSegment>> {
    let mut segments = Vec::new();

    let clean_input = input.replace(['(', ')'], ",");
    let parts: Vec<&str> = clean_input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    for part in parts {
        let segment = if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                bail!("Invalid range format: {}", part);
            }

            let start: u32 = range_parts[0]
                .trim()
                .parse()
                .with_context(|| format!("Invalid start page: {}", range_parts[0]))?;
            let end: u32 = range_parts[1]
                .trim()
                .parse()
                .with_context(|| format!("Invalid end page: {}", range_parts[1]))?;

            PageSegment::range(start, end)
        } else {
            let page: u32 = part
                .parse()
                .with_context(|| format!("Invalid page number: {}", part))?;
            PageSegment::single(page)
        };

        if !segment.is_valid() {
            bail!("Invalid segment: {:?}", segment);
        }

        segments.push(segment);
    }

    if segments.is_empty() {
        bail!("No valid segments found");
    }

    Ok(segments)
}

/**
 * Parse named segments from a string like "intro:1-3,chapter1:4-10,conclusion:11"
 * @param input The input string with named segments
 * @returns A vector of PageSegment with names
 * @throws anyhow::Error if the input is invalid
 */
pub fn parse_named_segments(input: &str) -> Result<Vec<PageSegment>> {
    let mut segments = Vec::new();

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let segment = if part.contains(':') {
            let name_parts: Vec<&str> = part.split(':').collect();
            if name_parts.len() != 2 {
                bail!("Invalid named segment format: {}", part);
            }

            let name = name_parts[0].trim().to_string();
            let page_part = name_parts[1].trim();

            if page_part.contains('-') {
                let range_parts: Vec<&str> = page_part.split('-').collect();
                if range_parts.len() != 2 {
                    bail!("Invalid range in named segment: {}", page_part);
                }

                let start: u32 = range_parts[0].parse()?;
                let end: u32 = range_parts[1].parse()?;

                PageSegment::named(start, Some(end), name)
            } else {
                let page: u32 = page_part.parse()?;
                PageSegment::named(page, None, name)
            }
        } else {
            parse_page_segments(part)?.into_iter().next().unwrap()
        };

        if !segment.is_valid() {
            bail!("Invalid segment: {:?}", segment);
        }

        segments.push(segment);
    }

    Ok(segments)
}

/**
 * Create a PDF document containing only the pages specified in the segment.
 */
fn create_pdf_with_segment(
    source_doc: &Document,
    segment: &PageSegment,
    all_pages: &BTreeMap<u32, (u32, u16)>,
    total_pages: usize,
) -> Result<Document> {
    let pages_to_include = segment.get_pages();

    for &page_num in &pages_to_include {
        if page_num == 0 || page_num > total_pages as u32 {
            return Err(anyhow::anyhow!(
                "Invalid page number: {}. PDF has {} pages (1-{})",
                page_num,
                total_pages,
                total_pages
            ));
        }
    }

    let include_set: std::collections::HashSet<usize> =
        pages_to_include.iter().map(|&p| (p - 1) as usize).collect();

    let mut pages_to_keep = Vec::new();
    for (index, (_page_no, page_id)) in all_pages.iter().enumerate() {
        if include_set.contains(&index) {
            pages_to_keep.push(*page_id);
        }
    }

    if pages_to_keep.is_empty() {
        return Err(anyhow::anyhow!("No pages to include in PDF"));
    }

    let mut target = Document::with_version("1.5");
    let mut page_objects: Vec<ObjectId> = Vec::new();

    for page_id in pages_to_keep {
        let new_page_id = copy_page_with_resources(source_doc, page_id, &mut target)?;
        page_objects.push(new_page_id);
    }

    create_pages_structure(&mut target, &page_objects)?;

    Ok(target)
}

/**
 * Split PDF based on provided segments
 */
pub fn split_pdfs_with_segments(
    input: &str,
    output_prefix: &str,
    segments: &[PageSegment],
) -> Result<Vec<String>> {
    let doc = Document::load(input).with_context(|| format!("Failed to load PDF '{}'", input))?;

    let all_pages = doc.get_pages();
    let total_pages = all_pages.len();

    if total_pages == 0 {
        return Err(anyhow::anyhow!("PDF has no pages"));
    }

    let mut output_files = Vec::new();

    for segment in segments {
        let output_filename = segment.generate_filename(output_prefix);

        let mut target_doc = create_pdf_with_segment(&doc, segment, &all_pages, total_pages)?;

        finalize_document(&mut target_doc, &output_filename)
            .with_context(|| format!("Failed to save PDF '{}'", output_filename))?;

        let pages = segment.get_pages();
        output_files.push(output_filename.clone());
        println!(
            "Created: {} (pages: {})",
            output_filename,
            pages
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    Ok(output_files)
}

/**
 * Split PDF into multiple PDFs based on simple page segments
 * @param input The input PDF file path
 * @param output_prefix The prefix for output files
 * @param segments_str The segments string (e.g., "1-3,5,
 * 7")
 * @returns A vector of output file names
 * @throws anyhow::Error if any error occurs
 */
pub fn split_pdfs(input: &str, output_prefix: &str, segments_str: &str) -> Result<Vec<String>> {
    let segments = parse_page_segments(segments_str)?;
    split_pdfs_with_segments(input, output_prefix, &segments)
}

/**
 * Split PDF into multiple PDFs based on named segments
 * @param input The input PDF file path
 * @param output_prefix The prefix for output files
 * @param segments_str The named segments string (e.g., "intro:1-3,chapter1:4-10,conclusion:11")
 * @returns A vector of output file names
 * @throws anyhow::Error if any error occurs
 */
pub fn split_pdfs_named(
    input: &str,
    output_prefix: &str,
    segments_str: &str,
) -> Result<Vec<String>> {
    let segments = parse_named_segments(segments_str)?;
    split_pdfs_with_segments(input, output_prefix, &segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_segment_creation() {
        let single = PageSegment::single(5);
        assert_eq!(single.get_pages(), vec![5]);
        assert_eq!(single.generate_filename("test"), "test_page_5.pdf");

        let range = PageSegment::range(3, 7);
        assert_eq!(range.get_pages(), vec![3, 4, 5, 6, 7]);
        assert_eq!(range.generate_filename("test"), "test_pages_3_7.pdf");

        let named = PageSegment::named(1, Some(3), "intro".to_string());
        assert_eq!(named.generate_filename("test"), "test_intro.pdf");
    }

    #[test]
    fn test_parse_segments() {
        let segments = parse_page_segments("1,3-5,7").unwrap();
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].get_pages(), vec![1]);
        assert_eq!(segments[1].get_pages(), vec![3, 4, 5]);
        assert_eq!(segments[2].get_pages(), vec![7]);
    }
}
