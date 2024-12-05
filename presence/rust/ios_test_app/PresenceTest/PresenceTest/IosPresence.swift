import os.log
import PresenceFFI

class IosPresence  {
    var rustPresence: OpaquePointer?
    init() {
        os_log("Init iOS Presence.")
        let selfPtr = ptrToSelf()
        rustPresence = presence_new(selfPtr, { ptr -> () in
            os_log("Rust called Swift.")
            if ptr != nil {
                let presencePtr: Unmanaged<IosPresence> =  Unmanaged.fromOpaque(ptr!)
                presencePtr.takeUnretainedValue().print()            }
            
        })
    }
    
    func ptrToSelf() -> UnsafeMutableRawPointer {
        return Unmanaged.passUnretained(self).toOpaque()
    }
    
    func print() {
        os_log("print");
    }
}

