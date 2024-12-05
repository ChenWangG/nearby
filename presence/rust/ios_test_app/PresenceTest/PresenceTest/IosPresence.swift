import os.log

class IosPresence  {
    init() {
        os_log("Init iOS Presence.")
    }
    
    func ptrToSelf() -> UnsafeMutableRawPointer {
        return Unmanaged.passUnretained(self).toOpaque()
    }
    
    func print() {
        os_log("print");
    }
}

