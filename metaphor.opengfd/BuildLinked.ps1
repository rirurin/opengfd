# Set Working Directory
Split-Path $MyInvocation.MyCommand.Path | Push-Location
[Environment]::CurrentDirectory = $PWD

Remove-Item "$env:RELOADEDIIMODS/metaphor.opengfd/*" -Force -Recurse
dotnet publish "./metaphor.opengfd.csproj" -c Release -o "$env:RELOADEDIIMODS/metaphor.opengfd" /p:OutputPath="./bin/Release" /p:ReloadedILLink="true"

# Restore Working Directory
Pop-Location