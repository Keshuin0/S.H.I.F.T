:: watchexec --debounce 15s -- .\sync.bat
@echo off
cd /d "D:\Project\Project S.H.I.F.T"
echo [!] Saving changes to GitHub...
git add .
git commit -m "Auto-sync update"
git push origin main
echo [X] Sync Complete!