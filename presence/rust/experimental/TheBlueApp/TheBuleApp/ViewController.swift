import UIKit

class ViewController: UIViewController {

    var bleScanner: BleScanner?

    override func viewDidLoad() {
        super.viewDidLoad()
        bleScanner = BleScanner()
    }
}
