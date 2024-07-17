# Hellow Cargo

## Creating

```powershell
PS C:\> cargo.exe new hello-cargo
PS C:\> cd hello-cargo
```

## Build & Run

```powershell
PS C:\> cargo.exe run
```

## Notes

1. Cargo.lock should be added to source control for an executable, for a library it should not be added to source control.
2. Use ms-vscode.cpptools to debug on Windows and vadimcn.vscode-lldb to debug on Linux/Mac.

## References

```
https://learn.microsoft.com/en-us/training/paths/rust-first-steps/
https://code.visualstudio.com/docs/languages/rust
```