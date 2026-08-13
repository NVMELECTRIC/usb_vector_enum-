# Architecture

SharpMTKClient is organized as a Windows C# source-code project for MTK protocol development. The licensed package is designed around separated workflow areas so developers can study, customize and extend the project for their own service software.

## High-Level Layers

| Layer | Purpose |
| --- | --- |
| User Interface | Windows desktop screens for connection, logging, flash, partition, file management, configuration and operation flow. |
| Workflow Layer | Coordinates selected operations such as read info, flash, backup, erase, format, RPMB, NV or file extraction. |
| Protocol Layer | Handles LEGACY, XFLASH and XML command families where supported. |
| Device Transport | Manages Windows USB / serial communication paths, port detection and reconnect behavior. |
| Download Agent Layer | Parses, selects, uploads and configures DA stages where supported. |
| Storage Layer | Provides partition, region, sector, size and storage helpers for EMMC, UFS, NAND and selected targets. |
| File-System Layer | Handles Android partition and image access paths such as EXT4, EROFS, F2FS and logical partitions. |
| Security / Crypto Helpers | Contains selected device crypto, key, authorization and research-oriented helper paths depending on package scope. |

## Protocol Organization

SharpMTKClient separates protocol behavior by workflow family:

* **LEGACY**: classic DA-based MTK flows for supported older devices.
* **XFLASH**: modern DA flows with native flash and partition operations where available.
* **XML**: MTK V6 / XML command paths used by supported firmware packages and device modes.

Each protocol path can have different command formats, reconnect behavior, storage handling, security requirements and supported operations.

## Typical Operation Flow

1. Detect a supported MediaTek port.
2. Establish BROM or Preloader communication.
3. Read hardware and security information.
4. Handle watchdog or boot preparation where supported.
5. Select DA, payload, EMI or XML workflow according to target requirements.
6. Upload or initialize the required stage.
7. Synchronize protocol state after mode changes.
8. Query storage and partition information.
9. Execute the selected operation.
10. Report progress, logs, status and errors to the UI.

## Extension Points

Licensed buyers commonly extend:

* Brand and model selection lists.
* DA selection rules.
* Firmware package parsing.
* Partition operation presets.
* NV and IMEI workflow screens.
* File-system browsing and extraction logic.
* Logging, reporting and customer-facing UI.
* Licensing, activation and branding layers.

## Design Goals

* Keep protocol code understandable for C# developers.
* Preserve direct control over Windows desktop workflow behavior.
* Allow private branding and commercial customization.
* Keep device-specific logic separated where practical.
* Support both service-software workflows and research-oriented protocol study.
