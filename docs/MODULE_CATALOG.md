# Module Catalog

This catalog describes the main source areas that may exist in the private SharpMTKClient package. Exact file names, package scope and implementation depth depend on the licensed delivery.

## Device and Transport Modules

* MediaTek USB port detection.
* USB plug / unplug event handling.
* Serial / USB communication wrappers.
* Native Windows API communication paths.
* Connection timeout, reconnect and mode transition handling.
* Device info and status logging.

## Protocol Modules

* BROM handshake and command helpers.
* Preloader communication helpers.
* LEGACY DA protocol paths.
* XFLASH protocol paths.
* XML / MTK V6 command paths.
* Command serialization and response parsing.
* Error mapping and operation result handling.

## Download Agent Modules

* DA parser.
* DA metadata handling.
* Stage-1 and Stage-2 upload flow.
* DA extension loading paths.
* EMI selection, receive and send workflow.
* Flash type, page size, storage size and read size setup.

## Flash and Firmware Modules

* Scatter firmware workflow.
* XML firmware workflow.
* Flash by partition name.
* Raw address read and write.
* Raw image and sparse image handling.
* Preloader read and write.
* Firmware upgrade and download-only style flows where supported.
* Progress and elapsed-time reporting.

## Partition and Storage Modules

* GPT parser.
* Partition table reader.
* Partition lookup by name.
* Sector and size calculation helpers.
* Read, write and erase selected partitions.
* Full flash backup paths where supported.
* GPT flash and rebuild paths.
* EMMC, UFS, NAND, NOR and SDMMC target helpers where available.

## File Management Modules

* Android partition detection.
* EXT4 read and write-related paths.
* Logical partition and `super` partition metadata handling.
* EROFS read paths.
* F2FS read paths.
* `build.prop` parser and Android information extraction.
* File and directory browser integration.
* File extraction, replacement, deletion and add workflows where supported.

## NV, IMEI and Key Modules

* NV partition backup.
* NV write and erase paths.
* NV item parsing and decode logic.
* NV item decryption paths where included.
* LD0B / nvdata related IMEI development paths.
* IMEI information read in supported operations.
* IMEI read and repair workflow areas for selected modes and brands.
* FlashMode IMEI repair paths for supported brands and chipsets.
* Hardware key read paths.
* DXCC, SEJ, GCPU and AES helper integration points where supported.

## RPMB Modules

* RPMB read paths.
* RPMB write paths.
* RPMB erase paths.
* RPMB key read paths where target mode supports it.
* Protocol-specific RPMB command integration.

## Security and Research Modules

* SLA / DAA handling paths.
* Kamakiri workflow integration.
* HeapBait workflow integration.
* Carbonara workflow integration.
* Bootloader unlock and relock paths.
* FRP and format workflows.
* Vbmeta / dm-verity patch workflows.
* Brand and model-specific examples where included.

## Patch and Advanced Service Modules

* Patch certificate workflow modules for selected OPPO and Xiaomi paths.
* Mi Account disable, remove and global conversion workflow areas.
* OPPO ID reset workflow areas.
* Huawei ID reset workflow areas.
* Payjoy remove and Payjoy unlock workflow areas.
* OTA remove workflow areas for OPPO, Realme, OnePlus and Xiaomi targets where supported.
* MDM remove, MDM + OTA remove and IT Admin / Network workflow areas.
* Walton MDM / Walock remove workflow areas.
* Nothing / CMF SIM Lock / MDM remove workflow areas.
* Xiaomi anti-relock modem patch workflow areas.
* Persist patch and demo remove workflow areas.
* Restore Vbmeta and Vbmeta Patch V1 / V2 / V3 workflow areas.
* Fix no efuse state, fix DM corrupted and remove security plugin workflow areas.

## UI and Product Modules

* Main operation window.
* Logs and operation history.
* Device and vendor selection configuration.
* File flash selection dialogs.
* File Management views.
* Progress reporting and user prompts.
* Branding, theme and license integration points.
