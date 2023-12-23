@echo off
sea-orm-cli generate entity -u sqlite://db.sqlite3?mode=rwc -o entity/src
pause