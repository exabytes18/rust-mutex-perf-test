# Just trying to get an idea of how many times we can lock/unlock a mutex per second in rust

## OSX (BEWARE: fuzzy results due to non-rigorous testing methodology... i.e. god knows what else the cores were doing or what frequency they were running at)

### MacBook Pro

Spec:
* 2.4GHz 8-Core Intel Core i9 (i9-9980HK)
* 32gb RAM 2667 MHz DDR4

```text
exabytes18@xenon mutex-perf-test % cargo run --release
   Compiling mutex-perf-test v0.1.0 (/Users/exabytes18/Documents/development/mutex-perf-test)
    Finished release [optimized] target(s) in 1.13s
     Running `target/release/mutex-perf-test`
Total time: 4.8947911s
      Rate: 204299/second
Total time: 4.836811474s
      Rate: 206748/second
Total time: 4.77376198s
      Rate: 209478/second
Total time: 5.013367109s
      Rate: 199467/second
Total time: 4.996008548s
      Rate: 200160/second
Total time: 5.008518861s
      Rate: 199660/second
Total time: 4.883726794s
      Rate: 204762/second
Total time: 5.029323952s
      Rate: 198834/second
Total time: 5.069117785s
      Rate: 197273/second
Total time: 5.001770576s
      Rate: 199929/second
```

TL;DR: Two threads can ping-pong back and forth at about 200,000 round-trips per second.

## Linux (slightly more reliable... though still didn't do anything fancy like playing with CPU governor, C States, etc)

### c6g.xlarge

Specs:
* 4 vCPUs, Graviton2
* 8gb RAM

```text
root@ip-10-100-0-90:/home/ubuntu/mutex-perf-test# lscpu 
Architecture:                    aarch64
CPU op-mode(s):                  32-bit, 64-bit
Byte Order:                      Little Endian
CPU(s):                          4
On-line CPU(s) list:             0-3
Thread(s) per core:              1
Core(s) per socket:              4
Socket(s):                       1
NUMA node(s):                    1
Vendor ID:                       ARM
Model:                           1
Model name:                      Neoverse-N1
Stepping:                        r3p1
BogoMIPS:                        243.75
L1d cache:                       256 KiB
L1i cache:                       256 KiB
L2 cache:                        4 MiB
L3 cache:                        32 MiB
NUMA node0 CPU(s):               0-3
Vulnerability Itlb multihit:     Not affected
Vulnerability L1tf:              Not affected
Vulnerability Mds:               Not affected
Vulnerability Meltdown:          Not affected
Vulnerability Spec store bypass: Mitigation; Speculative Store Bypass disabled via prctl
Vulnerability Spectre v1:        Mitigation; __user pointer sanitization
Vulnerability Spectre v2:        Not affected
Vulnerability Srbds:             Not affected
Vulnerability Tsx async abort:   Not affected
Flags:                           fp asimd evtstrm aes pmull sha1 sha2 crc32 atomics fphp asimdhp cpuid asimdrdm lrcpc dcpop asimddp ssbs
```

```text
root@ip-10-100-0-90:/home/ubuntu/mutex-perf-test# cargo run --release
   Compiling mutex-perf-test v0.1.0 (/home/ubuntu/mutex-perf-test)
    Finished release [optimized] target(s) in 0.48s
     Running `target/release/mutex-perf-test`
Total time: 9.475089936s
      Rate: 105540/second
Total time: 9.263987831s
      Rate: 107945/second
Total time: 9.334516666s
      Rate: 107129/second
Total time: 9.06804243s
      Rate: 110277/second
Total time: 9.155219386s
      Rate: 109227/second
Total time: 9.36409894s
      Rate: 106791/second
Total time: 9.263675715s
      Rate: 107949/second
Total time: 9.386189753s
      Rate: 106540/second
Total time: 9.34592606s
      Rate: 106998/second
Total time: 9.464450294s
      Rate: 105659/second
```

TL;DR: Two threads can ping-pong back and forth at about 105,000 round-trips per second.

### c5.xlarge

Specs:
* 4 vCPUs (2 physical cores), Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz
* 8gb RAM

```text
root@ip-10-100-0-188:/home/ubuntu/mutex-perf-test# lscpu 
Architecture:                    x86_64
CPU op-mode(s):                  32-bit, 64-bit
Byte Order:                      Little Endian
Address sizes:                   46 bits physical, 48 bits virtual
CPU(s):                          4
On-line CPU(s) list:             0-3
Thread(s) per core:              2
Core(s) per socket:              2
Socket(s):                       1
NUMA node(s):                    1
Vendor ID:                       GenuineIntel
CPU family:                      6
Model:                           85
Model name:                      Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz
Stepping:                        7
CPU MHz:                         3608.755
BogoMIPS:                        5999.99
Hypervisor vendor:               KVM
Virtualization type:             full
L1d cache:                       64 KiB
L1i cache:                       64 KiB
L2 cache:                        2 MiB
L3 cache:                        35.8 MiB
NUMA node0 CPU(s):               0-3
Vulnerability Itlb multihit:     KVM: Vulnerable
Vulnerability L1tf:              Mitigation; PTE Inversion
Vulnerability Mds:               Vulnerable: Clear CPU buffers attempted, no microcode; SMT Host state unknown
Vulnerability Meltdown:          Mitigation; PTI
Vulnerability Spec store bypass: Vulnerable
Vulnerability Spectre v1:        Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:        Mitigation; Full generic retpoline, STIBP disabled, RSB filling
Vulnerability Srbds:             Not affected
Vulnerability Tsx async abort:   Not affected
Flags:                           fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm
                                  constant_tsc rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 x2
                                 apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch invpcid_single pti fsgsbase tsc_
                                 adjust bmi1 avx2 smep bmi2 erms invpcid mpx avx512f avx512dq rdseed adx smap clflushopt clwb avx512cd avx512bw avx512vl xsaveopt xsa
                                 vec xgetbv1 xsaves ida arat pku ospke
```

```text
root@ip-10-100-0-188:/home/ubuntu/mutex-perf-test# cargo run --release
   Compiling mutex-perf-test v0.1.0 (/home/ubuntu/mutex-perf-test)
    Finished release [optimized] target(s) in 0.54s
     Running `target/release/mutex-perf-test`
Total time: 14.129792762s
      Rate: 70772/second
Total time: 14.390537532s
      Rate: 69490/second
Total time: 14.600402602s
      Rate: 68491/second
Total time: 14.790203439s
      Rate: 67612/second
Total time: 14.645409703s
      Rate: 68281/second
Total time: 14.79259643s
      Rate: 67601/second
Total time: 14.689641118s
      Rate: 68075/second
Total time: 14.626459786s
      Rate: 68369/second
Total time: 14.736750734s
      Rate: 67858/second
Total time: 14.747564154s
      Rate: 67808/second
```

TL;DR: Two threads can ping-pong back and forth at about 67,000 round-trips per second.

### z1d.xlarge

Specs:
* 4 vCPUs (2 physical cores), Intel(R) Xeon(R) Platinum 8151 CPU @ 3.40GHz, operating at 4.00GHz
* 8gb RAM

```text
root@ip-10-100-0-21:/home/ubuntu/mutex-perf-test# lscpu 
Architecture:                    x86_64
CPU op-mode(s):                  32-bit, 64-bit
Byte Order:                      Little Endian
Address sizes:                   46 bits physical, 48 bits virtual
CPU(s):                          4
On-line CPU(s) list:             0-3
Thread(s) per core:              2
Core(s) per socket:              2
Socket(s):                       1
NUMA node(s):                    1
Vendor ID:                       GenuineIntel
CPU family:                      6
Model:                           85
Model name:                      Intel(R) Xeon(R) Platinum 8151 CPU @ 3.40GHz
Stepping:                        4
CPU MHz:                         4022.014
BogoMIPS:                        6799.99
Hypervisor vendor:               KVM
Virtualization type:             full
L1d cache:                       64 KiB
L1i cache:                       64 KiB
L2 cache:                        2 MiB
L3 cache:                        24.8 MiB
NUMA node0 CPU(s):               0-3
Vulnerability Itlb multihit:     KVM: Vulnerable
Vulnerability L1tf:              Mitigation; PTE Inversion
Vulnerability Mds:               Vulnerable: Clear CPU buffers attempted, no microcode; SMT Host state unknown
Vulnerability Meltdown:          Mitigation; PTI
Vulnerability Spec store bypass: Vulnerable
Vulnerability Spectre v1:        Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:        Mitigation; Full generic retpoline, STIBP disabled, RSB filling
Vulnerability Srbds:             Not affected
Vulnerability Tsx async abort:   Vulnerable: Clear CPU buffers attempted, no microcode; SMT Host state unknown
Flags:                           fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm
                                  constant_tsc rep_good nopl xtopology nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 x2
                                 apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch invpcid_single pti fsgsbase tsc_
                                 adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm mpx avx512f avx512dq rdseed adx smap clflushopt clwb avx512cd avx512bw avx512vl xsav
                                 eopt xsavec xgetbv1 xsaves ida arat pku ospke
```

```text
root@ip-10-100-0-21:/home/ubuntu/mutex-perf-test# cargo run --release
   Compiling mutex-perf-test v0.1.0 (/home/ubuntu/mutex-perf-test)
    Finished release [optimized] target(s) in 0.48s
     Running `target/release/mutex-perf-test`
Total time: 13.659339676s
      Rate: 73210/second
Total time: 13.701596935s
      Rate: 72984/second
Total time: 13.657770548s
      Rate: 73218/second
Total time: 13.525946205s
      Rate: 73932/second
Total time: 13.661367545s
      Rate: 73199/second
Total time: 13.527282996s
      Rate: 73925/second
Total time: 13.546404975s
      Rate: 73820/second
Total time: 14.021727142s
      Rate: 71318/second
Total time: 13.754902598s
      Rate: 72701/second
Total time: 13.530456731s
      Rate: 73907/second
```

TL;DR: Two threads can ping-pong back and forth at about 73,000 round-trips per second.
