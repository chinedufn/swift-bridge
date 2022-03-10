import XCTest
import MySwiftPackage
@testable import swift_package_test_package

final class swift_package_test_packageTests: XCTestCase {
    func testPackageRun() throws {
        XCTAssertEqual("Hello, From Rust!", hello_rust().toString())
    }
}
