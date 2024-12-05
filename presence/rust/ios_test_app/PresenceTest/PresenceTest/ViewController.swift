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
    var iosPresence: IosPresence?
    private var rustPresence: OpaquePointer?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        print("Start Presence test.")
        iosPresence = IosPresence()
    }
}

