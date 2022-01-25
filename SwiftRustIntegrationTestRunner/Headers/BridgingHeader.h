//
//  BridgingHeader.h
//  SwiftRustIntegrationTestRunner
//
//  Created by Frankie Nwafili on 11/14/21.
//

#ifndef BridgingHeader_h
#define BridgingHeader_h

#include "../Generated/SwiftBridgeCore.h"
#include "../Generated/swift-integration-tests/swift-integration-tests.h"

void async_rust_fn(void* callback_wrapper, void async_rust_fn_callback(void* callback_wrapper, int32_t val));

#endif /* BridgingHeader_h */
