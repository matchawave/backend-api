# PowerShell script to build database from schema files
param(
    [string]$OutputDb = "temp.db",
    [string]$CombinedSchema = "./schemas/schema_combined.sql"
)

# Get all .sql files in schemas directory, excluding specific files
$schemaFiles = Get-ChildItem -Path "./schemas" -Filter "*.sql" | 
    Where-Object { $_.Name -ne "schema_combined.sql" } |
    Sort-Object Name |
    ForEach-Object { $_.FullName }

if ($schemaFiles.Count -eq 0) {
    Write-Error "No schema files found in ./schemas directory"
    exit 1
}

Write-Host "Found $($schemaFiles.Count) schema files:"
$schemaFiles | ForEach-Object { Write-Host "  - $(Split-Path $_ -Leaf)" }

# Remove existing files if they exist
if (Test-Path $OutputDb) {
    Remove-Item $OutputDb -Force
    Write-Host "Removed existing $OutputDb"
}

if (Test-Path $CombinedSchema) {
    Remove-Item $CombinedSchema -Force
    Write-Host "Removed existing $(Split-Path $CombinedSchema -Leaf)"
}

# First, combine all schema files into schema_combined.sql
Write-Host "Combining schemas into: $CombinedSchema"
Get-Content $schemaFiles | Out-File -FilePath $CombinedSchema -Encoding UTF8

if (-not (Test-Path $CombinedSchema)) {
    Write-Error "Failed to create combined schema file"
    exit 1
}

# Then use the combined schema to build the database
Write-Host "Building database: $OutputDb"
Get-Content $CombinedSchema | sqlite3 $OutputDb

if ($LASTEXITCODE -eq 0) {
    Write-Host "Database built successfully: $OutputDb" -ForegroundColor Green
    Write-Host "Combined schema available at: $CombinedSchema" -ForegroundColor Green
} else {
    Write-Error "Failed to build database"
    exit 1
}