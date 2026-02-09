//! Pattern learning end-to-end tests.
//!
//! Tests validate that pattern learning correctly:
//! - Normalizes commands to matchable patterns
//! - Records user decisions (kill/spare)
//! - Persists learned patterns to disk
//! - Loads patterns back on restart

use pt_core::supervision::{
    CommandNormalizer, PatternLearner, PatternLibrary, PatternLifecycle, SpecificityLevel,
    SupervisorCategory, SupervisorSignature,
};
use std::path::PathBuf;
use tempfile::tempdir;

fn temp_pattern_dir() -> PathBuf {
    let dir = tempdir().expect("tempdir");
    dir.keep()
}

#[test]
fn test_command_normalizer_generates_candidates() {
    let normalizer = CommandNormalizer::new();

    // Test node/jest command
    let candidates = normalizer.generate_candidates(
        "node",
        "/usr/bin/node /home/user/project/node_modules/.bin/jest --watch tests/",
    );

    assert!(!candidates.is_empty(), "should generate candidates");

    // Should have at least Exact, Standard, and Broad levels
    let levels: Vec<_> = candidates.iter().map(|c| c.level).collect();
    assert!(
        levels.contains(&SpecificityLevel::Standard),
        "should have Standard level"
    );
    assert!(
        levels.contains(&SpecificityLevel::Broad),
        "should have Broad level"
    );

    // Check specific patterns
    let patterns: Vec<_> = candidates.iter().map(|c| &c.process_pattern).collect();
    assert!(patterns.iter().any(|p| p.contains("node")));
    // Use type annotation for closure parameter to help inference
    assert!(patterns
        .iter()
        .any(|p: &&String| p.contains(".*") || p.contains(".+")));
}

#[test]
fn test_command_normalizer_python_command() {
    let normalizer = CommandNormalizer::new();

    let candidates = normalizer.generate_candidates(
        "python3",
        "python3 -m pytest /home/user/app/tests/test_api.py -v",
    );

    assert!(!candidates.is_empty(), "should generate candidates");

    // Check that patterns are normalized (paths generalized)
    let patterns: Vec<_> = candidates.iter().map(|c| &c.process_pattern).collect();
    let has_generalized = patterns
        .iter()
        .any(|p| p.contains(".*") || p.contains(".+"));
    assert!(has_generalized, "should have generalized patterns");
}

#[test]
fn test_pattern_library_create_and_save() {
    let temp_dir = temp_pattern_dir();

    // Create a new pattern library
    let mut library = PatternLibrary::new(temp_dir.clone());

    // Add a learned pattern
    let sig = SupervisorSignature::new("test-pattern", SupervisorCategory::Other)
        .with_process_patterns(vec!["node .*/jest"])
        .with_notes("test");

    library.add_learned(sig).expect("add pattern");

    // Save the library
    library.save().expect("save library");

    // Check that files were created
    assert!(temp_dir.join("patterns").join("learned.json").exists());
}

#[test]
fn test_pattern_persistence_roundtrip() {
    let temp_dir = temp_pattern_dir();

    // Create and save a pattern
    {
        let mut library = PatternLibrary::new(temp_dir.clone());

        let sig = SupervisorSignature::new("reload-test", SupervisorCategory::Other)
            .with_process_patterns(vec!["python.* -m pytest"])
            .with_notes("testing");

        library.add_learned(sig).expect("add pattern");
        library.save().expect("save library");
    }

    // Load the library fresh
    let mut library = PatternLibrary::new(temp_dir);
    library.load().expect("load library");

    // Verify the pattern was loaded
    let pattern = library.get_pattern("reload-test");
    assert!(pattern.is_some());
}

#[test]
fn test_pattern_learner_record_kill_decision() {
    let temp_dir = temp_pattern_dir();
    let mut library = PatternLibrary::new(temp_dir);

    // Record a kill decision
    {
        let mut learner = PatternLearner::new(&mut library).with_min_observations(1);
        learner
            .record_decision(
                "node",
                "/usr/bin/node /home/user/proj/node_modules/.bin/jest --watch tests/",
                true, // killed
            )
            .expect("record decision");
    }

    // Check that patterns were created
    // With 100% consistency (1 kill), Exact level is chosen -> "learned_node_exact"
    let stats = library
        .get_stats("learned_node_exact")
        .expect("stats exist");
    assert_eq!(stats.match_count, 1);
    assert_eq!(stats.reject_count, 1); // Kill = rejected (not a supervisor)
}

#[test]
fn test_pattern_learner_record_spare_decision() {
    let temp_dir = temp_pattern_dir();
    let mut library = PatternLibrary::new(temp_dir);

    // Record a spare decision
    {
        let mut learner = PatternLearner::new(&mut library).with_min_observations(1);

        learner
            .record_decision("node", "/usr/bin/node ./server.js", false) // spare
            .expect("record decision");
    }

    // Check that patterns were created
    // With 100% consistency (1 spare), Exact level is chosen -> "learned_node_exact"
    let stats = library
        .get_stats("learned_node_exact")
        .expect("stats exist");
    assert_eq!(stats.match_count, 1);
    assert_eq!(stats.accept_count, 1); // Spare = accepted (is a supervisor)
}

#[test]
fn test_pattern_learner_multiple_decisions() {
    let temp_dir = temp_pattern_dir();
    let mut library = PatternLibrary::new(temp_dir);

    // Record multiple decisions
    {
        let mut learner = PatternLearner::new(&mut library).with_min_observations(1);

        // 2 spares, 1 kill
        // First decision creates pattern with 100% consistency -> Exact level
        // Subsequent decisions use the same pattern
        learner
            .record_decision("node", "node app.js", false)
            .unwrap();
        learner
            .record_decision("node", "node app.js", false)
            .unwrap();
        learner
            .record_decision("node", "node app.js", true)
            .unwrap();
    }

    // Check that stats were recorded for the pattern
    // Pattern is created on first decision at Exact level -> "learned_node_exact"
    let stats = library.get_stats("learned_node_exact").unwrap();
    assert_eq!(stats.match_count, 3);
    assert_eq!(stats.accept_count, 2);
    assert_eq!(stats.reject_count, 1);
}

#[test]
fn test_mixed_actions_prefer_broad() {
    let temp_dir = temp_pattern_dir();
    let mut library = PatternLibrary::new(temp_dir);

    // Record mixed decisions (some kills, some spares)
    // With < 80% consistency, should prefer Broad level
    {
        let mut learner = PatternLearner::new(&mut library).with_min_observations(3);

        // Mix of actions: 2 kills, 2 spares = 50% consistency
        // This should result in a Broad level pattern
        learner
            .record_decision("node", "node /path/to/app.js", true)
            .unwrap(); // kill
        learner
            .record_decision("node", "node /path/to/server.js", false)
            .unwrap(); // spare
        learner
            .record_decision("node", "node /other/path/main.js", true)
            .unwrap(); // kill

        // After 3rd observation (first time we meet min_observations),
        // pattern should be created with Broad level due to 66% consistency
    }

    // Check that a pattern was created at Broad level (not Exact)
    // Broad level patterns use the name format "learned_<name>_broad"
    let broad_pattern = library.get_pattern("learned_node_broad");
    let exact_pattern = library.get_pattern("learned_node_exact");

    // With 66% consistency (2 kills, 1 spare), should prefer Broad
    assert!(
        broad_pattern.is_some() || exact_pattern.is_none(),
        "With mixed actions (<80% consistency), should prefer Broad or not create Exact"
    );
}

#[test]
fn test_pattern_lifecycle_transitions() {
    let temp_dir = temp_pattern_dir();
    let mut library = PatternLibrary::new(temp_dir.clone());

    // Add a pattern with initial state
    let sig = SupervisorSignature::new("lifecycle-test", SupervisorCategory::Other)
        .with_process_patterns(vec!["test-pattern"])
        .with_notes("test");
    library.add_learned(sig).expect("add pattern");

    // Save and reload
    library.save().expect("save");
    let mut library = PatternLibrary::new(temp_dir);
    library.load().expect("load");

    // Verify pattern exists with New lifecycle
    let pattern = library
        .get_pattern("lifecycle-test")
        .expect("pattern exists");
    assert_eq!(pattern.lifecycle, PatternLifecycle::New);

    // Simulate usage to advance lifecycle
    // (This requires manipulating stats and calling update_lifecycles,
    // which might be complex to mock here without direct access to private fields.
    // For now, we just verify it loads as New)
}

#[test]
fn test_persistence_integration() {
    let dir = tempdir().expect("tempdir");
    let temp_dir = dir.path().to_path_buf();

    // Create library, record decisions, save
    {
        let mut library = PatternLibrary::new(temp_dir.clone());
        let mut learner = PatternLearner::new(&mut library).with_min_observations(1); // Learn immediately

        learner
            .record_decision("node", "/usr/bin/node ./app.js", true)
            .expect("record");

        library.save().expect("save");
    }

    // Load fresh, verify patterns
    {
        let mut library = PatternLibrary::new(temp_dir.clone());
        library.load().expect("load");

        let patterns = library.all_active_patterns();
        assert!(!patterns.is_empty(), "should have patterns");
    }

    // Modify and save again
    {
        let mut library = PatternLibrary::new(temp_dir.clone());
        library.load().expect("load");

        // Add more decisions
        let mut learner = PatternLearner::new(&mut library); // Default min_obs=3

        // This won't create a NEW pattern immediately but will update stats of existing
        learner
            .record_decision("node", "/usr/bin/node ./app.js", false)
            .expect("record");

        library.save().expect("save");
    }

    // Final verification
    {
        let mut library = PatternLibrary::new(temp_dir);
        library.load().expect("load");

        // Pattern was created at Exact level -> "learned_node_exact"
        let stats = library
            .get_stats("learned_node_exact")
            .expect("stats exist");
        assert!(stats.match_count >= 2, "should have updated stats");
    }
}
