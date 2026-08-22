use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub hash: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileGroup {
    pub hash: String,
    pub size: u64,
    pub count: usize,
    pub files: Vec<FileInfo>,
    pub wasted: u64, // (count-1)*size
}

pub fn hash_file(path: &Path) -> Result<String, String> {
    let file = File::open(path).map_err(|e| format!("open {}: {}", path.display(), e))?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = reader.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

pub fn collect_files(paths: &[String]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for p in paths {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            out.push(pb);
        } else if pb.is_dir() {
            for entry in WalkDir::new(&pb).follow_links(false).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    out.push(entry.path().to_path_buf());
                }
            }
        }
    }
    out
}

pub fn scan_folders(paths: Vec<String>) -> Result<Vec<FileGroup>, String> {
    let files = collect_files(&paths);
    // group by size first (fast), then hash only size groups >1
    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for f in files {
        let meta = std::fs::metadata(&f).map_err(|e| e.to_string())?;
        if !meta.is_file() { continue; }
        let size = meta.len();
        size_map.entry(size).or_default().push(f);
    }

    let mut hash_map: HashMap<String, Vec<FileInfo>> = HashMap::new();
    // only process groups where size has duplicates (or 0-byte special)
    let candidates: Vec<PathBuf> = size_map.into_values().filter(|v| v.len() > 1).flatten().collect();

    // parallel hash
    let hashed: Vec<(PathBuf, String, u64)> = candidates.par_iter().filter_map(|p| {
        let size = std::fs::metadata(p).ok()?.len();
        let hash = hash_file(p).ok()?;
        Some((p.clone(), hash, size))
    }).collect();

    for (path, hash, size) in hashed {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let info = FileInfo { path: path.to_string_lossy().to_string(), size, hash: hash.clone(), name };
        hash_map.entry(hash).or_default().push(info);
    }

    let mut groups: Vec<FileGroup> = hash_map.into_iter().filter_map(|(hash, mut files)| {
        if files.len() < 2 { return None; }
        files.sort_by(|a,b| a.path.cmp(&b.path));
        let size = files[0].size;
        let count = files.len();
        let wasted = (count as u64 - 1) * size;
        Some(FileGroup { hash, size, count, files, wasted })
    }).collect();

    groups.sort_by(|a,b| b.wasted.cmp(&a.wasted));
    Ok(groups)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn hash_identical_same() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("a.txt");
        let p2 = dir.path().join("b.txt");
        std::fs::write(&p1, b"hello world").unwrap();
        std::fs::write(&p2, b"hello world").unwrap();
        assert_eq!(hash_file(&p1).unwrap(), hash_file(&p2).unwrap());
    }
    #[test]
    fn hash_different_diff() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("a.txt");
        let p2 = dir.path().join("b.txt");
        std::fs::write(&p1, b"hello").unwrap();
        std::fs::write(&p2, b"world").unwrap();
        assert_ne!(hash_file(&p1).unwrap(), hash_file(&p2).unwrap());
    }
    #[test]
    fn scan_groups() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("a.txt");
        let p2 = dir.path().join("b.txt");
        let p3 = dir.path().join("c.txt");
        std::fs::write(&p1, b"dup").unwrap();
        std::fs::write(&p2, b"dup").unwrap();
        std::fs::write(&p3, b"unique").unwrap();
        let groups = scan_folders(vec![dir.path().to_string_lossy().to_string()]).unwrap();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].count, 2);
        assert_eq!(groups[0].wasted, 3);
    }
    #[test]
    fn zero_byte_group() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("a"), b"").unwrap();
        std::fs::write(dir.path().join("b"), b"").unwrap();
        let groups = scan_folders(vec![dir.path().to_string_lossy().to_string()]).unwrap();
        assert_eq!(groups.len(), 1);
    }
}
