#import <UIKit/UIKit.h>
#import <WebKit/WebKit.h>
#import <objc/runtime.h>

// Disable WKWebView scrollView contentInsetAdjustmentBehavior
// This allows viewport-fit=cover to work correctly on iOS

static IMP original_didMoveToWindow = NULL;

static void swizzled_didMoveToWindow(id self, SEL _cmd) {
    if (original_didMoveToWindow) {
        ((void (*)(id, SEL))original_didMoveToWindow)(self, _cmd);
    }

    if ([self isKindOfClass:[WKWebView class]]) {
        WKWebView *webView = (WKWebView *)self;
        webView.scrollView.contentInsetAdjustmentBehavior = UIScrollViewContentInsetAdjustmentNever;
    }
}

// No __attribute__((constructor)): called explicitly once from
// src/main.rs. Keeping the constructor attribute here in addition to that
// explicit call double-runs the swizzle (constructor fires at load, then
// the Rust call fires again), and the second pass captures the
// already-swizzled implementation as "original" — infinite recursion the
// next time didMoveToWindow fires (EXC_BAD_ACCESS / stack overflow).
void DisableContentInsetAdjustment(void) {
    Class WKWebViewClass = [WKWebView class];
    if (!WKWebViewClass) {
        NSLog(@"DisableContentInsetAdjustment: WKWebView class not found");
        return;
    }

    Method originalMethod = class_getInstanceMethod(WKWebViewClass, @selector(didMoveToWindow));
    if (!originalMethod) {
        NSLog(@"DisableContentInsetAdjustment: didMoveToWindow method not found");
        return;
    }

    original_didMoveToWindow = method_getImplementation(originalMethod);
    method_setImplementation(originalMethod, (IMP)swizzled_didMoveToWindow);
    NSLog(@"DisableContentInsetAdjustment: Successfully configured");
}
