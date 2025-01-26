param (
    [System.Boolean] $IsDebug = $False,
    [System.Boolean] $ShowPrintf = $False
)

[System.Boolean] $global:is_pushed = $False

function GoToRelativeFolder {
    param (
        [string] $ChildPath
    )
    if ($global:is_pushed) { Pop-Location }
    [IO.Path]::Combine((Get-Location).ToString(), $ChildPath) | Push-Location
    $global:is_pushed = $True
}

function GoToFolder {
    param (
        [string] $Path
    )
    if ($global:is_pushed) { Pop-Location }
    $Path | Push-Location
    $global:is_pushed = $True
}

function GetNonNullEnvironmentVariable {
    param (
        [string] $EnvVariable
    )
    $TryEnvValue = [System.Environment]::GetEnvironmentVariable($EnvVariable)
    if ([System.String]::IsNullOrEmpty($TryEnvValue)) {
        Write-Error "No value was provided for the environmental variable ${EnvVariable}. 
This should be set before executing BuildReloadedMod"
    } else {
        $TryEnvValue
    }
}

function SetEnvironmentVariableIfNull {
    param (
        [string] $EnvVariable,
        [string] $EnvValue
    )
    $TryEnvValue = [System.Environment]::GetEnvironmentVariable($EnvVariable)
    if ([System.String]::IsNullOrEmpty($TryEnvValue)) {
        [System.Environment]::SetEnvironmentVariable($EnvVariable, $EnvValue)
    }
}

function GetNameWithUnderscores {
    param (
        [string] $Name
    )
    $Name.Replace("-", "_")
}

[string] $global:OPENGFD_LIB = "opengfd"
# We don't define a specific target here, ideally components that are decompiled enough to be unit tested should be platform-independent.

function TestRustCrate {
    param (
        [string] $FriendlyName,
        [string] $BuildStd,
        [string] $BuildStdFeatures,
        [string] $CrateType
    )
    $Profile = if ($IsDebug) { "--profile=release-debug" } else { "--profile=release" }
    $NoCapture = if ($ShowPrintf) { "--nocapture" } else { "" }
    cargo +nightly test $Profile -- $NoCapture # -Z build-std=$BuildStd -Z build-std-features=$BuildStdFeatures
    # if (!$?) {
    #     Write-Error "Tests failed for ${FriendlyName}"
    # }
}

# Set working directory
Split-Path $MyInvocation.MyCommand.Path | Push-Location
[Environment]::CurrentDirectory = $PWD
$BASE_PATH = (Get-Location).ToString();
[System.Environment]::SetEnvironmentVariable("RUST_BACKTRACE", 1)
[System.Environment]::SetEnvironmentVariable("RUSTFLAGS", "-C panic=abort -Z panic_abort_tests")
GoToFolder -Path ([IO.Path]::Combine($BASE_PATH, $global:OPENGFD_LIB))
TestRustCrate -FriendlyName $global:OPENGFD_LIB -BuildStd "std,panic_abort" -BuildStdFeatures "panic_immediate_abort"
# Restore Working Directory
if ($global:is_pushed) {
    Pop-Location
}
