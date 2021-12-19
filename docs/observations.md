# Empirical Observations

Chaoscope gives insight about the `pallet-chaos` extrinsics effects on Runtimes that include it.

Below are some empirical observations collected from a `substrate-node-template` with `pallet-chaos` added to it. The runtime was executed in Native mode.

For each extrinsic provided by `pallet-chaos`, a table is displayed with the relevant data.

## dragBlockUnitWeight(n)

Drags block production by calculating hashes in a loop (`n` times), with constant unitary extrinsic weight.

|       n       | block execution time (ms) | fees charges |
|:-------------:|:----------------:|:------:|
|   1_000_000   |         6_000        | 125_000_114 |
|   5_000_000   |         20_001        | 125_000_114 |
|   10_000_000  |         32_000        | 125_000_114 |
|   50_000_000  |         86_001        | 125_000_114 |
|  100_000_000  |         346_000        | 125_000_114 |

# Machine Specs
The machine was a Google Cloud `n2-standard-4` with the following specs:

## Software
- OS:
```
$ lsb_release -a
No LSB modules are available.
Distributor ID:	Ubuntu
Description:	Ubuntu 20.04.3 LTS
Release:	20.04
Codename:	focal
```
- Rust:
```
$ cargo --version
cargo 1.57.0 (b2e52d7ca 2021-10-21)
$ rustc --version
rustc --version
```

## Hardware
- CPU:
```
$ lscpu
Architecture:                    x86_64
CPU op-mode(s):                  32-bit, 64-bit
Byte Order:                      Little Endian
Address sizes:                   46 bits physical, 48 bits v
                                 irtual
CPU(s):                          4
On-line CPU(s) list:             0-3
Thread(s) per core:              2
Core(s) per socket:              2
Socket(s):                       1
NUMA node(s):                    1
Vendor ID:                       GenuineIntel
CPU family:                      6
Model:                           85
Model name:                      Intel(R) Xeon(R) CPU @ 2.80
                                 GHz
Stepping:                        7
CPU MHz:                         2800.266
BogoMIPS:                        5600.53
Hypervisor vendor:               KVM
Virtualization type:             full
L1d cache:                       64 KiB
L1i cache:                       64 KiB
L2 cache:                        2 MiB
L3 cache:                        33 MiB
NUMA node0 CPU(s):               0-3
Vulnerability Itlb multihit:     Not affected
Vulnerability L1tf:              Not affected
Vulnerability Mds:               Mitigation; Clear CPU buffe
                                 rs; SMT Host state unknown
Vulnerability Meltdown:          Not affected
Vulnerability Spec store bypass: Mitigation; Speculative Sto
                                 re Bypass disabled via prct
                                 l and seccomp
Vulnerability Spectre v1:        Mitigation; usercopy/swapgs
                                  barriers and __user pointe
                                 r sanitization
Vulnerability Spectre v2:        Mitigation; Enhanced IBRS,
                                 IBPB conditional, RSB filli
                                 ng
Vulnerability Srbds:             Not affected
Vulnerability Tsx async abort:   Mitigation; Clear CPU buffe
                                 rs; SMT Host state unknown
```
- RAM:
```
$ lshw -C memory
firmware                
     description: BIOS
     vendor: Google
     physical id: 0
     version: Google
     date: 01/01/2011
     size: 96KiB
memory
     description: System Memory
     physical id: 200
     size: 16GiB
     capabilities: ecc
     configuration: errordetection=multi-bit-ecc
   bank
        description: DIMM RAM Synchronous
        physical id: 0
        slot: DIMM 0
        size: 16GiB
        width: 64 bits
```
- Disk:
```
$ fio --randrepeat=1 --ioengine=posixaio --direct=1 --gtod_reduce=1 --name=test --filename=test --bs=4k --iodepth=64 --size=4G --readwrite=randrw --rwmixread=75
test: (groupid=0, jobs=1): err= 0: pid=14828: Thu Dec  9 00:08:55 2021
  read: IOPS=1524, BW=6097KiB/s (6244kB/s)(3070MiB/515580msec)
   bw (  KiB/s): min= 1880, max= 7024, per=100.00%, avg=6097.02, stdev=370.71, samples=1031
   iops        : min=  470, max= 1756, avg=1524.25, stdev=92.68, samples=1031
  write: IOPS=509, BW=2038KiB/s (2087kB/s)(1026MiB/515580msec); 0 zone resets
   bw (  KiB/s): min=  664, max= 2568, per=100.00%, avg=2037.72, stdev=165.13, samples=1031
   iops        : min=  166, max=  642, avg=509.43, stdev=41.28, samples=1031
  cpu          : usr=0.78%, sys=0.23%, ctx=131133, majf=0, minf=40
  IO depths    : 1=0.1%, 2=0.1%, 4=0.1%, 8=12.5%, 16=25.0%, 32=50.0%, >=64=12.5%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=98.6%, 8=0.1%, 16=0.0%, 32=0.0%, 64=1.4%, >=64=0.0%
     issued rwts: total=785920,262656,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=64
Run status group 0 (all jobs):
   READ: bw=6097KiB/s (6244kB/s), 6097KiB/s-6097KiB/s (6244kB/s-6244kB/s), io=3070MiB (3219MB), run=515580-515580msec
  WRITE: bw=2038KiB/s (2087kB/s), 2038KiB/s-2038KiB/s (2087kB/s-2087kB/s), io=1026MiB (1076MB), run=515580-515580msec
Disk stats (read/write):
  sda: ios=785753/263165, merge=0/675, ticks=363111/132665, in_queue=495788, util=100.00%
```
- PCI Drivers:
```
$ lspci
00:00.0 Host bridge: Intel Corporation 440FX - 82441FX PMC [Natoma] (rev 02)
00:01.0 ISA bridge: Intel Corporation 82371AB/EB/MB PIIX4 ISA (rev 03)
00:01.3 Bridge: Intel Corporation 82371AB/EB/MB PIIX4 ACPI (rev 03)
00:03.0 Non-VGA unclassified device: Red Hat, Inc. Virtio SCSI
00:04.0 Ethernet controller: Red Hat, Inc. Virtio network device
00:05.0 Unclassified device [00ff]: Red Hat, Inc. Virtio RNG
```
- Other Hardware:
```
$ lshw
bernardo-dev                
    description: Computer
    product: Google Compute Engine
    vendor: Google
    serial: GoogleCloud-0094C8FDB10E9B935E08F12B5059DC0A
    width: 64 bits
    capabilities: smbios-2.4 dmi-2.4 smp vsyscall32
    configuration: boot=normal uuid=0094C8FD-B10E-9B93-5E08-F12B5059DC0A
  core
       description: Motherboard
       product: Google Compute Engine
       vendor: Google
       physical id: 0
       serial: Board-GoogleCloud-0094C8FDB10E9B935E08F12B5059DC0A
     firmware
          description: BIOS
          vendor: Google
          physical id: 0
          version: Google
          date: 01/01/2011
          size: 96KiB
     cpu:0
          description: CPU
          product: Intel(R) Xeon(R) CPU @ 2.80GHz
          vendor: Intel Corp.
          physical id: 1001
          bus info: cpu@0
          slot: CPU 1
          size: 2GHz
          capacity: 2GHz
          width: 64 bits
          capabilities: fpu fpu_exception wp vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp x86-64 constant_tsc rep_good nopl xtopology nonstop_tsc cpuid tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 x2apic movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch invpcid_single ssbd ibrs ibpb stibp ibrs_enhanced fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx avx512f avx512dq rdseed adx smap clflushopt clwb avx512cd avx512bw avx512vl xsaveopt xsavec xgetbv1 xsaves arat avx512_vnni md_clear arch_capabilities
     cpu:1
          description: CPU
          vendor: Google
          physical id: 1002
          bus info: cpu@1
          slot: CPU 2
          size: 2GHz
          capacity: 2GHz
     cpu:2
          description: CPU
          vendor: Google
          physical id: 1003
          bus info: cpu@2
          slot: CPU 3
          size: 2GHz
          capacity: 2GHz
     cpu:3
          description: CPU
          vendor: Google
          physical id: 1004
          bus info: cpu@3
          slot: CPU 4
          size: 2GHz
          capacity: 2GHz
     memory
          description: System Memory
          physical id: 200
          size: 16GiB
          capabilities: ecc
          configuration: errordetection=multi-bit-ecc
        bank
             description: DIMM RAM Synchronous
             physical id: 0
             slot: DIMM 0
             size: 16GiB
             width: 64 bits
     pci
          description: Host bridge
          product: 440FX - 82441FX PMC [Natoma]
          vendor: Intel Corporation
          physical id: 100
          bus info: pci@0000:00:00.0
          version: 02
          width: 32 bits
          clock: 33MHz
        isa
             description: ISA bridge
             product: 82371AB/EB/MB PIIX4 ISA
             vendor: Intel Corporation
             physical id: 1
             bus info: pci@0000:00:01.0
             version: 03
             width: 32 bits
             clock: 33MHz
             capabilities: isa bus_master
             configuration: latency=0
        bridge UNCLAIMED
             description: Bridge
             product: 82371AB/EB/MB PIIX4 ACPI
             vendor: Intel Corporation
             physical id: 1.3
             bus info: pci@0000:00:01.3
             version: 03
             width: 32 bits
             clock: 33MHz
             capabilities: bridge bus_master
             configuration: latency=0
        generic:0
             description: Non-VGA unclassified device
             product: Virtio SCSI
             vendor: Red Hat, Inc.
             physical id: 3
             bus info: pci@0000:00:03.0
             version: 00
             width: 32 bits
             clock: 33MHz
             capabilities: msix bus_master cap_list
             configuration: driver=virtio-pci latency=0
             resources: irq:11 ioport:c040(size=64) memory:c0001000-c000107f
           virtio0 UNCLAIMED
                description: Virtual I/O device
                physical id: 0
                bus info: virtio@0
                configuration: driver=virtio_scsi
        network
             description: Ethernet controller
             product: Virtio network device
             vendor: Red Hat, Inc.
             physical id: 4
             bus info: pci@0000:00:04.0
             version: 00
             width: 32 bits
             clock: 33MHz
             capabilities: msix bus_master cap_list
             configuration: driver=virtio-pci latency=0
             resources: irq:10 ioport:c000(size=64) memory:c0000000-c00000ff
           virtio1
                description: Ethernet interface
                physical id: 0
                bus info: virtio@1
                logical name: ens4
                serial: 42:01:0a:84:00:28
                capabilities: ethernet physical
                configuration: autonegotiation=off broadcast=yes driver=virtio_net driverversion=1.0.0 ip=10.132.0.40 link=yes multicast=yes
        generic:1
             description: Unclassified device
             product: Virtio RNG
             vendor: Red Hat, Inc.
             physical id: 5
             bus info: pci@0000:00:05.0
             version: 00
             width: 32 bits
             clock: 33MHz
             capabilities: msix bus_master cap_list
             configuration: driver=virtio-pci latency=0
             resources: irq:10 ioport:c080(size=32) memory:c0002000-c000203f
           virtio2 UNCLAIMED
                description: Virtual I/O device
                physical id: 0
                bus info: virtio@2
                configuration: driver=virtio_rng
     pnp00:00
          product: PnP device PNP0b00
          physical id: 1
          capabilities: pnp
          configuration: driver=rtc_cmos
     pnp00:01
          product: PnP device PNP0303
          physical id: 2
          capabilities: pnp
          configuration: driver=i8042 kbd
     pnp00:02
          product: PnP device PNP0f13
          physical id: 3
          capabilities: pnp
          configuration: driver=i8042 aux
     pnp00:03
          product: PnP device PNP0501
          physical id: 4
          capabilities: pnp
          configuration: driver=serial
     pnp00:04
          product: PnP device PNP0501
          physical id: 5
          capabilities: pnp
          configuration: driver=serial
     pnp00:05
          product: PnP device PNP0501
          physical id: 6
          capabilities: pnp
          configuration: driver=serial
     pnp00:06
          product: PnP device PNP0501
          physical id: 7
          capabilities: pnp
          configuration: driver=serial
     scsi
          physical id: 8
          logical name: scsi0
        disk
             description: SCSI Disk
             product: PersistentDisk
             vendor: Google
             physical id: 0.1.0
             bus info: scsi@0:0.1.0
             logical name: /dev/sda
             version: 1
             size: 100GiB (107GB)
             capabilities: gpt-1.00 partitioned partitioned:gpt
             configuration: ansiversion=6 guid=c1264fea-f689-45ca-8d50-7d4f0901617a logicalsectorsize=512 sectorsize=4096
           volume:0
                description: EXT4 volume
                vendor: Linux
                physical id: 1
                bus info: scsi@0:0.1.0,1
                logical name: /dev/sda1
                logical name: /
                version: 1.0
                serial: dec83c81-6fb6-4b33-8fd0-0d7912f69e91
                size: 2140MiB
                capacity: 99GiB
                capabilities: journaled extended_attributes large_files huge_files dir_nlink recover 64bit extents ext4 ext2 initialized
                configuration: created=2021-12-03 01:48:39 filesystem=ext4 label=cloudimg-rootfs lastmountpoint=/build/mountpoint_gce modified=2021-12-03 01:51:34 mount.fstype=ext4 mount.options=rw,relatime mounted=2021-12-08 23:53:41 state=mounted
           volume:1
                description: BIOS Boot partition
                vendor: EFI
                physical id: e
                bus info: scsi@0:0.1.0,14
                logical name: /dev/sda14
                serial: 2d60d1ae-f1c2-45e2-997d-2de2e18764b5
                capacity: 4095KiB
                capabilities: nofs
           volume:2
                description: Windows FAT volume
                vendor: mkfs.fat
                physical id: f
                bus info: scsi@0:0.1.0,15
                logical name: /dev/sda15
                logical name: /boot/efi
                version: FAT32
                serial: 78ca-0965
                size: 105MiB
                capacity: 105MiB
                capabilities: boot fat initialized
                configuration: FATs=2 filesystem=fat label=UEFI mount.fstype=vfat mount.options=rw,relatime,fmask=0077,dmask=0077,codepage=437,iocharset=iso8859-1,shortname=mixed,errors=remount-ro state=mounted
```
