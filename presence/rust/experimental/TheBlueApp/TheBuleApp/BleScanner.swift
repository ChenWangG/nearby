import Foundation
import CoreBluetooth
import os.log

class BleScanner : NSObject {

    static let serviceUUID = CBUUID(string: "FF01")
    static let log = OSLog(subsystem: Bundle.main.bundleIdentifier!, category: "Scanner")

    private var centralManager: CBCentralManager!
    private var scanningTimer: Timer?

    override init() {
        print("Init BLE Scanner.")
        super.init()
        centralManager = CBCentralManager(delegate: self, queue: nil)
    }

    func startScanning() {
        scanningTimer = Timer.scheduledTimer(withTimeInterval: TimeInterval(20), repeats: false, block: { (_) in
            self.stopScanning()
        })
        centralManager.scanForPeripherals(withServices: [ BleScanner.serviceUUID ], options: nil)
        os_log("BLE scan started", log: BleScanner.log, type: .info)
    }

    func stopScanning() {
        centralManager.stopScan()
        os_log("BLE scan stopped", log: BleScanner.log, type: .info)
        
    }
}

extension BleScanner : CBCentralManagerDelegate {

    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        if centralManager.state == .poweredOn {
            startScanning()
        } else {
            os_log("BLE powered off, failed to start BLE scan", log: BleScanner.log, type: .error)
        }
    }

    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String : Any], rssi RSSI: NSNumber) {
        for (key, value) in advertisementData {
            if key == CBAdvertisementDataServiceDataKey {
                let serviceData = value as! [CBUUID : NSData]
                for (uuid, data) in serviceData {
                    os_log("Advertisement data: %{public}s: %{public}s", log: BleScanner.log, type: .info, uuid.uuidString, data.debugDescription)
                }
            }
        }
    }
}


