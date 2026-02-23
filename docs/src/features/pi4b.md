# Raspberry Pi 4B Tuning

When MASH detects a Raspberry Pi 4B, it applies targeted optimisations for HDD and SSD
storage attached via USB 3.0.

## Auto-Detection

MASH reads `/proc/device-tree/model` to detect Pi hardware. No flags required.

```bash
mash-setup status
# Pi:   Raspberry Pi 4 Model B Rev 1.5
```

## Applied Optimisations

### Mount Options

Adds `noatime,commit=60` to `/etc/fstab` entries for the root and data partitions:
- `noatime` — eliminates access-time writes on every file read (reduces SD/HDD wear)
- `commit=60` — flushes the journal every 60s instead of every 5s (reduces write frequency)

### Swap Configuration

Moves the swapfile to an external HDD if one is detected, keeping the SD card free from
write-intensive swap I/O.

### Kernel Parameters (`/etc/sysctl.conf`)

| Parameter                  | Value | Reason                             |
|----------------------------|-------|------------------------------------|
| `vm.swappiness`            | 10    | Prefer RAM; swap only under pressure|
| `vm.dirty_ratio`           | 15    | Write cache ceiling                |
| `vm.dirty_background_ratio`| 5     | Background writeback starts early  |

### I/O Scheduler

Sets `mq-deadline` for USB-attached block devices — better latency for external SSDs/HDDs
than the default `none` or `bfq`.

### USB 3.0 Detection

MASH identifies USB 3.0 controllers and reports their capabilities in the doctor output.

## Preflight Checks

Before applying changes, MASH runs Pi-specific preflight:

- **Disk health**: reads SMART data via `hdparm`
- **Scheduler**: reports current I/O scheduler
- **Partition layout**: inspects mount points and filesystem types

## Dry-Run

```bash
mash-setup --dry-run
```

All Pi tuning changes are logged without being applied. Review with:

```bash
cat ~/mash-install.log
```

## After Tuning

A reboot is recommended to activate kernel parameter changes:

```bash
sudo reboot
```
