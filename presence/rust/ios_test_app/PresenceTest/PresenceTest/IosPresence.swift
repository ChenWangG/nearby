import os.log
import PresenceFFI

class IosPresence  {
    var rustPresence: OpaquePointer?
    var bleScanner: SwiftBleScanner?
    
    init() {
        os_log("Init iOS Presence.")
        bleScanner = SwiftBleScanner()
        let selfPtr = Unmanaged.passUnretained(self).toOpaque()
        rustPresence = presence_new(selfPtr, { ptr, rust_callback -> () in
            os_log("Rust called Swift.")
            if ptr == nil || rust_callback == nil {
                os_log("Passs empty pointer of Rust Presence or Callback.")
                return
            }
            
            let presencePtr: Unmanaged<IosPresence> =  Unmanaged.fromOpaque(ptr!)
            let presence = presencePtr.takeUnretainedValue()
            presence.print()
            presence.bleScanner!.setRustBleScanCallback(callback: rust_callback!)
            
        })
    }
    
    func print() {
        os_log("print");
    }
}

