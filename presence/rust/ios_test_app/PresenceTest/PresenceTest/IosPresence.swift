import os.log
import PresenceFFI

class IosPresence  {
    var rustPresence: OpaquePointer?
    var bleScanner: SwiftBleScanner?
    
    init() {
        os_log("Init iOS Presence.")
        bleScanner = SwiftBleScanner()
        let selfPtr = Unmanaged.passUnretained(self).toOpaque()
        rustPresence = presence_new(selfPtr, { ptr -> () in
            os_log("Rust called Swift.")
            if ptr == nil { return }
            
            let presencePtr: Unmanaged<IosPresence> =  Unmanaged.fromOpaque(ptr!)
            let presence = presencePtr.takeUnretainedValue()
            presence.print()
            
        })
    }
    
    func print() {
        os_log("print");
    }
}

