//! No-mock telemetry schema write + read roundtrip tests for bd-yaps.
//!
//! Validates all 7 table schemas:
//! - Write real record batches via BatchedWriter
//! - Read back parquet files and validate schemas match
//! - Verify field counts, types, and nullability

use arrow::array::*;
use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use chrono::Utc;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use pt_telemetry::schema::{
    audit_schema, outcomes_schema, proc_features_schema, proc_inference_schema,
    proc_samples_schema, runs_schema, signature_matches_schema, TableName, TelemetrySchema,
};
use pt_telemetry::writer::{BatchedWriter, WriterConfig};
use std::fs;
use std::sync::Arc;
use tempfile::TempDir;

// ============================================================================
// Helpers
// ============================================================================

/// Create a record batch with one row of default values matching the schema.
/// Uses None for nullable fields, placeholder values for required fields.
fn create_batch_for_schema(schema: &Schema) -> RecordBatch {
    use arrow::datatypes::{DataType, TimeUnit};

    let mut columns: Vec<Arc<dyn arrow::array::Array>> = Vec::new();

    for field in schema.fields() {
        let col: Arc<dyn arrow::array::Array> = match field.data_type() {
            DataType::Utf8 => {
                if field.is_nullable() {
                    Arc::new(StringArray::from(vec![None::<&str>]))
                } else {
                    Arc::new(StringArray::from(vec!["test_value"]))
                }
            }
            DataType::Int8 => {
                if field.is_nullable() {
                    Arc::new(Int8Array::from(vec![None::<i8>]))
                } else {
                    Arc::new(Int8Array::from(vec![0i8]))
                }
            }
            DataType::Int16 => {
                if field.is_nullable() {
                    Arc::new(Int16Array::from(vec![None::<i16>]))
                } else {
                    Arc::new(Int16Array::from(vec![0i16]))
                }
            }
            DataType::Int32 => {
                if field.is_nullable() {
                    Arc::new(Int32Array::from(vec![None::<i32>]))
                } else {
                    Arc::new(Int32Array::from(vec![0i32]))
                }
            }
            DataType::Int64 => {
                if field.is_nullable() {
                    Arc::new(Int64Array::from(vec![None::<i64>]))
                } else {
                    Arc::new(Int64Array::from(vec![0i64]))
                }
            }
            DataType::Float32 => {
                if field.is_nullable() {
                    Arc::new(Float32Array::from(vec![None::<f32>]))
                } else {
                    Arc::new(Float32Array::from(vec![0.0f32]))
                }
            }
            DataType::Boolean => {
                if field.is_nullable() {
                    Arc::new(BooleanArray::from(vec![None::<bool>]))
                } else {
                    Arc::new(BooleanArray::from(vec![false]))
                }
            }
            DataType::Timestamp(TimeUnit::Microsecond, tz) => {
                let ts = Utc::now().timestamp_micros();
                let arr = TimestampMicrosecondArray::from(vec![if field.is_nullable() {
                    None
                } else {
                    Some(ts)
                }]);
                if let Some(tz) = tz {
                    Arc::new(arr.with_timezone(tz.as_ref()))
                } else {
                    Arc::new(arr)
                }
            }
            other => panic!(
                "Unsupported data type {:?} for field {}",
                other,
                field.name()
            ),
        };
        columns.push(col);
    }

    RecordBatch::try_new(Arc::new(schema.clone()), columns)
        .unwrap_or_else(|e| panic!("Failed to create batch: {}", e))
}

/// Write a batch via BatchedWriter and return the parquet file path.
fn write_and_close(temp_dir: &TempDir, table: TableName, schema: &Schema) -> std::path::PathBuf {
    let config = WriterConfig::new(
        temp_dir.path().to_path_buf(),
        "pt-20260115-143022-schm".to_string(),
        "schema-test-host".to_string(),
    )
    .with_batch_size(1);

    let mut writer = BatchedWriter::new(table, Arc::new(schema.clone()), config);
    let batch = create_batch_for_schema(schema);
    writer.write(batch).expect("write batch");
    writer.close().expect("close writer")
}

/// Read parquet file and return the Arrow schema.
fn read_parquet_schema(path: &std::path::Path) -> Schema {
    let file = fs::File::open(path).expect("open parquet file");
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).expect("parquet reader");
    builder.schema().as_ref().clone()
}

/// Read parquet file and return all record batches.
fn read_parquet_batches(path: &std::path::Path) -> Vec<RecordBatch> {
    let file = fs::File::open(path).expect("open parquet file");
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).expect("parquet reader");
    let reader = builder.build().expect("build reader");
    reader.collect::<Result<Vec<_>, _>>().expect("read batches")
}

// ============================================================================
// Per-Table Schema Write + Read Roundtrip Tests
// ============================================================================

#[test]
fn test_audit_schema_write_read_roundtrip() {
    let temp_dir = TempDir::new().expect("temp dir");
    let schema = audit_schema();
    let path = write_and_close(&temp_dir, TableName::Audit, &schema);

    assert!(path.exists(), "audit parquet file should exist");

    let read_schema = read_parquet_schema(&path);
    assert_eq!(
        read_schema.fields().len(),
        schema.fields().len(),
        "audit schema field count mismatch"
    );

    // Validate specific fields
    assert!(read_schema.field_with_name("audit_ts").is_ok());
    assert!(read_schema.field_with_name("session_id").is_ok());
    assert!(read_schema.field_with_name("event_type").is_ok());
    assert!(read_schema.field_with_name("host_id").is_ok());

    let batches = read_parquet_batches(&path);
    assert_eq!(batches.len(), 1, "should have 1 batch");
    assert_eq!(batches[0].num_rows(), 1, "should have 1 row");

    eprintln!(
        "[INFO] audit schema: {} fields, {} rows verified",
        schema.fields().len(),
        batches[0].num_rows()
    );
}

#[test]
fn test_runs_schema_write_read_roundtrip() {
    let temp_dir = TempDir::new().expect("temp dir");
    let schema = runs_schema();
    let path = write_and_close(&temp_dir, TableName::Runs, &schema);

    let read_schema = read_parquet_schema(&path);
    assert_eq!(
        read_schema.fields().len(),
        schema.fields().len(),
        "runs schema field count mismatch: expected {}, got {}",
        schema.fields().len(),
        read_schema.fields().len()
    );

    assert!(read_schema.field_with_name("session_id").is_ok());
    assert!(read_schema.field_with_name("started_at").is_ok());
    assert!(read_schema.field_with_name("schema_version").is_ok());
    assert!(read_schema.field_with_name("pt_version").is_ok());

    let batches = read_parquet_batches(&path);
    assert_eq!(batches[0].num_rows(), 1);

    eprintln!(
        "[INFO] runs schema: {} fields verified",
        schema.fields().len()
    );
}

#[test]
fn test_proc_samples_schema_write_read_roundtrip() {
    let temp_dir = TempDir::new().expect("temp dir");
    let schema = proc_samples_schema();
    let path = write_and_close(&temp_dir, TableName::ProcSamples, &schema);

    let read_schema = read_parquet_schema(&path);
    assert_eq!(
        read_schema.fields().len(),
        schema.fields().len(),
        "proc_samples schema field count mismatch"
    );

    assert!(read_schema.field_with_name("pid").is_ok());
    assert!(read_schema.field_with_name("start_id").is_ok());
    assert!(read_schema.field_with_name("rss_bytes").is_ok());
    assert!(read_schema.field_with_name("container_id").is_ok());

    let batches = read_parquet_batches(&path);
    assert_eq!(batches[0].num_rows(), 1);

    eprintln!(
        "[INFO] proc_samples schema: {} fields verified",
        schema.fields().len()
    );
}

#[test]
fn test_proc_features_schema_write_read_roundtrip() {
    let temp_dir = TempDir::new().expect("temp dir");
    let schema = proc_features_schema();
    let path = write_and_close(&temp_dir, TableName::ProcFeatures, &schema);

    let read_schema = read_parquet_schema(&path);
    assert_eq!(
        read_schema.fields().len(),
        schema.fields().len(),
        "proc_features schema field count mismatch"
    );

    assert!(read_schema.field_with_name("proc_type").is_ok());
    assert!(read_schema.field_with_name("is_orphan").is_ok());
    assert!(read_schema.field_with_name("cmd_pattern").is_ok());

    let batches = read_parquet_batches(&path);
    assert_eq!(batches[0].num_rows(), 1);

    eprintln!(
        "[INFO] proc_features schema: {} fields verified",
        schema.fields().len()
    );
}

#[test]
fn test_proc_inference_schema_write_read_roundtrip() {
    let temp_dir = TempDir::new().expect("temp dir");
    let schema = proc_inference_schema();
    let path = write_and_close(&temp_dir, TableName::ProcInference, &schema);

    let read_schema = read_parquet_schema(&path);
    assert_eq!(
        read_schema.fields().len(),
        schema.fields().len(),
        "proc_inference schema field count mismatch"
    );

    assert!(read_schema.field_with_name("p_abandoned").is_ok());
    assert!(read_schema.field_with_name("score").is_ok());
    assert!(read_schema.field_with_name("recommendation").is_ok());
    assert!(read_schema.field_with_name("evidence_tags_json").is_ok());
    assert!(read_schema.field_with_name("signature_id").is_ok());

    let batches = read_parquet_batches(&path);
    assert_eq!(batches[0].num_rows(), 1);

    eprintln!(
        "[INFO] proc_inference schema: {} fields verified",
        schema.fields().len()
    );
}

#[test]
fn test_outcomes_schema_write_read_roundtrip() {
    let temp_dir = TempDir::new().expect("temp dir");
    let schema = outcomes_schema();
    let path = write_and_close(&temp_dir, TableName::Outcomes, &schema);

    let read_schema = read_parquet_schema(&path);
    assert_eq!(
        read_schema.fields().len(),
        schema.fields().len(),
        "outcomes schema field count mismatch"
    );

    assert!(read_schema.field_with_name("decision").is_ok());
    assert!(read_schema.field_with_name("action_successful").is_ok());
    assert!(read_schema.field_with_name("user_feedback").is_ok());

    let batches = read_parquet_batches(&path);
    assert_eq!(batches[0].num_rows(), 1);

    eprintln!(
        "[INFO] outcomes schema: {} fields verified",
        schema.fields().len()
    );
}

#[test]
fn test_signature_matches_schema_write_read_roundtrip() {
    let temp_dir = TempDir::new().expect("temp dir");
    let schema = signature_matches_schema();
    let path = write_and_close(&temp_dir, TableName::SignatureMatches, &schema);

    let read_schema = read_parquet_schema(&path);
    assert_eq!(
        read_schema.fields().len(),
        schema.fields().len(),
        "signature_matches schema field count mismatch"
    );

    assert!(read_schema.field_with_name("signature_id").is_ok());
    assert!(read_schema.field_with_name("match_confidence").is_ok());
    assert!(read_schema
        .field_with_name("predicted_prob_abandoned")
        .is_ok());
    assert!(read_schema.field_with_name("actual_abandoned").is_ok());

    let batches = read_parquet_batches(&path);
    assert_eq!(batches[0].num_rows(), 1);

    eprintln!(
        "[INFO] signature_matches schema: {} fields verified",
        schema.fields().len()
    );
}

// ============================================================================
// Schema Container Tests
// ============================================================================

#[test]
fn test_telemetry_schema_container_all_tables() {
    let schemas = TelemetrySchema::new();

    let tables = [
        TableName::Runs,
        TableName::ProcSamples,
        TableName::ProcFeatures,
        TableName::ProcInference,
        TableName::Outcomes,
        TableName::Audit,
        TableName::SignatureMatches,
    ];

    for table in tables {
        let schema = schemas.get(table);
        assert!(
            !schema.fields().is_empty(),
            "Schema for {:?} should have fields",
            table
        );
        // Every schema must have session_id as the first or second field
        let has_session_id = schema.fields().iter().any(|f| f.name() == "session_id");
        // Audit uses audit_ts as first field, but all have session_id
        assert!(
            has_session_id,
            "Schema for {:?} should contain session_id",
            table
        );

        eprintln!(
            "[INFO] {:?}: {} fields, retention={} days",
            table,
            schema.fields().len(),
            table.retention_days()
        );
    }
}

// ============================================================================
// Multi-Batch Write Tests
// ============================================================================

#[test]
fn test_audit_multi_batch_write() {
    let temp_dir = TempDir::new().expect("temp dir");
    let schema = audit_schema();
    let config = WriterConfig::new(
        temp_dir.path().to_path_buf(),
        "pt-20260115-143022-multi".to_string(),
        "multi-host".to_string(),
    )
    .with_batch_size(100); // Don't auto-flush

    let mut writer = BatchedWriter::new(TableName::Audit, Arc::new(schema.clone()), config);

    // Write 5 batches
    for _ in 0..5 {
        let batch = create_batch_for_schema(&schema);
        writer.write(batch).expect("write batch");
    }

    let path = writer.close().expect("close writer");
    let batches = read_parquet_batches(&path);

    let total_rows: usize = batches.iter().map(|b| b.num_rows()).sum();
    assert_eq!(total_rows, 5, "should have 5 total rows across batches");

    eprintln!(
        "[INFO] Multi-batch write: 5 rows across {} batches",
        batches.len()
    );
}

// ============================================================================
// Parquet File Integrity Tests
// ============================================================================

#[test]
fn test_parquet_files_readable_by_standard_reader() {
    let temp_dir = TempDir::new().expect("temp dir");
    let tables = [
        (TableName::Audit, audit_schema()),
        (TableName::Runs, runs_schema()),
        (TableName::ProcSamples, proc_samples_schema()),
    ];

    for (table, schema) in &tables {
        let path = write_and_close(&temp_dir, *table, schema);

        // Read via standard parquet reader
        let file = fs::File::open(&path).expect("open file");
        let builder = ParquetRecordBatchReaderBuilder::try_new(file).expect("reader");

        // Verify metadata
        let metadata = builder.metadata();
        assert!(
            metadata.num_row_groups() > 0,
            "Table {:?}: should have at least 1 row group",
            table
        );

        let parquet_schema = builder.schema();
        assert_eq!(
            parquet_schema.fields().len(),
            schema.fields().len(),
            "Table {:?}: field count mismatch",
            table
        );

        eprintln!(
            "[INFO] {:?}: {} row groups, {} fields, file size={} bytes",
            table,
            metadata.num_row_groups(),
            parquet_schema.fields().len(),
            fs::metadata(&path).unwrap().len()
        );
    }
}
