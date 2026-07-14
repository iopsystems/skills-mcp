use std::{collections::BTreeMap, path::Path};

use anyhow::{anyhow, bail, Context, Result};
use include_dir::Dir;
use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogIndex {
    pub schema_version: u32,
    pub templates: Vec<CatalogEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CatalogEntry {
    pub id: String,
    pub manifest: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplateManifest {
    pub schema_version: u32,
    pub id: String,
    pub version: String,
    pub purpose: String,
    pub entrypoint: String,
    pub compatibility: Vec<String>,
    pub files: Vec<TemplateFile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplateFile {
    pub path: String,
    pub sha256: String,
    pub merge_strategy: MergeStrategy,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MergeStrategy {
    ThreeWay,
    PreserveLocal,
}

#[derive(Debug, Clone, Serialize)]
pub struct BuildSource {
    pub repository: String,
    pub commit: String,
    pub dirty: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateSummary {
    pub id: String,
    pub version: String,
    pub purpose: String,
    pub compatibility: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateBundleFile {
    pub path: String,
    pub sha256: String,
    pub merge_strategy: MergeStrategy,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateBundle {
    pub source: BuildSource,
    pub manifest: TemplateManifest,
    pub aggregate_sha256: String,
    pub files: Vec<TemplateBundleFile>,
}

#[derive(Debug)]
struct LoadedTemplate {
    manifest: TemplateManifest,
    files: BTreeMap<String, String>,
    aggregate_sha256: String,
    source: BuildSource,
}

#[derive(Debug)]
pub struct TemplateRegistry {
    templates: BTreeMap<String, LoadedTemplate>,
}

impl TemplateRegistry {
    pub fn from_dir(root: &'static Dir<'static>) -> Result<Self> {
        let catalog_file = root
            .get_file("catalog.yaml")
            .ok_or_else(|| anyhow!("embedded template catalog.yaml is missing"))?;
        let catalog = catalog_file
            .contents_utf8()
            .context("embedded template catalog.yaml is not UTF-8")?;
        let mut files = BTreeMap::new();
        collect_files(root, &mut files);
        files.remove("catalog.yaml");
        load_registry(catalog, files, build_source())
    }

    pub fn summaries(&self) -> Vec<TemplateSummary> {
        self.templates
            .values()
            .map(LoadedTemplate::summary)
            .collect()
    }

    pub fn get(&self, id: &str, path: Option<&str>) -> Result<TemplateBundle> {
        let template = self
            .templates
            .get(id)
            .ok_or_else(|| anyhow!("unknown template id {id:?}"))?;
        let declared = template.declared_files();
        let files = match path {
            Some(path) => {
                let descriptor = declared
                    .get(path)
                    .ok_or_else(|| anyhow!("undeclared template path {path:?} for {id:?}"))?;
                vec![template.bundle_file(descriptor)]
            }
            None => declared
                .values()
                .map(|descriptor| template.bundle_file(descriptor))
                .collect(),
        };

        Ok(TemplateBundle {
            source: template.source.clone(),
            manifest: template.manifest.clone(),
            aggregate_sha256: template.aggregate_sha256.clone(),
            files,
        })
    }
}

impl LoadedTemplate {
    fn declared_files(&self) -> BTreeMap<&str, &TemplateFile> {
        self.manifest
            .files
            .iter()
            .map(|file| (file.path.as_str(), file))
            .collect()
    }

    fn summary(&self) -> TemplateSummary {
        TemplateSummary {
            id: self.manifest.id.clone(),
            version: self.manifest.version.clone(),
            purpose: self.manifest.purpose.clone(),
            compatibility: self.manifest.compatibility.clone(),
        }
    }

    fn bundle_file(&self, descriptor: &TemplateFile) -> TemplateBundleFile {
        TemplateBundleFile {
            path: descriptor.path.clone(),
            sha256: descriptor.sha256.clone(),
            merge_strategy: descriptor.merge_strategy.clone(),
            content: self.files[&descriptor.path].clone(),
        }
    }
}

fn load_registry(
    catalog_yaml: &str,
    files: BTreeMap<String, Vec<u8>>,
    source: BuildSource,
) -> Result<TemplateRegistry> {
    let catalog: CatalogIndex =
        serde_yaml::from_str(catalog_yaml).context("failed to parse template catalog.yaml")?;
    if catalog.schema_version != SCHEMA_VERSION {
        bail!(
            "unsupported template catalog schema_version {}",
            catalog.schema_version
        );
    }

    let mut templates = BTreeMap::new();
    for entry in catalog.templates {
        if templates.contains_key(&entry.id) {
            bail!("duplicate catalog template id {:?}", entry.id);
        }
        validate_relative_path(&entry.manifest, "catalog manifest path")?;
        let manifest_bytes = files
            .get(&entry.manifest)
            .ok_or_else(|| anyhow!("template manifest {:?} is missing", entry.manifest))?;
        let manifest_yaml = std::str::from_utf8(manifest_bytes)
            .with_context(|| format!("template manifest {:?} is not UTF-8", entry.manifest))?;
        let manifest: TemplateManifest = serde_yaml::from_str(manifest_yaml)
            .with_context(|| format!("failed to parse template manifest {:?}", entry.manifest))?;
        let template = load_template(&entry, manifest, &files, source.clone())?;
        templates.insert(entry.id, template);
    }

    Ok(TemplateRegistry { templates })
}

fn load_template(
    entry: &CatalogEntry,
    manifest: TemplateManifest,
    files: &BTreeMap<String, Vec<u8>>,
    source: BuildSource,
) -> Result<LoadedTemplate> {
    if manifest.schema_version != SCHEMA_VERSION {
        bail!(
            "unsupported template manifest schema_version {} for {:?}",
            manifest.schema_version,
            entry.id
        );
    }
    if manifest.id != entry.id {
        bail!(
            "manifest id {:?} does not match catalog id {:?}",
            manifest.id,
            entry.id
        );
    }
    Version::parse(&manifest.version)
        .with_context(|| format!("invalid semantic version {:?}", manifest.version))?;

    validate_relative_path(&manifest.entrypoint, "entrypoint")?;
    let mut declared = BTreeMap::new();
    for file in &manifest.files {
        validate_relative_path(&file.path, "declared path")?;
        validate_sha256(&file.sha256, &file.path)?;
        if declared.insert(file.path.as_str(), file).is_some() {
            bail!("duplicate declared path {:?}", file.path);
        }
    }
    if !declared.contains_key(manifest.entrypoint.as_str()) {
        bail!(
            "template entrypoint is not declared: {:?}",
            manifest.entrypoint
        );
    }

    let template_root = Path::new(&entry.manifest)
        .parent()
        .and_then(Path::to_str)
        .unwrap_or("");
    let mut loaded_files = BTreeMap::new();
    for file in declared.values() {
        let embedded_path = if template_root.is_empty() {
            file.path.clone()
        } else {
            format!("{template_root}/{}", file.path)
        };
        let bytes = files.get(&embedded_path).ok_or_else(|| {
            anyhow!(
                "declared template file {:?} is missing for {:?}",
                file.path,
                manifest.id
            )
        })?;
        let content = std::str::from_utf8(bytes).with_context(|| {
            format!(
                "declared template file {:?} is not UTF-8 for {:?}",
                file.path, manifest.id
            )
        })?;
        let actual = sha256(bytes);
        if actual != file.sha256 {
            bail!(
                "SHA-256 mismatch for {:?}: expected {}, got {}",
                file.path,
                file.sha256,
                actual
            );
        }
        loaded_files.insert(file.path.clone(), content.to_owned());
    }

    let aggregate_sha256 = aggregate_digest(declared.values().copied());
    Ok(LoadedTemplate {
        manifest,
        files: loaded_files,
        aggregate_sha256,
        source,
    })
}

fn validate_relative_path(path: &str, kind: &str) -> Result<()> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains('\\')
        || path
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..")
    {
        bail!("invalid {kind} {path:?}");
    }
    Ok(())
}

fn validate_sha256(value: &str, path: &str) -> Result<()> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        bail!("invalid lowercase SHA-256 {:?} for {path:?}", value);
    }
    Ok(())
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn aggregate_digest<'a>(files: impl Iterator<Item = &'a TemplateFile>) -> String {
    let mut hasher = Sha256::new();
    for file in files {
        hasher.update(file.path.as_bytes());
        hasher.update([0]);
        hasher.update(file.sha256.as_bytes());
        hasher.update(b"\n");
    }
    format!("{:x}", hasher.finalize())
}

fn collect_files(dir: &'static Dir<'static>, files: &mut BTreeMap<String, Vec<u8>>) {
    for file in dir.files() {
        files.insert(
            file.path().to_string_lossy().into_owned(),
            file.contents().to_vec(),
        );
    }
    for child in dir.dirs() {
        collect_files(child, files);
    }
}

fn build_source() -> BuildSource {
    BuildSource {
        repository: env!("IOP_SKILLS_SOURCE_REPOSITORY").to_owned(),
        commit: env!("IOP_SKILLS_SOURCE_COMMIT").to_owned(),
        dirty: env!("IOP_SKILLS_SOURCE_DIRTY") == "true",
    }
}

#[cfg(test)]
fn test_source() -> BuildSource {
    BuildSource {
        repository: "test://repository".to_owned(),
        commit: "0000000000000000000000000000000000000000".to_owned(),
        dirty: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    const HELLO_SHA256: &str = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    const WORLD_SHA256: &str = "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7";

    fn catalog(entries: &str) -> String {
        format!("schema_version: 1\ntemplates:\n{entries}")
    }

    fn manifest(id: &str, version: &str, entrypoint: &str, files: &str) -> String {
        format!(
            "schema_version: 1\nid: {id}\nversion: {version}\npurpose: Test template\nentrypoint: {entrypoint}\ncompatibility:\n  - codex\nfiles:\n{files}"
        )
    }

    fn declared_file(path: &str, sha256: &str) -> String {
        format!("  - path: {path}\n    sha256: {sha256}\n    merge_strategy: three-way\n")
    }

    fn valid_fixture() -> (String, BTreeMap<String, Vec<u8>>) {
        let catalog = catalog("  - id: starter\n    manifest: starter/manifest.yaml\n");
        let manifest = manifest(
            "starter",
            "1.2.3",
            "SKILL.md",
            &declared_file("SKILL.md", HELLO_SHA256),
        );
        let files = BTreeMap::from([
            ("starter/manifest.yaml".to_owned(), manifest.into_bytes()),
            ("starter/SKILL.md".to_owned(), b"hello".to_vec()),
            (
                "starter/undeclared.txt".to_owned(),
                b"not retrievable".to_vec(),
            ),
        ]);
        (catalog, files)
    }

    fn error_for(catalog: &str, files: BTreeMap<String, Vec<u8>>) -> String {
        load_registry(catalog, files, test_source())
            .expect_err("fixture should be rejected")
            .to_string()
    }

    #[test]
    fn parses_valid_catalog_and_manifest() {
        let (catalog, files) = valid_fixture();
        let registry = load_registry(&catalog, files, test_source()).unwrap();

        let summaries = registry.summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].id, "starter");
        assert_eq!(summaries[0].version, "1.2.3");
        assert_eq!(
            serde_json::to_value(&summaries[0]).unwrap(),
            serde_json::json!({
                "id": "starter",
                "version": "1.2.3",
                "purpose": "Test template",
                "compatibility": ["codex"]
            })
        );

        let bundle = registry.get("starter", None).unwrap();
        let serialized = serde_json::to_value(&bundle).unwrap();
        assert_eq!(
            serialized,
            serde_json::json!({
                "source": {
                    "repository": "test://repository",
                    "commit": "0000000000000000000000000000000000000000",
                    "dirty": false
                },
                "manifest": {
                    "schema_version": 1,
                    "id": "starter",
                    "version": "1.2.3",
                    "purpose": "Test template",
                    "entrypoint": "SKILL.md",
                    "compatibility": ["codex"],
                    "files": [{
                        "path": "SKILL.md",
                        "sha256": HELLO_SHA256,
                        "merge_strategy": "three-way"
                    }]
                },
                "aggregate_sha256":
                    "5db4717b2be68db0dd10d52cd74360a4b433eb6563fd0bd3e3805ee038b942fb",
                "files": [{
                    "path": "SKILL.md",
                    "sha256": HELLO_SHA256,
                    "merge_strategy": "three-way",
                    "content": "hello"
                }]
            })
        );
    }

    #[test]
    fn rejects_duplicate_catalog_id() {
        let (catalog, files) = valid_fixture();
        let duplicate = catalog.replace(
            "  - id: starter\n    manifest: starter/manifest.yaml\n",
            "  - id: starter\n    manifest: starter/manifest.yaml\n  - id: starter\n    manifest: starter/manifest.yaml\n",
        );

        assert!(error_for(&duplicate, files).contains("duplicate catalog template id"));
    }

    #[test]
    fn rejects_catalog_id_not_matching_manifest_id() {
        let (catalog, mut files) = valid_fixture();
        let raw = String::from_utf8(files.remove("starter/manifest.yaml").unwrap()).unwrap();
        files.insert(
            "starter/manifest.yaml".to_owned(),
            raw.replace("id: starter", "id: other").into_bytes(),
        );

        assert!(error_for(&catalog, files).contains("does not match catalog id"));
    }

    #[test]
    fn rejects_invalid_semantic_version() {
        let (catalog, mut files) = valid_fixture();
        let raw = String::from_utf8(files.remove("starter/manifest.yaml").unwrap()).unwrap();
        files.insert(
            "starter/manifest.yaml".to_owned(),
            raw.replace("version: 1.2.3", "version: latest")
                .into_bytes(),
        );

        assert!(error_for(&catalog, files).contains("invalid semantic version"));
    }

    #[test]
    fn rejects_invalid_declared_paths() {
        for path in [
            "/absolute.md",
            "../parent.md",
            "",
            "directory//empty.md",
            r"directory\windows.md",
        ] {
            let (catalog, mut files) = valid_fixture();
            let raw = String::from_utf8(files.remove("starter/manifest.yaml").unwrap()).unwrap();
            let yaml_path = if path.is_empty() { "\"\"" } else { path };
            files.insert(
                "starter/manifest.yaml".to_owned(),
                raw.replace("path: SKILL.md", &format!("path: {yaml_path}"))
                    .into_bytes(),
            );

            let error = error_for(&catalog, files);
            assert!(
                error.contains("invalid declared path"),
                "path {path:?} produced: {error}"
            );
        }
    }

    #[test]
    fn rejects_duplicate_declared_paths() {
        let (catalog, mut files) = valid_fixture();
        let raw = String::from_utf8(files.remove("starter/manifest.yaml").unwrap()).unwrap();
        let duplicate = format!(
            "{}{}",
            declared_file("SKILL.md", HELLO_SHA256),
            declared_file("SKILL.md", HELLO_SHA256)
        );
        files.insert(
            "starter/manifest.yaml".to_owned(),
            raw.replace(&declared_file("SKILL.md", HELLO_SHA256), &duplicate)
                .into_bytes(),
        );

        assert!(error_for(&catalog, files).contains("duplicate declared path"));
    }

    #[test]
    fn rejects_missing_entrypoint() {
        let (catalog, mut files) = valid_fixture();
        let raw = String::from_utf8(files.remove("starter/manifest.yaml").unwrap()).unwrap();
        files.insert(
            "starter/manifest.yaml".to_owned(),
            raw.replace("entrypoint: SKILL.md", "entrypoint: README.md")
                .into_bytes(),
        );

        assert!(error_for(&catalog, files).contains("entrypoint is not declared"));
    }

    #[test]
    fn rejects_undeclared_retrieval_path() {
        let (catalog, files) = valid_fixture();
        let registry = load_registry(&catalog, files, test_source()).unwrap();

        let error = registry
            .get("starter", Some("undeclared.txt"))
            .expect_err("undeclared embedded file must not be retrievable")
            .to_string();
        assert!(error.contains("undeclared template path"));
    }

    #[test]
    fn rejects_file_sha256_mismatch() {
        let (catalog, mut files) = valid_fixture();
        files.insert("starter/SKILL.md".to_owned(), b"changed".to_vec());

        assert!(error_for(&catalog, files).contains("SHA-256 mismatch"));
    }

    #[test]
    fn aggregate_digest_is_stable_independent_of_manifest_file_order() {
        let catalog = catalog("  - id: starter\n    manifest: starter/manifest.yaml\n");
        let first_then_second = format!(
            "{}{}",
            declared_file("a.txt", HELLO_SHA256),
            declared_file("b.txt", WORLD_SHA256)
        );
        let second_then_first = format!(
            "{}{}",
            declared_file("b.txt", WORLD_SHA256),
            declared_file("a.txt", HELLO_SHA256)
        );
        let fixture = |declared: &str| {
            BTreeMap::from([
                (
                    "starter/manifest.yaml".to_owned(),
                    manifest("starter", "1.0.0", "a.txt", declared).into_bytes(),
                ),
                ("starter/a.txt".to_owned(), b"hello".to_vec()),
                ("starter/b.txt".to_owned(), b"world".to_vec()),
            ])
        };

        let first = load_registry(&catalog, fixture(&first_then_second), test_source()).unwrap();
        let second = load_registry(&catalog, fixture(&second_then_first), test_source()).unwrap();

        let first_bundle = first.get("starter", None).unwrap();
        assert_eq!(
            first_bundle.aggregate_sha256,
            "b210f78423e6ee5f7763cc7c6965ac7d551b46d6cad0f71ba7fd28e4255a358e"
        );
        assert_eq!(
            first_bundle.aggregate_sha256,
            second.get("starter", None).unwrap().aggregate_sha256
        );
        assert_eq!(
            first_bundle
                .files
                .iter()
                .map(|file| file.path.as_str())
                .collect::<Vec<_>>(),
            vec!["a.txt", "b.txt"]
        );
    }
}
