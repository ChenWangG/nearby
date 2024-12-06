import os.log
import PresenceFFI
import UIKit

class IosPresence  {
    var rustPresence: OpaquePointer?
    var bleScanner: SwiftBleScanner?
    var view: UIView!
    
    init(view: UIView) {
        os_log("Init iOS Presence.")
        self.view = view
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
            presence.bleScanner!.setRustBleScanCallback(callback: rust_callback!)
            
        })
    }
    
    func start() {
        os_log("Start Presence runtime.");
        presence_start(rustPresence!) { ptr, action -> () in
            os_log("On Discovery: %{public}d", action)
            let presencePtr: Unmanaged<IosPresence> =  Unmanaged.fromOpaque(ptr!)
            let presence = presencePtr.takeUnretainedValue()
            presence.print(action: action)
        }
    }
    
    func set_request() {
        os_log("Set Presence request.");
        presence_set_request(rustPresence!)
    }
    
    func print(action: UInt32) {
        os_log("print action: %{public}d", action);
        DispatchQueue.main.async {
            os_log("access main thread.")
            if action == 5 {
                self.view.isHidden = true
            } else {
                self.view.isHidden = false
            }
        }
    }
}

