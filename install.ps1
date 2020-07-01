if(Test-Path ".\Makefile.toml") {
    $OPENCLOUD_DIR = Get-Location
} else { 
    git clone https://github.com/Rheydskey/OpenCloud.git opencloud
    Set-Location opencloud
    $OPENCLOUD_DIR = Get-Location
}
Set-Location $OPENCLOUD_DIR
#Test if path contains cargo binaries 
$paths = $env:Path.Split(";")#split path to array
if (!($paths -contains "${env:USERPROFILE}\.cargo\bin")) {
    #Automatic install deps
    Write-Host -ForegroundColor Green "Building Opencloud deps..."
    Write-Host -ForegroundColor Magenta "Installing chocolatey..."
    write-host ""
    Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1')) >  $PSScriptRoot\install.log
    refreshenv | Out-null #Use chocolatey refreshenv script
    write-host ""
    Write-Host -ForegroundColor Magenta "Installing visual studio 2017 C++ Build Tools"
    write-host "" #Use upgrade instead install to prevent errors
    choco upgrade visualstudio2017buildtools -y 
    write-host ""
    write-host -foregroundcolor Magenta "Installing rustup" #Downloading latest rustup version
    if (Test-path $PSScriptRoot\rustup-init.exe) { Remove-Item $PSScriptRoot\rustup-init.exe }
    Invoke-WebRequest -OutFile "${PSScriptRoot}\rustup-init.exe" "https://win.rustup.rs/x86_64"
    & $PSScriptRoot\rustup-init.exe -y >> install.log
    RefreshEnv | Out-Null
} 
cargo install cargo-make
cargo make build_release