## Leptos install issue in windows

### Problem 1: Git Bash and Perl conflict

If you installed Git Bash terminal on Windows. The issue is that Git Bash ships with its own `perl.exe`, which interferes with the build process for `cargo-leptos`.

**Fix:**

* Install Strawberry Perl separately
* Add its `/bin` directory to your system PATH
* Make sure Strawberry Perl comes **before** Git Bash in PATH priority
* Use PowerShell (or another terminal) instead of Git Bash for installation so the perl you installed seperately takes priority

**Note:** The installation is time heavy by default. leptos installation can take upto 40 minutes 


### Problem 2: OpenSSL setup on Windows

The Rust API guide explains OpenSSL setup for Linux and macOS, but skips Windows entirely.

**Fix:**

* Download and install OpenSSL from [Shining Light Productions (MSI installer)](https://slproweb.com/products/Win32OpenSSL.html)
* During setup, select: “The OpenSSL binaries (/bin) directory”
* Add this to PATH:
  `C:\OpenSSL-Win64\bin`
* Create an environment variable:
  `OPENSSL_DIR = C:\OpenSSL-Win64`
