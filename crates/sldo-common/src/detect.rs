//! Build and test command auto-detection.
//!
//! Mirrors the `detect_commands()` function from `run-milestones.sh`.

use std::fs;
use std::path::Path;

/// Detect build commands based on project files in the given directory.
pub fn detect_build_commands(project_dir: &Path) -> Vec<String> {
    let mut cmds = Vec::new();

    // Rust / Cargo
    if project_dir.join("Cargo.toml").exists() {
        cmds.push("cargo build --workspace".to_string());
    }

    // Node / npm / pnpm / yarn
    if let Some(pm_cmd) = detect_node_build(project_dir) {
        cmds.push(pm_cmd);
    }

    // Go
    if project_dir.join("go.mod").exists() {
        cmds.push("go build ./...".to_string());
    }

    // Makefile
    if let Some(makefile_cmd) = detect_makefile_target(project_dir, "build") {
        cmds.push(makefile_cmd);
    }

    cmds
}

/// Detect test commands based on project files in the given directory.
pub fn detect_test_commands(project_dir: &Path) -> Vec<String> {
    let mut cmds = Vec::new();

    // Rust / Cargo
    if project_dir.join("Cargo.toml").exists() {
        cmds.push("cargo test --workspace".to_string());
    }

    // Node / npm / pnpm / yarn
    if let Some(pm_cmd) = detect_node_test(project_dir) {
        cmds.push(pm_cmd);
    }

    // Python
    if detect_python_project(project_dir) {
        if let Some(pytest_cmd) = detect_pytest(project_dir) {
            cmds.push(pytest_cmd);
        }
    }

    // Go
    if project_dir.join("go.mod").exists() {
        cmds.push("go test ./...".to_string());
    }

    // Makefile
    if let Some(makefile_cmd) = detect_makefile_target(project_dir, "test") {
        cmds.push(makefile_cmd);
    }

    cmds
}

/// Detect which package manager to use for Node projects.
fn detect_package_manager(project_dir: &Path) -> Option<&'static str> {
    if project_dir.join("pnpm-lock.yaml").exists() {
        Some("pnpm")
    } else if project_dir.join("yarn.lock").exists() {
        Some("yarn")
    } else if project_dir.join("package-lock.json").exists()
        || project_dir.join("package.json").exists()
    {
        Some("npm")
    } else {
        None
    }
}

/// Check if package.json has a specific script.
fn package_json_has_script(project_dir: &Path, script: &str) -> bool {
    let pkg_path = project_dir.join("package.json");
    if let Ok(content) = fs::read_to_string(pkg_path) {
        content.contains(&format!("\"{}\"", script))
    } else {
        false
    }
}

fn detect_node_build(project_dir: &Path) -> Option<String> {
    let pm = detect_package_manager(project_dir)?;
    if package_json_has_script(project_dir, "build") {
        Some(format!("{} run build", pm))
    } else {
        None
    }
}

fn detect_node_test(project_dir: &Path) -> Option<String> {
    let pm = detect_package_manager(project_dir)?;
    if package_json_has_script(project_dir, "test") {
        Some(format!("{} test", pm))
    } else {
        None
    }
}

fn detect_python_project(project_dir: &Path) -> bool {
    project_dir.join("pyproject.toml").exists()
        || project_dir.join("setup.py").exists()
        || project_dir.join("setup.cfg").exists()
}

fn detect_pytest(project_dir: &Path) -> Option<String> {
    let pyproject = project_dir.join("pyproject.toml");
    if pyproject.exists() {
        if let Ok(content) = fs::read_to_string(pyproject) {
            if content.contains("pytest") {
                return Some("pytest".to_string());
            }
        }
    }
    // Fallback: check if pytest is available (simplified — just suggest it)
    None
}

fn detect_makefile_target(project_dir: &Path, target: &str) -> Option<String> {
    let makefile = project_dir.join("Makefile");
    if makefile.exists() {
        if let Ok(content) = fs::read_to_string(makefile) {
            let pattern = format!("{}:", target);
            if content.lines().any(|line| line.starts_with(&pattern)) {
                return Some(format!("make {}", target));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_cargo_project() {
        // Given: Directory with Cargo.toml (this repo's root)
        let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        // When: detect_build_commands(dir) is called
        let cmds = detect_build_commands(project_dir);
        // Then: Contains "cargo build --workspace"
        assert!(
            cmds.contains(&"cargo build --workspace".to_string()),
            "Expected 'cargo build --workspace' in {:?}",
            cmds
        );
    }

    #[test]
    fn detect_cargo_test_commands() {
        // Given: Directory with Cargo.toml
        let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        // When: detect_test_commands(dir) is called
        let cmds = detect_test_commands(project_dir);
        // Then: Contains "cargo test --workspace"
        assert!(
            cmds.contains(&"cargo test --workspace".to_string()),
            "Expected 'cargo test --workspace' in {:?}",
            cmds
        );
    }

    #[test]
    fn detect_npm_project_with_build_script() {
        // Given: Directory with package.json containing "build" script
        let tmp = std::env::temp_dir().join("sldo_test_detect_npm");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(
            tmp.join("package.json"),
            r#"{"scripts": {"build": "tsc", "test": "vitest"}}"#,
        )
        .unwrap();

        // When: detect_build_commands(dir) is called
        let cmds = detect_build_commands(&tmp);
        // Then: Contains "npm run build"
        assert!(
            cmds.contains(&"npm run build".to_string()),
            "Expected 'npm run build' in {:?}",
            cmds
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn no_build_files_empty_dir() {
        // Given: Empty directory
        let tmp = std::env::temp_dir().join("sldo_test_detect_empty");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();

        // When: detect_build_commands(dir) is called
        let cmds = detect_build_commands(&tmp);
        // Then: Returns empty Vec
        assert!(cmds.is_empty());

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn detect_go_project() {
        // Given: Directory with go.mod
        let tmp = std::env::temp_dir().join("sldo_test_detect_go");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(tmp.join("go.mod"), "module example.com/test\n").unwrap();

        // When: detect_build_commands(dir) is called
        let cmds = detect_build_commands(&tmp);
        // Then: Contains "go build ./..."
        assert!(cmds.contains(&"go build ./...".to_string()));

        let test_cmds = detect_test_commands(&tmp);
        assert!(test_cmds.contains(&"go test ./...".to_string()));

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn detect_makefile_with_build_target() {
        // Given: Directory with Makefile containing build: target
        let tmp = std::env::temp_dir().join("sldo_test_detect_make");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(
            tmp.join("Makefile"),
            "build:\n\techo building\ntest:\n\techo testing\n",
        )
        .unwrap();

        // When: detect_build_commands(dir) is called
        let cmds = detect_build_commands(&tmp);
        assert!(cmds.contains(&"make build".to_string()));

        let test_cmds = detect_test_commands(&tmp);
        assert!(test_cmds.contains(&"make test".to_string()));

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
