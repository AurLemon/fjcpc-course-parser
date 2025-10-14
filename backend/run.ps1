# PowerShell script to run the Rust backend

Write-Host "🦀 Starting FJCPC Course Parser - Rust Backend" -ForegroundColor Cyan
Write-Host ""

# Check if .env exists
if (-not (Test-Path ".env")) {
    Write-Host "⚠️  .env file not found!" -ForegroundColor Yellow
    Write-Host "📝 Creating .env from .env.example..." -ForegroundColor Yellow
    Copy-Item ".env.example" ".env"
    Write-Host "✅ Please edit .env file and set TEST_STUDENT_UCODE" -ForegroundColor Green
    Write-Host ""
    exit 1
}

# Build and run
Write-Host "🔨 Building in release mode..." -ForegroundColor Green
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Build successful!" -ForegroundColor Green
    Write-Host ""
    Write-Host "🚀 Starting server..." -ForegroundColor Cyan
    Write-Host "📍 Server will be available at http://127.0.0.1:8080" -ForegroundColor Cyan
    Write-Host "🛑 Press Ctrl+C to stop" -ForegroundColor Yellow
    Write-Host ""
    cargo run --release
} else {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

