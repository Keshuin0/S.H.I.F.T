@echo off
echo [!] Saving changes to GitHub dev branch...
git add .
git commit -m "Auto-sync update"
git push origin dev
echo [X] Sync Complete!