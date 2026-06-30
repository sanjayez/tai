$ErrorActionPreference = "Stop"

Write-Host "Packaging is not wired yet."
Write-Host "Target flow: build desktop app, verify signed manifests, optionally include sidecars/models/OCR, produce Windows installer."
Write-Host "Large local models must be bundled through an explicit packaging mode, not routine Tauri builds."
