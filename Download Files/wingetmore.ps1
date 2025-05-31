param
(
    [string[]]$uninstall,
    [string[]]$upgrade,
    [string[]]$install,
    [string[]]$search
)

if($null -ne $uninstall)
{
    foreach($item in $uninstall)
    {
        Write-Host "`nUninstalling $item`n" -BackgroundColor DarkRed -ForegroundColor White
        winget.exe uninstall $item
    }
}

if($null -ne $upgrade)
{
    if($upgrade[0] -eq "--all")
    {
        Write-Host "`nUpdating everything 😊`n" -BackgroundColor DarkBlue -ForegroundColor White
        winget.exe upgrade --all
    }
    else
    {
        foreach($item in $upgrade)
        {
            Write-Host "`nUpdating $item`n" -BackgroundColor DarkBlue -ForegroundColor White
            winget.exe upgrade $item
        }
    }
}

if($null -ne $install)
{
    foreach($item in $install)
    {
        Write-Host "`nInstalling $item`n" -BackgroundColor DarkMagenta -ForegroundColor White
        winget.exe install $item
    }
}

if($null -ne $search)
{
    foreach($item in $search)
    {
        Write-Host "`nSearching for $item`n" -BackgroundColor DarkGreen -ForegroundColor White
        winget.exe search $item
    }
}