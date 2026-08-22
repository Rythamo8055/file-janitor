use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub phase: String,
    pub scanned: usize,
    pub total: usize,
    pub percent: u8,
    pub message: String,
}

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
    scan_folders_with_progress(paths, None::<Box<dyn FnMut(ScanProgress) + Send + Sync>>)
}

pub fn scan_folders_with_progress(
    paths: Vec<String>,
    mut progress_cb: Option<Box<dyn FnMut(ScanProgress) + Send + Sync>>,
) -> Result<Vec<FileGroup>, String> {
    let has_progress = progress_cb.is_some();
    let mut emit = |phase: &str, scanned: usize, total: usize, msg: String| {
        if let Some(cb) = progress_cb.as_mut() {
            let percent = if total > 0 { ((scanned as f32 / total as f32) * 100.0) as u8 } else { 0 };
            cb(ScanProgress { phase: phase.to_string(), scanned, total, percent, message: msg });
        }
    };
    emit("collecting", 0, 0, "Finding files...".to_string());
    let files = collect_files(&paths);
    emit("collecting", files.len(), files.len(), format!("Found {} files", files.len()));
    // harden: skip unreadable files, don't fail whole scan on one permission error
    // also cap: if >50K files, warn via error (caller can confirm)
    const MAX_FILES: usize = 50000;
    if files.len() > MAX_FILES {
        return Err(format!("Too many files: {} (limit {}). Pick a smaller folder or use subfolders. Scanned {} paths.", files.len(), MAX_FILES, paths.join(", ")));
    }
    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    let mut skipped = 0usize;
    for f in files {
        let meta = match std::fs::metadata(&f) {
            Ok(m) => m,
            Err(_) => { skipped += 1; continue; }
        };
        if !meta.is_file() { continue; }
        let size = meta.len();
        // harden: skip huge files >2GB to avoid OOM in parallel hash
        const MAX_FILE_SIZE: u64 = 2 * 1024 * 1024 * 1024;
        if size > MAX_FILE_SIZE {
            skipped += 1;
            continue;
        }
        size_map.entry(size).or_default().push(f);
    }

    emit("grouping", size_map.len(), size_map.len(), format!("Grouped by size, {} candidates to hash", {
        let c: usize = size_map.values().filter(|v| v.len()>1).map(|v| v.len()).sum();
        c
    }));
    let mut hash_map: HashMap<String, Vec<FileInfo>> = HashMap::new();
    // only process groups where size has duplicates (or 0-byte special)
    let mut candidates: Vec<PathBuf> = size_map.into_values().filter(|v| v.len() > 1).flatten().collect();
    // harden: cap candidates to avoid hashing 10K+ huge files at once (OOM)
    const MAX_CANDIDATES: usize = 20000;
    let was_capped = candidates.len() > MAX_CANDIDATES;
    if was_capped {
        candidates.truncate(MAX_CANDIDATES);
    }
    emit("hashing", 0, candidates.len(), format!("Hashing {} files...", candidates.len()));

    // hashing with real progress numbers: if progress listener exists, do sequential with per-10 emits for smooth bar; else parallel for speed (tests)
    let hashed: Vec<(PathBuf, String, u64)> = if has_progress {
        let mut out = Vec::new();
        for (idx, p) in candidates.iter().enumerate() {
            if let (Ok(size), Ok(hash)) = (std::fs::metadata(p).map(|m| m.len()), hash_file(p)) {
                out.push((p.clone(), hash, size));
            }
            if (idx + 1) % 10 == 0 || idx + 1 == candidates.len() {
                emit("hashing", idx + 1, candidates.len(), format!("Hashed {} of {} files", idx + 1, candidates.len()));
            }
        }
        out
    } else {
        candidates.par_iter().filter_map(|p| {
            let size = std::fs::metadata(p).ok()?.len();
            let hash = hash_file(p).ok()?;
            Some((p.clone(), hash, size))
        }).collect()
    };
    emit("hashing", candidates.len(), candidates.len(), format!("Hashed {} files", candidates.len()));

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
    // harden: cap groups to 500 max to avoid UI crash rendering 1000s (frontend paginates, but backend also caps)
    const MAX_GROUPS: usize = 500;
    if groups.len() > MAX_GROUPS {
        groups.truncate(MAX_GROUPS);
    }
    emit("done", groups.len(), groups.len(), format!("Found {} duplicate groups", groups.len()));
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

    #[test]
    fn large_folder_no_crash() {
        let dir = tempdir().unwrap();
        // 200 files, 100 dup pairs by size+content
        for i in 0..100 {
            let content = format!("content-{}", i % 10); // 10 distinct contents, each 10 dupes
            std::fs::write(dir.path().join(format!("a{}.txt", i)), content.as_bytes()).unwrap();
            std::fs::write(dir.path().join(format!("b{}.txt", i)), content.as_bytes()).unwrap();
        }
        let groups = scan_folders(vec![dir.path().to_string_lossy().to_string()]).unwrap();
        assert!(groups.len() <= 500, "capped");
        assert!(groups.len() >= 5, "found some dupes");
        // also test skip of unreadable: create a broken symlink
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let _ = symlink("/nonexistent", dir.path().join("broken"));
            let groups2 = scan_folders(vec![dir.path().to_string_lossy().to_string()]).unwrap();
            assert!(groups2.len() >= 5);
        }
    }

    #[test]
    fn permission_skip_no_crash() {
        // single unreadable shouldn't fail whole scan
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("ok1.txt"), b"same").unwrap();
        std::fs::write(dir.path().join("ok2.txt"), b"same").unwrap();
        let groups = scan_folders(vec![dir.path().to_string_lossy().to_string(), "/nonexistent_xyz_123".to_string()]).unwrap();
        assert_eq!(groups.len(), 1);
    }
}
