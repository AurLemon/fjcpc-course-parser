# PowerShell script to run tests

Write-Host "🧪 Running FJCPC Course Parser Tests" -ForegroundColor Cyan
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

# Run tests
Write-Host "🔨 Building tests..." -ForegroundColor Green
cargo build --tests

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ Build successful!" -ForegroundColor Green
    Write-Host ""
    Write-Host "🧪 Running tests..." -ForegroundColor Cyan
    Write-Host ""
    cargo test -- --nocapture
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✅ All tests passed!" -ForegroundColor Green
    } else {
        Write-Host ""
        Write-Host "❌ Some tests failed!" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

