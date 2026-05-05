```markdown
    ## 🛠 Hardware Implementation Notes
    ### AVF / pKVM Hypervisor Support
    During Phase 1.6 testing on Samsung Galaxy Fold 6 (SM-F956W), it was confirmed that Samsung Knox / RKP (Real-time Kernel Protection) restricts third-party access to the `virtualmachine` system service. 
    
    **Architectural Pivot:** S.H.I.F.T now utilizes a **Universal Zero-Trust Enclave** powered by SELinux `isolatedProcess` flags. This achieves the same security guarantees (mathematical isolation from Network and Disk) while ensuring compatibility across all flagship Android hardware.
```
---
