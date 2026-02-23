# Pi SD Card Sync

On Raspberry Pi, SD cards are vulnerable to corruption on sudden power loss. MASH includes
`sync_file()` and `verify_file_written()` utilities to harden file writes.

## The Problem

SD cards use flash storage with write caches. An unexpected power cut before the cache flushes
can corrupt files — config files, scripts, and package lists are all at risk.

## MASH's Approach

All critical file writes in MASH go through `verify::sync_file()`:

1. Write the file content
2. Call `fsync()` to flush to storage
3. Optionally call `fsync()` on the parent directory (ensures directory entry is committed)
4. Verify the written content matches what was intended

## Enabling SD Card Safety

This is automatic on all platforms. No special flags are needed — MASH always syncs critical
files after writing them.

## Manual Sync After MASH

After a large install, it's good practice to sync before rebooting:

```bash
sync
sudo reboot
```

## UPS / Graceful Shutdown

For a Pi running as a server, consider:
- A UPS (uninterruptible power supply) with graceful shutdown signaling
- The `safe-reboot` pattern: `sudo systemctl start pigpiod && sync && sudo reboot`

## Checking SD Card Health

```bash
mash-setup doctor
```

Doctor mode checks the health of attached storage using `hdparm` and reports any SMART errors
or filesystem issues.

## Backup Strategy

MASH's ABB law (Always Be Backing up) applies to your Pi too:

```bash
# Backup SD card image
sudo dd if=/dev/mmcblk0 bs=4M | gzip > ~/mash-pi-backup.img.gz

# Or use rclone (installed by MASH) to sync to cloud/NAS
rclone sync /home/user remote:backup
```
