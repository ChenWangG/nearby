//
//  ViewController.swift
//  PresenceTest
//
//  Created by Chen Wang on 12/3/24.
//

import UIKit
import SwiftFFI
import os.log

class ViewController: UIViewController {
    var bleScanner: SwiftBleScanner?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        bleScanner = SwiftBleScanner()
    }
}

