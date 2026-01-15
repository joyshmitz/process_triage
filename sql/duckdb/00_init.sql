-- Process Triage DuckDB Views and Macros
-- Version: 1.0.0
--
-- This SQL bundle provides standard views and macros for querying the
-- process triage telemetry lake. Load it into DuckDB with:
--
--   duckdb -c ".read sql/duckdb/00_init.sql"
--
-- Or from within DuckDB:
--   .read sql/duckdb/00_init.sql
--
-- Prerequisites:
--   - Parquet files in the telemetry lake directory
--   - SET pt_data_dir = '/path/to/telemetry';  (before loading)

-- Create a schema for process triage objects
CREATE SCHEMA IF NOT EXISTS pt;

-- Configuration variables (set these before loading data)
-- Example: SET pt_data_dir = '/home/user/.local/share/process_triage/telemetry';

-- Helper macro to construct table paths
CREATE OR REPLACE MACRO pt.table_path(table_name) AS
    CASE
        WHEN current_setting('pt_data_dir', 'default') != 'default'
        THEN current_setting('pt_data_dir') || '/' || table_name || '/**/*.parquet'
        ELSE table_name || '/**/*.parquet'
    END;

-- Register Parquet tables as views (lazy loading)
-- These views read from the configured pt_data_dir

CREATE OR REPLACE VIEW pt.raw_runs AS
SELECT * FROM read_parquet(pt.table_path('runs'), union_by_name=true, filename=true);

CREATE OR REPLACE VIEW pt.raw_proc_samples AS
SELECT * FROM read_parquet(pt.table_path('proc_samples'), union_by_name=true, filename=true);

CREATE OR REPLACE VIEW pt.raw_proc_features AS
SELECT * FROM read_parquet(pt.table_path('proc_features'), union_by_name=true, filename=true);

CREATE OR REPLACE VIEW pt.raw_proc_inference AS
SELECT * FROM read_parquet(pt.table_path('proc_inference'), union_by_name=true, filename=true);

CREATE OR REPLACE VIEW pt.raw_outcomes AS
SELECT * FROM read_parquet(pt.table_path('outcomes'), union_by_name=true, filename=true);

CREATE OR REPLACE VIEW pt.raw_audit AS
SELECT * FROM read_parquet(pt.table_path('audit'), union_by_name=true, filename=true);
