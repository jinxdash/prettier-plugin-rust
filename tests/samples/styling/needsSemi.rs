fn f() {
	#[cfg(unix)] { 0 }
	#[cfg(windows)] { 1 }
}