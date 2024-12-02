//
//  ViewController.swift
//  TestFFI
//
//  Created by Chen Wang on 12/1/24.
//

import UIKit
import SwiftFFI
import os.log

class ViewController: UIViewController {

    override func viewDidLoad() {
        print("Start App.")
        os_log("Start test Swift FFI.")
        super.viewDidLoad()
        let rust_object = rust_object_new { input -> () in
            os_log("Swift called from Rust: %{public}d", input)
        }
        rust_object_call_swift(rust_object)
    }


}

