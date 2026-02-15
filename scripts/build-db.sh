#!/bin/bash
# Bash script to build database from schema files

OUTPUT_DB="${1:-temp.db}"
COMBINED_SCHEMA="${2:-./schemas/schema_combined.sql}"

# Get all .sql files in schemas directory, excluding specific files
SCHEMA_FILES=($(find ./schemas -name "*.sql" -not -name "schema.sql" -not -name "schema_combined.sql" | sort))

if [ ${#SCHEMA_FILES[@]} -eq 0 ]; then
    echo "Error: No schema files found in ./schemas directory" >&2
    exit 1
fi

echo "Found ${#SCHEMA_FILES[@]} schema files:"
for file in "${SCHEMA_FILES[@]}"; do
    echo "  - $(basename "$file")"
done

# Remove existing files if they exist
if [ -f "$OUTPUT_DB" ]; then
    rm -f "$OUTPUT_DB"
    echo "Removed existing $OUTPUT_DB"
fi

if [ -f "$COMBINED_SCHEMA" ]; then
    rm -f "$COMBINED_SCHEMA"
    echo "Removed existing $(basename "$COMBINED_SCHEMA")"
fi

# First, combine all schema files into schema_combined.sql
echo "Combining schemas into: $COMBINED_SCHEMA"
cat "${SCHEMA_FILES[@]}" > "$COMBINED_SCHEMA"

if [ ! -f "$COMBINED_SCHEMA" ]; then
    echo "Error: Failed to create combined schema file" >&2
    exit 1
fi

# Then use the combined schema to build the database
echo "Building database: $OUTPUT_DB"
sqlite3 "$OUTPUT_DB" < "$COMBINED_SCHEMA"

if [ $? -eq 0 ]; then
    echo "Database built successfully: $OUTPUT_DB"
    echo "Combined schema available at: $COMBINED_SCHEMA"
else
    echo "Error: Failed to build database" >&2
    exit 1
fi