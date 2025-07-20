param(
  [string]$PACK,
  [string]$TAG,
  [string]$LABEL,
  [string]$TARGET
)

$OUT = "$PACK-$TAG-$LABEL.zip"
New-Item -ItemType Directory -Path pkg\temp\bin -Force | Out-Null

$BIN_NAMES = (
  cargo metadata --format-version 1 --no-deps |
  ConvertFrom-Json
).packages |
  ForEach-Object {
      $_.targets |
      Where-Object { $_.kind -contains 'bin' } |
      ForEach-Object { $_.name }
  } | Sort-Object -Unique

foreach ($BIN in $BIN_NAMES) {
    $SRC = "target\$TARGET\release\$BIN.exe"
    Copy-Item $SRC -Destination pkg\temp\bin
}

Copy-Item README.md -Destination pkg\temp -ErrorAction SilentlyContinue
Copy-Item LICENSE   -Destination pkg\temp -ErrorAction SilentlyContinue

Compress-Archive -Path pkg\temp\* -DestinationPath "pkg\$OUT" -Force
Remove-Item -Path pkg\temp -Recurse -Force -ErrorAction SilentlyContinue
