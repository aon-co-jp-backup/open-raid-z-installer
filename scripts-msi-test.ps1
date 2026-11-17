# MSI install/uninstall test (run elevated). Log: F:\adb-target\msi-test.log
$log='F:\adb-target\msi-test.log'; $msi='F:\adb-target\oritauri\release\bundle\msi\OpenRaidZ Installer_0.1.0_x64_en-US.msi'
"start $(Get-Date)" | Out-File $log
$p=Start-Process msiexec -ArgumentList "/i `"$msi`" /qn /l*v F:\adb-target\msi-install.log" -Wait -PassThru
"install exit=$($p.ExitCode)" | Out-File $log -Append
$exe=Get-ChildItem 'C:\Program Files' -Directory | Where-Object {$_.Name -match 'OpenRaidZ|Open Raid'} | ForEach-Object {Get-ChildItem $_.FullName -Filter *.exe -Recurse} | Where-Object {$_.Name -notmatch 'unins'} | Select -First 1
"exe=$($exe.FullName)" | Out-File $log -Append
if($exe){$a=Start-Process $exe.FullName -PassThru; Start-Sleep 8; "alive=$(-not $a.HasExited)" | Out-File $log -Append; Stop-Process $a -Force -ErrorAction SilentlyContinue}
$p=Start-Process msiexec -ArgumentList "/x `"$msi`" /qn /l*v F:\adb-target\msi-uninstall.log" -Wait -PassThru
"uninstall exit=$($p.ExitCode)" | Out-File $log -Append
"exe_remaining=$(if($exe){Test-Path $exe.FullName})" | Out-File $log -Append
"done" | Out-File $log -Append
