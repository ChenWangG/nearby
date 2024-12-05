//
//  ViewController.swift
//  PresenceTest
//
//  Created by Chen Wang on 12/3/24.
//

import UIKit
import os.log
import PresenceFFI

class ViewController: UIViewController {
    var bleScanner: SwiftBleScanner?
    private var rustPresence: OpaquePointer?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        print("Start Presence test.")
        rustPresence = presence_new()
        
        let iosPresence = IosPresence()
        let rawPtr = iosPresence.ptrToSelf()
        let ptr: Unmanaged<IosPresence> =  Unmanaged.fromOpaque(rawPtr)
        ptr.takeUnretainedValue().print()
    }
}

