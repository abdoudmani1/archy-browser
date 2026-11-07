[Setup]
AppName=Archy Browser
AppVersion=1.0
DefaultDirName={autopf}\Archy Browser
DefaultGroupName=Archy Browser
OutputDir=.
OutputBaseFilename=Archy-Installer
Compression=lzma2
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64

[Files]
Source: "target\release\archy-browser.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\Archy Browser"; Filename: "{app}\archy-browser.exe"
Name: "{autodesktop}\Archy Browser"; Filename: "{app}\archy-browser.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional icons:"
