fn main() {
    #[cfg(any(feature = "dynamic-server", feature = "static-render"))]
    {
        eprintln!(
            "This binary target does not accept the `dynamic-server` or `static-render` features. \
            Use the `dynamic-server` or `static-render` binary targets instead."
        );
        exit(1);
    }

    #[cfg(all(not(feature = "dynamic-server"), not(feature = "static-render")))]
    grola::make_parsers();
}
