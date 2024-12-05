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
    private var iosPresence: IosPresence?
    
    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        print("Start Presence test.")
        iosPresence = IosPresence()
        DispatchQueue.global(qos: .userInitiated).async {
            os_log("start run on backkground queue")
            self.iosPresence?.start()

            DispatchQueue.main.async {
                os_log("start run on main queue")
            }
        }
        os_log("Continue after starting background queue")
        self.iosPresence?.set_request()
    }
}

