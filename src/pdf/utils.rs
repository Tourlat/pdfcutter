use anyhow::Result;
use lopdf::{dictionary, Document, Object, ObjectId};
use std::collections::{HashMap, HashSet, VecDeque};

/// Copy a page and all its resources to the target document
pub fn copy_page_with_resources(source: &Document, page_id: ObjectId, target: &mut Document) -> Result<ObjectId> {
    let mut visited = HashSet::new();
    let mut to_copy = VecDeque::new();
    let mut id_mapping = HashMap::new();
    
    // Start with the page object
    to_copy.push_back(page_id);
    
    // Breadth-first traversal to collect all referenced objects
    while let Some(current_id) = to_copy.pop_front() {
        if visited.contains(&current_id) {
            continue;
        }
        visited.insert(current_id);
        
        if let Ok(obj) = source.get_object(current_id) {
            // Find all object references in this object
            collect_references(obj, &mut to_copy);
        }
    }
    
    // Copy all collected objects to target document
    for &obj_id in &visited {
        if let Ok(obj) = source.get_object(obj_id) {
            let new_id = target.add_object(obj.clone());
            id_mapping.insert(obj_id, new_id);
        }
    }
    
    // Update all references in the copied objects
    for &new_id in id_mapping.values() {
        if let Ok(obj) = target.get_object_mut(new_id) {
            update_references(obj, &id_mapping);
        }
    }
    
    // Return the new page ID
    Ok(id_mapping[&page_id])
}

/// Create the Pages structure for a PDF document
pub fn create_pages_structure(target: &mut Document, page_objects: &[ObjectId]) -> Result<()> {
    // Create Pages root object
    let pages_id = target.new_object_id();
    let kids: Vec<Object> = page_objects.iter()
        .map(|&id| Object::Reference(id)).collect();
    
    let pages_dict = dictionary! {
        "Type" => "Pages",
        "Kids" => kids,
        "Count" => (page_objects.len() as i64),
    };
    target.objects.insert(pages_id, Object::Dictionary(pages_dict));

    // Update all pages to reference the new Pages parent
    for &page_id in page_objects {
        if let Ok(page_obj) = target.get_object_mut(page_id) {
            if let Ok(page_dict) = page_obj.as_dict_mut() {
                page_dict.set("Parent", pages_id);
            }
        }
    }

    // Create Catalog
    let catalog_id = target.new_object_id();
    target.objects.insert(
        catalog_id,
        Object::Dictionary(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id
        }),
    );

    // Set up the document trailer
    target.trailer.set("Root", catalog_id);
    
    Ok(())
}

/// Finalize and save the PDF document
pub fn finalize_document(target: &mut Document, output: &str) -> Result<()> {
    target.max_id = target.objects.len() as u32;
    target.renumber_objects();
    target.adjust_zero_pages();
    target.save(output)?;
    Ok(())
}

/// Collect all object references from an object
fn collect_references(obj: &Object, to_copy: &mut VecDeque<ObjectId>) {
    match obj {
        Object::Reference(id) => {
            to_copy.push_back(*id);
        }
        Object::Dictionary(dict) => {
            for (_, value) in dict.iter() {
                collect_references(value, to_copy);
            }
        }
        Object::Array(arr) => {
            for item in arr {
                collect_references(item, to_copy);
            }
        }
        Object::Stream(stream) => {
            collect_references(&Object::Dictionary(stream.dict.clone()), to_copy);
        }
        _ => {}
    }
}

/// Update object references in a copied object
fn update_references(obj: &mut Object, id_mapping: &HashMap<ObjectId, ObjectId>) {
    match obj {
        Object::Reference(id) => {
            if let Some(&new_id) = id_mapping.get(id) {
                *id = new_id;
            }
        }
        Object::Dictionary(dict) => {
            for (_, value) in dict.iter_mut() {
                update_references(value, id_mapping);
            }
        }
        Object::Array(arr) => {
            for item in arr.iter_mut() {
                update_references(item, id_mapping);
            }
        }
        Object::Stream(stream) => {
            for (_, value) in stream.dict.iter_mut() {
                update_references(value, id_mapping);
            }
        }
        _ => {}
    }
}