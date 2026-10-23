a = Analysis(
    ["local_server.py"],
    pathex=["."],
    binaries=[],
    datas=[],
    hiddenimports=["uvicorn.logging", "uvicorn.loops.auto", "uvicorn.protocols.http.auto"],
    hookspath=[],
    runtime_hooks=[],
    excludes=["celery", "redis", "IPython", "matplotlib", "numpy", "pandas", "scipy", "pytest"],
    noarchive=False,
)
pyz = PYZ(a.pure)
exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.datas,
    [],
    name="cuepoint-backend",
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    console=False,
)
