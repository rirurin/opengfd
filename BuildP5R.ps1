param (
    [System.Boolean] $Debug = $False,
    [System.Boolean] $SkipGlobals = $False,
    [System.Boolean] $Timings = $False,
    [System.Boolean] $Publish = $False
)

$ProjectName = "opengfd.BuildP5R"
dotnet run --project "buildscript/$ProjectName/$ProjectName.csproj" -- (Get-Location).ToString() "Debug" $Debug "SkipGlobals" $SkipGlobals "Timings" $Timings "Publish" $Publish