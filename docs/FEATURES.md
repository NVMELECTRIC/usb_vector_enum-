# Features

This document is the canonical SharpMTKClient feature overview. It combines the main feature list, feature matrix and advanced service workflow areas so the repository has one primary source for product capabilities.

Feature availability depends on chipset, selected DA, boot mode, firmware package, security state, licensed package scope and operation requirements. Support for any specific model must be confirmed before purchase.

## Protocol Families

| Area | Description |
| --- | --- |
| LEGACY | Classic DA-based MediaTek protocol flow for supported legacy devices. |
| XFLASH | Modern DA workflow with native partition operations and faster flash paths where supported. |
| XML | MTK V6 / XML command workflow for supported firmware packages and device modes. |

## Device and Boot Flow

| Feature | Status |
| --- | --- |
| VID / PID MediaTek port detection | Included where supported |
| USB plug / unplug event handling | Included where supported |
| BROM service detection and handshake | Included where supported |
| Preloader communication | Included where supported |
| Hardware code, sub-code and version parsing | Included where supported |
| Security flag parsing | Included where supported |
| MEID / SOCID reading | Device dependent |
| Watchdog handling | Device dependent |
| DA selection | Included where supported |
| Stage-1 / Stage-2 upload | Included where supported |
| EMI receive / send workflow | Device dependent |
| Connection status and operation log output | Included where supported |

## Flash and Partition Operations

| Feature | Status |
| --- | --- |
| Scatter firmware workflow | Included where supported |
| XML firmware workflow | Included where supported |
| Native XFLASH / XML flash | Included where supported |
| Download, firmware upgrade and download-only XML paths | Included where supported |
| Raw partition read / write | Included where supported |
| Raw firmware flash | Included where supported |
| Sparse image handling | Included where supported |
| GPT read, flash and rebuild | Included where supported |
| Partition dump / backup | Included where supported |
| Partition erase | Included where supported |
| Full flash backup | Device dependent |
| Preloader read / write | Included where supported |
| Safe format | Device dependent |
| Manual format | Device dependent |
| FRP erase | Device dependent |
| Bootloader unlock / relock | Device dependent |
| Permanent unlock development paths | Device dependent |
| Read / orange state fix | Device dependent |

## File-System and Android Data

| Feature | Status |
| --- | --- |
| EXT4 partition detection | Included where supported |
| EXT4 browse and extraction | Included where supported |
| Native C# EXT4 stream reader / writer paths | Included where supported |
| EXT4 file replace / delete / add | Package dependent |
| Logical partition / super parsing | Included where supported |
| Dynamic partition browsing | Included where supported |
| EROFS detection and read paths | Included where supported |
| F2FS detection and read paths | Included where supported |
| Android `build.prop` extraction | Included where supported |
| File and directory extraction | Included where supported |
| Raw image source handling | Included where supported |

## NV, IMEI, RPMB and Keys

| Feature | Status |
| --- | --- |
| NV backup | Included where supported |
| NV write | Device dependent |
| NV erase | Device dependent |
| NV item decode logic | Package dependent |
| NV item decryption | Package dependent |
| Read IMEI info from `nvdata` / `ld0b` | Device dependent |
| IMEI read / repair source areas | Brand and mode dependent |
| FlashMode IMEI repair | Brand and mode dependent |
| Vivo / Xiaomi / OPPO IMEI repair paths | Brand and mode dependent |
| Infinix / Tecno / Itel IMEI repair paths | Brand and mode dependent |
| Samsung MT67xx IMEI-related key paths | Brand and mode dependent |
| Hardware key read paths | Device dependent |
| DXCC helper integration | Device dependent |
| SEJ / GCPU helper integration | Device dependent |
| AES helper paths | Included where supported |
| RPMB read | Device dependent |
| RPMB write | Device dependent |
| RPMB erase | Device dependent |
| RPMB key read | Device dependent |

## Security and Research-Oriented Paths

| Feature | Status |
| --- | --- |
| SLA / DAA handling | Device dependent |
| Brand and model SLA key handling | Device dependent |
| Kamakiri workflow | Device dependent |
| HeapBait workflow | Device dependent |
| Carbonara workflow | Device dependent |
| Infinix DA + exploit paths for selected MT6789, MT6835, MT6855, MT6878, MT6886, MT6893, MT6895, MT6896, MT6897, MT6983, MT6985, MT6989 and MT6993 targets | Device dependent |
| FRP / format workflow code paths | Device dependent |
| Vbmeta / dm-verity patch paths | Device dependent |
| Brand-specific patch examples | Package dependent |

## Patch and Advanced Service Workflows

All workflows are intended for lawful development, authorized device maintenance, internal service tooling and research by qualified teams.

| Feature | Notes |
| --- | --- |
| Patch cert | Selected OPPO / Xiaomi workflows where supported. |
| Disable Mi Account | System, APK and hosts based paths where supported. |
| Mi Account + Global | Selected `cust` / `opcust` paths where supported. |
| Mi Account Remove + Convert Global V2 | Device and firmware dependent. |
| Reset Huawei ID | Device and firmware dependent. |
| Reset OPPO ID | Device and firmware dependent. |
| Remove security plugin | Device and firmware dependent. |
| OTA Remove | Selected OPPO / Realme / OnePlus workflows where supported. |
| Xiaomi OTA Remove | Device and firmware dependent. |
| MDM + OTA Remove | Device and firmware dependent. |
| MDM Remove | Device and firmware dependent. |
| Walton MDM / Walock Remove | Selected Walton workflows where supported. |
| Nothing / CMF SIM Lock / MDM Remove | Selected Nothing / CMF workflows where supported. |
| Payjoy Remove | Device and firmware dependent. |
| Payjoy Remove through `oeminfo` | Selected paths where supported. |
| IT Admin / Network / Payjoy Unlock | Device and firmware dependent. |
| Modem Patch | Xiaomi anti-relock paths where supported. |
| Persist Patch | Device and firmware dependent. |
| Demo Remove | Device and firmware dependent. |
| Restore Vbmeta | Security ON paths where supported. |
| Vbmeta Patch V1 / V2 / V3 | Security OFF paths where supported. |
| Fix no efuse state | Device and firmware dependent. |
| Fix DM corrupted | Device and firmware dependent. |
| Fix read / orange state | Device dependent after bootloader unlock. |

## Example Brand Workflow Areas

| Brand / Area | Notes |
| --- | --- |
| Samsung MTK | Selected service, NV, partition and authorization paths depending on model and package scope. |
| Motorola MTK | MTKV5 and MTKV6 workflow examples for supported chipsets and models. |
| Infinix MTK | MTKV5 and MTKV6 workflow examples where supported. |
| Honor / Vivo | Selected modern MTK workflow examples where supported. |
| Xiaomi / Redmi | Selected flash, partition, file-system and information read paths where supported. |
| OPPO / Realme / OnePlus | Selected patch, OTA, account and service workflows where supported. |
| Tecno / Itel | Selected IMEI, service and partition workflows where supported. |
| Huawei | Selected account and partition workflows where supported. |
| TCL / Alcatel | Selected model-specific support paths where included. |
| Walton | Selected MDM / Walock workflows where supported. |
| Nothing / CMF | Selected SIM Lock / MDM workflows where supported. |

## Storage Targets

| Storage | Notes |
| --- | --- |
| EMMC | Supported where protocol and DA allow it. |
| UFS | Supported where protocol and DA allow it. |
| NAND | Supported where protocol and DA allow it. |
| NOR / SDMMC | Supported in selected paths where available. |
