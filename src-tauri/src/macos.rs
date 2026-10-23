use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObjectProtocol};
use objc2::{msg_send, sel, MainThreadMarker};
use objc2_app_kit::NSWritingToolsBehavior;
use objc2_foundation::{
	ns_string, NSDictionary, NSNumber, NSString, NSUserDefaults,
};
use objc2_web_kit::WKWebViewConfiguration;

pub fn disable_text_assistance() {
	let keys = [
		ns_string!("WebAutomaticQuoteSubstitutionEnabled"),
		ns_string!("WebAutomaticDashSubstitutionEnabled"),
		ns_string!("WebAutomaticTextReplacementEnabled"),
		ns_string!("WebAutomaticSpellingCorrectionEnabled"),
		ns_string!("WebGrammarCheckingEnabled"),
		ns_string!("WebSmartInsertDeleteEnabled"),
		ns_string!("NSAutomaticInlinePredictionEnabled"),
		ns_string!("NSAutomaticTextCompletionEnabled"),
		ns_string!("NSAutoFillSystemInsertMenuEnabled"),
	];
	let off: &AnyObject = &NSNumber::new_bool(false);
	let defaults = NSDictionary::<NSString, AnyObject>::from_slices(
		&keys,
		&keys.map(|_| off),
	);
	unsafe {
		NSUserDefaults::standardUserDefaults().registerDefaults(&defaults)
	};
}

pub fn webview_configuration() -> Retained<WKWebViewConfiguration> {
	let mtm = MainThreadMarker::new().expect("must run on the main thread");
	let config = unsafe { WKWebViewConfiguration::new(mtm) };
	if config.respondsToSelector(sel!(setWritingToolsBehavior:)) {
		let _: () = unsafe {
			msg_send![
				&config,
				setWritingToolsBehavior: NSWritingToolsBehavior::None
			]
		};
	}
	config
}
