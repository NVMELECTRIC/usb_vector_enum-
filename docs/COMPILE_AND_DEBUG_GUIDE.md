# Compile and Debug Guide

This public repository does not contain the private SharpMTKClient source code package. The notes below describe the typical development environment expected by the licensed C# project.

## Typical Environment

* Windows 10 or Windows 11.
* Visual Studio with .NET desktop development workload.
* C# / .NET Framework or .NET runtime version required by the licensed package.
* Administrator access for selected USB, driver and device workflows.
* Required USB drivers for target MediaTek device modes.

Exact requirements may vary by licensed package version.

## Project Opening

Licensed buyers normally receive a Visual Studio solution or project package.

Recommended steps:

1. Extract the licensed package to a private local directory.
2. Open the solution in Visual Studio.
3. Restore NuGet packages if the package uses them.
4. Confirm build configuration and platform target.
5. Build the solution.
6. Run with the required permissions for the selected workflow.

## Debugging Device Workflows

For protocol and flash workflows, collect:

* Device model and chipset.
* Boot mode used.
* Hardware code and hardware sub-code where available.
* Storage type.
* Selected DA or firmware package.
* Full application log.
* Operation target and failure point.

## USB and Driver Notes

SharpMTKClient is designed around Windows-based MTK workflows. Some operations may use native Windows API communication paths, while selected authorization or research paths may require additional USB handling components depending on package scope.

Driver state, port mode and device timing can affect protocol behavior. Always reproduce with a known-good cable, port and driver setup before reporting protocol issues.

## Build Problems

If a licensed package does not build:

* Confirm the exact Visual Studio version.
* Confirm the target .NET version.
* Restore required NuGet packages.
* Check missing private assets or excluded licensed files.
* Send the full build output privately to support.

## Support

Support terms are described in [SUPPORT_POLICY.md](SUPPORT_POLICY.md).
