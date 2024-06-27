fn main() {
    #[cfg(feature = "dynamic-server")]
    {
        eprintln!(
            "This binary target does not accept the `dynamic-server` feature. Use the \
            `dynamic-server` binary target instead"
        );
        exit(1);
    }

    #[cfg(not(feature = "dynamic-server"))]
    grola::make_parsers();
}
