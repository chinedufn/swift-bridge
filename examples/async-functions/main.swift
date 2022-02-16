import Foundation

// We use a DispatchGroup to make the program wait until
// our async function finishes before exiting.
// You wouldn't need to do this in a long-running applications.
let group = DispatchGroup()
group.enter()

Task {
    let ipAddress = await get_my_ip_from_rust()
    print("IP address: \(ipAddress.origin().toString()")

    group.leave()
}

group.wait()
