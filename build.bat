@echo off
setlocal enabledelayedexpansion
set CACHE_DIR=duckdb-cache
set PROJECT_DIR=%~dp0
set MODE=full
if /i "%1"=="/lite" set MODE=lite
if /i "%1"=="/full" set MODE=full
set BUILD_ARGS=
:parse_args
if "%1"=="" goto :done_parse
set FIRST=%1
if /i "!FIRST!"=="/lite" goto :skip_arg
if /i "!FIRST!"=="/full" goto :skip_arg
set BUILD_ARGS=!BUILD_ARGS! %1
:skip_arg
shift
goto :parse_args
:done_parse
if defined BUILD_ARGS set BUILD_ARGS=!BUILD_ARGS:~1!
set "CACHE_LIB=%PROJECT_DIR%!CACHE_DIR!\duckdb_static.lib"
if "%MODE%"=="lite" goto :lite_build
if not exist "%CACHE_LIB%" (
    echo [error] missing DuckDB library: %CACHE_DIR%\duckdb_static.lib
    echo [error] put duckdb_static.lib + duckdb.h into %CACHE_DIR%\
    echo [error] or set DUCKDB_LIB_DIR to a directory that has them)
    exit /b 1
)
echo [cache] using %CACHE_DIR%\duckdb_static.lib
set "DUCKDB_LIB_DIR=%PROJECT_DIR%%CACHE_DIR%"
set "DUCKDB_INCLUDE_DIR=%PROJECT_DIR%%CACHE_DIR%"
set "DUCKDB_STATIC=1"
cd /d "%PROJECT_DIR%"
cargo build --release --features http --bins !BUILD_ARGS!
exit /b !errorlevel!
:lite_build
if not exist "%CACHE_LIB%" (
    echo [error] missing DuckDB library: %CACHE_DIR%\duckdb_static.lib
    echo [error] run build.bat first
    exit /b 1
)
echo [cache] using %CACHE_DIR%\duckdb_static.lib
set "DUCKDB_LIB_DIR=%PROJECT_DIR%%CACHE_DIR%"
set "DUCKDB_INCLUDE_DIR=%PROJECT_DIR%%CACHE_DIR%"
set "DUCKDB_STATIC=1"
cd /d "%PROJECT_DIR%"
cargo build --release --no-default-features --features duckdb-cached,http --bins !BUILD_ARGS!
exit /b !errorlevel!
