#import <UIKit/UIKit.h>
#import <WebKit/WebKit.h>
#import <objc/runtime.h>

// Hide keyboard accessory bar in WKWebView
// Based on cordova-plugin-ionic-keyboard approach

__attribute__((constructor))
static void HideKeyboardAccessoryBar(void) {
    // Build class name dynamically (WKContentView)
    NSString *WKClassString = [@[@"WK", @"Content", @"View"] componentsJoinedByString:@""];

    Class WKClass = NSClassFromString(WKClassString);
    if (!WKClass) {
        NSLog(@"HideKeyboardAccessory: WKContentView class not found");
        return;
    }

    Method WKMethod = class_getInstanceMethod(WKClass, @selector(inputAccessoryView));
    if (!WKMethod) {
        NSLog(@"HideKeyboardAccessory: inputAccessoryView method not found");
        return;
    }

    // Replace implementation with block that returns nil
    IMP newImp = imp_implementationWithBlock(^(id _self) {
        return nil;
    });

    method_setImplementation(WKMethod, newImp);
    NSLog(@"HideKeyboardAccessory: Successfully hidden");
}
